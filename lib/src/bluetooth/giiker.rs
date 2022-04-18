use crate::bluetooth::{BluetoothCubeDevice, BluetoothCubeEvent};
use crate::common::{Cube, InitialCubeState, Move, TimedMove};
use crate::cube3x3x3::Cube3x3x3;
use anyhow::{anyhow, Result};
use btleplug::api::{Characteristic, Peripheral};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use uuid::Uuid;

struct GiikerCube<P: Peripheral + 'static> {
    device: P,
    state: Arc<Mutex<Cube3x3x3>>,
    synced: Arc<Mutex<bool>>,
}

impl<P: Peripheral + 'static> GiikerCube<P> {
    const KEY_STREAM: &'static [u8] = &[
        0xb0, 0x51, 0x68, 0xe0, 0x56, 0x89, 0xed, 0x77, 0x26, 0x1a, 0xc1, 0xa1, 0xd2, 0x7e, 0x96,
        0x51, 0x5d, 0x0d, 0xec, 0xf9, 0x59, 0xeb, 0x58, 0x18, 0x71, 0x51, 0xd6, 0x83, 0x82, 0xc7,
        0x02, 0xa9, 0x27, 0xa5, 0xab, 0x29,
    ];

    pub fn new(
        device: P,
        move_data: Characteristic,
        move_listener: Box<dyn Fn(BluetoothCubeEvent) + Send + 'static>,
    ) -> Result<Self> {
        // Any writes at all to characteristics on this cube will hang forever, as this
        // cube does not respond to bluetooth writes correctly. We cannot request battery
        // state or reset the cube state at all. So we simply assume that the initial
        // state is solved. Resetting the state will only reset the internal state that
        // we maintain.
        let state = Arc::new(Mutex::new(Cube3x3x3::new()));

        let first = Mutex::new(true);
        let state_copy = state.clone();
        let synced = Arc::new(Mutex::new(true));
        let synced_copy = synced.clone();
        let start_time = Mutex::new(Instant::now());
        let last_move_time = Mutex::new(0);

        device.on_notification(Box::new(move |value| {
            let mut value = value.value.clone();
            if value.len() < 20 {
                *synced_copy.lock().unwrap() = false;
                return;
            }

            if *first.lock().unwrap() {
                // Ignore first move data since it is from before connection.
                *first.lock().unwrap() = false;
                return;
            }

            // Check for encoded packets
            if value[18] == 0xa7 {
                let key_offset_a = (value[19] >> 4) as usize;
                let key_offset_b = (value[19] & 0xf) as usize;
                for i in 0..18 {
                    value[i] = value[i].wrapping_add(
                        Self::KEY_STREAM[i + key_offset_a]
                            .wrapping_add(Self::KEY_STREAM[i + key_offset_b]),
                    );
                }
            }

            let mv = match value[16] {
                0x11 => Move::B,
                0x12 => Move::B2,
                0x13 => Move::Bp,
                0x21 => Move::D,
                0x22 => Move::D2,
                0x23 => Move::Dp,
                0x31 => Move::L,
                0x32 => Move::L2,
                0x33 => Move::Lp,
                0x41 => Move::U,
                0x42 => Move::U2,
                0x43 => Move::Up,
                0x51 => Move::R,
                0x52 => Move::R2,
                0x53 => Move::Rp,
                0x61 => Move::F,
                0x62 => Move::F2,
                0x63 => Move::Fp,
                _ => {
                    *synced_copy.lock().unwrap() = false;
                    return;
                }
            };

            // Apply move to the cube state.
            state_copy.lock().unwrap().do_move(mv);

            // Get time since last move. Keep computation relative to start time so
            // that rounding errors don't cause errors in the total time.
            let current_time = (Instant::now() - *start_time.lock().unwrap()).as_millis();
            let move_time = current_time - *last_move_time.lock().unwrap();
            *last_move_time.lock().unwrap() = current_time;

            // Let clients know there is a new move
            move_listener(BluetoothCubeEvent::Move(
                vec![TimedMove::new(mv, move_time as u32)],
                state_copy.lock().unwrap().clone(),
            ));
        }));
        device.subscribe(&move_data)?;

        Ok(Self {
            device,
            state,
            synced,
        })
    }
}

impl<P: Peripheral + 'static> BluetoothCubeDevice for GiikerCube<P> {
    fn cube_state(&self) -> Cube3x3x3 {
        self.state.lock().unwrap().clone()
    }

    fn battery_percentage(&self) -> Option<u32> {
        // Incompatible with some bluetooth stacks, battery level is ignored
        None
    }

    fn battery_charging(&self) -> Option<bool> {
        // Incompatible with some bluetooth stacks, battery level is ignored
        None
    }

    fn reset_cube_state(&self) {
        // Incompatible with some bluetooth stacks, internal state only
        *self.state.lock().unwrap() = Cube3x3x3::new();
    }

    fn synced(&self) -> bool {
        *self.synced.lock().unwrap()
    }

    fn disconnect(&self) {
        let _ = self.device.disconnect();
    }
}

pub(crate) fn giiker_connect<P: Peripheral + 'static>(
    device: P,
    move_listener: Box<dyn Fn(BluetoothCubeEvent) + Send + 'static>,
) -> Result<Box<dyn BluetoothCubeDevice>> {
    let characteristics = device.discover_characteristics()?;

    let mut move_data = None;
    for characteristic in characteristics {
        if characteristic.uuid == Uuid::from_str("0000aadc-0000-1000-8000-00805f9b34fb").unwrap() {
            move_data = Some(characteristic);
        }
    }
    if move_data.is_some() {
        Ok(Box::new(GiikerCube::new(
            device,
            move_data.unwrap(),
            move_listener,
        )?))
    } else {
        Err(anyhow!("Unrecognized Giiker version"))
    }
}
