use crate::bluetooth::{BluetoothCubeDevice, BluetoothCubeEvent};
use crate::common::{Color, Cube, CubeFace, Move, TimedMove};
use crate::cube3x3x3::{Cube3x3x3, Cube3x3x3Faces};
use anyhow::{anyhow, Result};
use btleplug::api::{Characteristic, Peripheral, WriteType};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use uuid::Uuid;

struct GoCube<P: Peripheral + 'static> {
    device: P,
    state: Arc<Mutex<Cube3x3x3>>,
    battery_percentage: Arc<Mutex<Option<u32>>>,
    synced: Arc<Mutex<bool>>,
    write: Characteristic,
}

impl<P: Peripheral + 'static> GoCube<P> {
    const ROTATE_MESSAGE: u8 = 0x01;
    const STATE_MESSAGE: u8 = 0x02;
    const BATTERY_MESSAGE: u8 = 0x05;

    const REQUEST_BATTERY_MESSAGE: u8 = 0x32;
    const REQUEST_STATE_MESSAGE: u8 = 0x33;
    const RESET_STATE_MESSAGE: u8 = 0x35;
    const DISABLE_ORIENTATION_MESSAGE: u8 = 0x37;

    const CUBE_STATE_TIMEOUT_MS: usize = 2000;

    pub fn new(
        device: P,
        read: Characteristic,
        write: Characteristic,
        move_listener: Box<dyn Fn(BluetoothCubeEvent) + Send + 'static>,
    ) -> Result<Self> {
        let state = Arc::new(Mutex::new(Cube3x3x3::new()));
        let state_set = Arc::new(Mutex::new(false));
        let battery_percentage = Arc::new(Mutex::new(None));
        let synced = Arc::new(Mutex::new(true));

        let state_copy = state.clone();
        let state_set_copy = state_set.clone();
        let battery_percentage_copy = battery_percentage.clone();
        let synced_copy = synced.clone();
        let start_time = Mutex::new(Instant::now());
        let last_move_time = Mutex::new(0);

        device.on_notification(Box::new(move |value| {
            if value.value.len() < 4 {
                *synced_copy.lock().unwrap() = false;
                return;
            }
            if value.value.len() < value.value[1] as usize {
                *synced_copy.lock().unwrap() = false;
                return;
            }
            if value.value[1] < 4 {
                *synced_copy.lock().unwrap() = false;
                return;
            }

            match value.value[2] {
                Self::ROTATE_MESSAGE => {
                    let count = (value.value[1] as usize - 4) / 2;
                    let mut moves = Vec::new();
                    for i in 0..count {
                        let move_idx = value.value[3 + i * 2] as usize;
                        let mv = match move_idx {
                            0 => Move::B,
                            1 => Move::Bp,
                            2 => Move::F,
                            3 => Move::Fp,
                            4 => Move::U,
                            5 => Move::Up,
                            6 => Move::D,
                            7 => Move::Dp,
                            8 => Move::R,
                            9 => Move::Rp,
                            0xa => Move::L,
                            0xb => Move::Lp,
                            _ => {
                                *synced_copy.lock().unwrap() = false;
                                return;
                            }
                        };

                        // Apply move to the cube state.
                        state_copy.lock().unwrap().do_move(mv);

                        moves.push(mv);
                    }

                    // Get time since last move. Keep computation relative to start time so
                    // that rounding errors don't cause errors in the total time.
                    let current_time = (Instant::now() - *start_time.lock().unwrap()).as_millis();
                    let move_time = (current_time - *last_move_time.lock().unwrap()) as u32;
                    *last_move_time.lock().unwrap() = current_time;

                    let mut timed_moves = Vec::new();
                    for (idx, mv) in moves.iter().enumerate() {
                        timed_moves.push(TimedMove::new(*mv, if idx == 0 { move_time } else { 0 }));
                    }

                    // Let clients know there is a new move
                    move_listener(BluetoothCubeEvent::Move(
                        timed_moves,
                        state_copy.lock().unwrap().clone(),
                    ));
                }
                Self::STATE_MESSAGE => {
                    if value.value.len() < 64 {
                        *synced_copy.lock().unwrap() = false;
                        return;
                    }

                    if let Ok(state) = Self::decode_cube_state(&value.value) {
                        *state_copy.lock().unwrap() = state;
                        *state_set_copy.lock().unwrap() = true;
                    } else {
                        *synced_copy.lock().unwrap() = false;
                    }
                }
                Self::BATTERY_MESSAGE => {
                    *battery_percentage_copy.lock().unwrap() = Some(value.value[3] as u32);
                }
                _ => (),
            }
        }));
        device.subscribe(&read)?;

        // Turn off orientation messages
        device.write(
            &write,
            &[Self::DISABLE_ORIENTATION_MESSAGE],
            WriteType::WithResponse,
        )?;

        // Request initial cube state
        let mut loop_count = 0;
        loop {
            device.write(
                &write,
                &[Self::REQUEST_STATE_MESSAGE],
                WriteType::WithResponse,
            )?;

            std::thread::sleep(Duration::from_millis(200));

            if *state_set.lock().unwrap() {
                break;
            }

            loop_count += 1;
            if loop_count > Self::CUBE_STATE_TIMEOUT_MS / 200 {
                return Err(anyhow!("Did not receive initial cube state"));
            }
        }

        // Request battery state
        device.write(
            &write,
            &[Self::REQUEST_BATTERY_MESSAGE],
            WriteType::WithResponse,
        )?;

        Ok(Self {
            device,
            state,
            battery_percentage,
            synced,
            write,
        })
    }

    fn decode_cube_state(data: &[u8]) -> Result<Cube3x3x3> {
        const FACES: [CubeFace; 6] = [
            CubeFace::Back,
            CubeFace::Front,
            CubeFace::Top,
            CubeFace::Bottom,
            CubeFace::Right,
            CubeFace::Left,
        ];
        const COLORS: [Color; 6] = [
            Color::Blue,
            Color::Green,
            Color::White,
            Color::Yellow,
            Color::Red,
            Color::Orange,
        ];
        const ORDER: [usize; 8] = [
            0 * 3 + 0,
            0 * 3 + 1,
            0 * 3 + 2,
            1 * 3 + 2,
            2 * 3 + 2,
            2 * 3 + 1,
            2 * 3 + 0,
            1 * 3 + 0,
        ];
        const ORDER_OFFSET: [usize; 6] = [0, 0, 6, 2, 0, 0];
        let mut state: [Color; 6 * 9] = [Color::White; 6 * 9];

        for face in 0..6 {
            // Find face index in our representation using mapping
            let target_face_idx = FACES[face] as u8 as usize;

            // Place colors into the cube state array
            let offset = target_face_idx * 9;
            state[offset + 1 * 3 + 1] = COLORS[face];
            for i in 0..8 {
                let color_idx = data[4 + face * 9 + i];
                if color_idx >= 6 {
                    return Err(anyhow!("Invalid cube state"));
                }

                state[offset + ORDER[(i + ORDER_OFFSET[face]) % 8]] = COLORS[color_idx as usize];
            }
        }

        // Create cube state and convert to normal format
        Ok(Cube3x3x3Faces::from_colors(state).as_pieces())
    }
}

impl<P: Peripheral> BluetoothCubeDevice for GoCube<P> {
    fn cube_state(&self) -> Cube3x3x3 {
        self.state.lock().unwrap().clone()
    }

    fn battery_percentage(&self) -> Option<u32> {
        *self.battery_percentage.lock().unwrap()
    }

    fn battery_charging(&self) -> Option<bool> {
        // No charging indicator on GoCube
        None
    }

    fn reset_cube_state(&self) {
        let _ = self.device.write(
            &self.write,
            &[Self::RESET_STATE_MESSAGE],
            WriteType::WithResponse,
        );

        *self.state.lock().unwrap() = Cube3x3x3::new();
    }

    fn synced(&self) -> bool {
        *self.synced.lock().unwrap()
    }

    fn disconnect(&self) {
        let _ = self.device.disconnect();
    }
}

pub(crate) fn gocube_connect<P: Peripheral + 'static>(
    device: P,
    move_listener: Box<dyn Fn(BluetoothCubeEvent) + Send + 'static>,
) -> Result<Box<dyn BluetoothCubeDevice>> {
    let characteristics = device.discover_characteristics()?;

    let mut write = None;
    let mut read = None;
    for characteristic in characteristics {
        if characteristic.uuid == Uuid::from_str("6e400002-b5a3-f393-e0a9-e50e24dcca9e").unwrap() {
            write = Some(characteristic);
        } else if characteristic.uuid
            == Uuid::from_str("6e400003-b5a3-f393-e0a9-e50e24dcca9e").unwrap()
        {
            read = Some(characteristic);
        }
    }
    if read.is_some() && write.is_some() {
        Ok(Box::new(GoCube::new(
            device,
            read.unwrap(),
            write.unwrap(),
            move_listener,
        )?))
    } else {
        Err(anyhow!("Unrecognized GoCube version"))
    }
}
