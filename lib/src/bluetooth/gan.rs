use crate::bluetooth::BluetoothCubeDevice;
use crate::common::{Cube, Move, TimedMove};
use crate::cube3x3x3::{Corner3x3x3, CornerPiece3x3x3, Cube3x3x3, Edge3x3x3, EdgePiece3x3x3};
use aes::{
    cipher::generic_array::GenericArray,
    cipher::{BlockDecrypt, BlockEncrypt},
    Aes128, Block, NewBlockCipher,
};
use anyhow::{anyhow, Result};
use btleplug::api::{Characteristic, Peripheral, WriteType};
use std::collections::HashSet;
use std::convert::{TryFrom, TryInto};
use std::iter::FromIterator;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use uuid::Uuid;

const CUBE_MOVES_MESSAGE: u8 = 2;
const CUBE_STATE_MESSAGE: u8 = 4;
const BATTERY_STATE_MESSAGE: u8 = 9;
const RESET_CUBE_STATE_MESSAGE: u8 = 10;

const CUBE_STATE_TIMEOUT_MS: usize = 2000;

struct GANCubeVersion2<P: Peripheral + 'static> {
    device: P,
    state: Arc<Mutex<Cube3x3x3>>,
    battery_percentage: Arc<Mutex<Option<u32>>>,
    battery_charging: Arc<Mutex<Option<bool>>>,
    synced: Arc<Mutex<bool>>,
    write: Characteristic,
    cipher: GANCubeVersion2Cipher,
}

#[derive(Clone)]
struct GANCubeVersion2Cipher {
    device_key: [u8; 16],
    device_iv: [u8; 16],
}

impl<P: Peripheral> GANCubeVersion2<P> {
    pub fn new(
        device: P,
        read: Characteristic,
        write: Characteristic,
        move_listener: Box<dyn Fn(&[TimedMove], &Cube3x3x3) + Send + 'static>,
    ) -> Result<Self> {
        // Derive keys. These are based on a 6 byte device identifier found in the
        // manufacturer data.
        let device_key: [u8; 6] = if let Some(data) = device.properties().manufacturer_data.get(&1)
        {
            if data.len() >= 9 {
                let mut result = [0; 6];
                result.copy_from_slice(&data[3..9]);
                result
            } else {
                return Err(anyhow!("Device identifier data invalid"));
            }
        } else {
            return Err(anyhow!("Manufacturer data missing device identifier"));
        };

        const GAN_V2_KEY: [u8; 16] = [
            0x01, 0x02, 0x42, 0x28, 0x31, 0x91, 0x16, 0x07, 0x20, 0x05, 0x18, 0x54, 0x42, 0x11,
            0x12, 0x53,
        ];
        const GAN_V2_IV: [u8; 16] = [
            0x11, 0x03, 0x32, 0x28, 0x21, 0x01, 0x76, 0x27, 0x20, 0x95, 0x78, 0x14, 0x32, 0x12,
            0x02, 0x43,
        ];
        let mut key = GAN_V2_KEY.clone();
        let mut iv = GAN_V2_IV.clone();
        for (idx, byte) in device_key.iter().enumerate() {
            key[idx] = ((key[idx] as u16 + *byte as u16) % 255) as u8;
            iv[idx] = ((iv[idx] as u16 + *byte as u16) % 255) as u8;
        }
        let cipher = GANCubeVersion2Cipher {
            device_key: key,
            device_iv: iv,
        };

        let state = Arc::new(Mutex::new(Cube3x3x3::new()));
        let state_set = Arc::new(Mutex::new(false));
        let battery_percentage = Arc::new(Mutex::new(None));
        let battery_charging = Arc::new(Mutex::new(None));
        let last_move_count = Arc::new(Mutex::new(None));
        let synced = Arc::new(Mutex::new(true));

        let cipher_copy = cipher.clone();
        let state_copy = state.clone();
        let state_set_copy = state_set.clone();
        let battery_percentage_copy = battery_percentage.clone();
        let battery_charging_copy = battery_charging.clone();
        let synced_copy = synced.clone();

        device.on_notification(Box::new(move |value| {
            if let Ok(value) = cipher_copy.decrypt(&value.value) {
                let message_type = Self::extract_bits(&value, 0, 4) as u8;
                match message_type {
                    CUBE_MOVES_MESSAGE => {
                        let current_move_count = Self::extract_bits(&value, 4, 8) as u8;

                        // If we haven't received a cube state message yet, we can't know what
                        // the curent cube state is. Ignore moves until the cube state message
                        // is received. If there has been a cube state message, we will have
                        // a last move count and we can continue.
                        let mut last_move_count_option = last_move_count.lock().unwrap();
                        if let Some(last_move_count) = *last_move_count_option {
                            // Check number of moves since last message.
                            let move_count =
                                current_move_count.wrapping_sub(last_move_count) as usize;
                            if move_count > 7 {
                                // There are too many moves since the last message. Our cube
                                // state is out of sync. Let the client know and reset the
                                // last move count such that we don't parse any more move
                                // messages, since they aren't valid anymore.
                                *synced_copy.lock().unwrap() = false;
                                *last_move_count_option = None;
                                return;
                            }

                            // Gather the moves
                            let mut moves = Vec::with_capacity(move_count);
                            for j in 0..move_count {
                                // Build move list in reverse order. In the packet the moves
                                // are from the latest move to the oldest move, but the callback
                                // should take the moves in the order they happened.
                                let i = (move_count - 1) - j;

                                // Decode move data
                                let move_num = Self::extract_bits(&value, 12 + i * 5, 5) as usize;
                                let move_time = Self::extract_bits(&value, 12 + 7 * 5 + i * 16, 16);
                                const MOVES: &[Move] = &[
                                    Move::U,
                                    Move::Up,
                                    Move::R,
                                    Move::Rp,
                                    Move::F,
                                    Move::Fp,
                                    Move::D,
                                    Move::Dp,
                                    Move::L,
                                    Move::Lp,
                                    Move::B,
                                    Move::Bp,
                                ];
                                if move_num >= MOVES.len() {
                                    // Bad move data. Cube is now desynced.
                                    *synced_copy.lock().unwrap() = false;
                                    *last_move_count_option = None;
                                    return;
                                }
                                let mv = MOVES[move_num];
                                moves.push(TimedMove::new(mv, move_time));

                                // Apply move to the cube state.
                                state_copy.lock().unwrap().do_move(mv);
                            }

                            *last_move_count_option = Some(current_move_count);

                            // Let clients know there is a new move
                            move_listener(&moves, state_copy.lock().unwrap().deref());
                        }
                    }
                    CUBE_STATE_MESSAGE => {
                        *last_move_count.lock().unwrap() =
                            Some(Self::extract_bits(&value, 4, 8) as u8);

                        // Set up corner and edge state
                        let mut corners = [0; 8];
                        let mut corner_twist = [0; 8];
                        let mut corners_left: HashSet<u32> =
                            HashSet::from_iter((&[0, 1, 2, 3, 4, 5, 6, 7]).iter().cloned());
                        let mut edges = [0; 12];
                        let mut edge_parity = [0; 12];
                        let mut edges_left: HashSet<u32> = HashSet::from_iter(
                            (&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]).iter().cloned(),
                        );
                        let mut total_corner_twist = 0;
                        let mut total_edge_parity = 0;

                        // Decode corners. There are only 7 in the packet because the
                        // last one is implicit (the one missing).
                        for i in 0..7 {
                            corners[i] = Self::extract_bits(&value, 12 + i * 3, 3);
                            corner_twist[i] = Self::extract_bits(&value, 33 + i * 2, 2);
                            total_corner_twist += corner_twist[i];
                            if !corners_left.remove(&corners[i]) || corner_twist[i] >= 3 {
                                return;
                            }
                        }

                        // Decode edges. There are only 11 in the packet because the
                        // last one is implicit (the one missing).
                        for i in 0..11 {
                            edges[i] = Self::extract_bits(&value, 47 + i * 4, 4);
                            edge_parity[i] = Self::extract_bits(&value, 91 + i, 1);
                            total_edge_parity += edge_parity[i];
                            if !edges_left.remove(&edges[i]) || edge_parity[i] >= 2 {
                                return;
                            }
                        }

                        // Add in the missing corner and edge based on the last one
                        // left. There will always be exactly one left since we
                        // already verified each corner and edge was unique.
                        corners[7] = *corners_left.iter().next().unwrap();
                        edges[11] = *edges_left.iter().next().unwrap();

                        // Compute the corner twist and edge parity of the last corner
                        // and edge piece. The corner twist must be a multiple of 3 and
                        // the edge parity must be even.
                        corner_twist[7] = (3 - total_corner_twist % 3) % 3;
                        edge_parity[11] = total_edge_parity & 1;

                        // Create cube state. Our representation of the cube state matches
                        // the one used in the packet. We have already verified the data
                        // is valid so we can unwrap the conversions with panic.
                        let mut corner_pieces = Vec::with_capacity(8);
                        let mut edge_pieces = Vec::with_capacity(12);
                        for i in 0..8 {
                            corner_pieces.push(CornerPiece3x3x3 {
                                piece: Corner3x3x3::try_from(corners[i] as u8).unwrap(),
                                orientation: corner_twist[i] as u8,
                            });
                        }
                        for i in 0..12 {
                            edge_pieces.push(EdgePiece3x3x3 {
                                piece: Edge3x3x3::try_from(edges[i] as u8).unwrap(),
                                orientation: edge_parity[i] as u8,
                            });
                        }

                        let cube = Cube3x3x3::from_corners_and_edges(
                            corner_pieces.try_into().unwrap(),
                            edge_pieces.try_into().unwrap(),
                        );

                        *state_copy.lock().unwrap() = cube;
                        *state_set_copy.lock().unwrap() = true;
                    }
                    BATTERY_STATE_MESSAGE => {
                        *battery_charging_copy.lock().unwrap() =
                            Some(Self::extract_bits(&value, 4, 4) != 0);
                        *battery_percentage_copy.lock().unwrap() =
                            Some(Self::extract_bits(&value, 8, 8));
                    }
                    _ => (),
                }
            }
        }));
        device.subscribe(&read).unwrap();

        // Request initial cube state
        let mut loop_count = 0;
        loop {
            let mut message: [u8; 20] = [0; 20];
            message[0] = CUBE_STATE_MESSAGE;
            let message = cipher.encrypt(&message)?;
            device.write(&write, &message, WriteType::WithResponse)?;

            std::thread::sleep(Duration::from_millis(200));

            if *state_set.lock().unwrap() {
                break;
            }

            loop_count += 1;
            if loop_count > CUBE_STATE_TIMEOUT_MS / 200 {
                return Err(anyhow!("Did not receive initial cube state"));
            }
        }

        // Request battery state immediately
        let mut message: [u8; 20] = [0; 20];
        message[0] = BATTERY_STATE_MESSAGE;
        let message = cipher.encrypt(&message)?;
        device.write(&write, &message, WriteType::WithResponse)?;

        Ok(Self {
            device,
            state,
            battery_percentage,
            battery_charging,
            synced,
            cipher,
            write,
        })
    }

    fn extract_bits(data: &[u8], start: usize, count: usize) -> u32 {
        let mut result = 0;
        for i in 0..count {
            let bit = start + i;
            result <<= 1;
            if data[bit / 8] & (1 << (7 - (bit % 8))) != 0 {
                result |= 1;
            }
        }
        result
    }
}

impl GANCubeVersion2Cipher {
    fn decrypt(&self, value: &[u8]) -> Result<Vec<u8>> {
        if value.len() <= 16 {
            return Err(anyhow!("Packet size less than expected length"));
        }

        // Packets are larger than block size. First decrypt the last 16 bytes
        // of the packet in place.
        let mut value = value.to_vec();
        let aes = Aes128::new(GenericArray::from_slice(&self.device_key));
        let offset = value.len() - 16;
        let end_cipher = &value[offset..];
        let mut end_plain = Block::clone_from_slice(end_cipher);
        aes.decrypt_block(&mut end_plain);
        for i in 0..16 {
            end_plain[i] ^= self.device_iv[i];
            value[offset + i] = end_plain[i];
        }

        // Decrypt the first 16 bytes of the packet in place. This will overlap
        // with the decrypted block above.
        let start_cipher = &value[0..16];
        let mut start_plain = Block::clone_from_slice(start_cipher);
        aes.decrypt_block(&mut start_plain);
        for i in 0..16 {
            start_plain[i] ^= self.device_iv[i];
            value[i] = start_plain[i];
        }

        Ok(value)
    }

    fn encrypt(&self, value: &[u8]) -> Result<Vec<u8>> {
        if value.len() <= 16 {
            return Err(anyhow!("Packet size less than expected length"));
        }

        // Packets are larger than block size. First encrypt the first 16 bytes
        // of the packet in place.
        let mut value = value.to_vec();
        for i in 0..16 {
            value[i] ^= self.device_iv[i];
        }
        let mut cipher = Block::clone_from_slice(&value[0..16]);
        let aes = Aes128::new(GenericArray::from_slice(&self.device_key));
        aes.encrypt_block(&mut cipher);
        for i in 0..16 {
            value[i] = cipher[i];
        }

        // Decrypt the last 16 bytes of the packet in place. This will overlap
        // with the decrypted block above.
        let offset = value.len() - 16;
        for i in 0..16 {
            value[offset + i] ^= self.device_iv[i];
        }
        let mut cipher = Block::clone_from_slice(&value[offset..]);
        aes.encrypt_block(&mut cipher);
        for i in 0..16 {
            value[offset + i] = cipher[i];
        }

        Ok(value)
    }
}

impl<P: Peripheral> BluetoothCubeDevice for GANCubeVersion2<P> {
    fn cube_state(&self) -> Cube3x3x3 {
        self.state.lock().unwrap().clone()
    }

    fn battery_percentage(&self) -> Option<u32> {
        *self.battery_percentage.lock().unwrap()
    }

    fn battery_charging(&self) -> Option<bool> {
        *self.battery_charging.lock().unwrap()
    }

    fn reset_cube_state(&self) {
        // These bytes represent the cube state in the solved state.
        let message: [u8; 20] = [
            RESET_CUBE_STATE_MESSAGE,
            0x05,
            0x39,
            0x77,
            0x00,
            0x00,
            0x01,
            0x23,
            0x45,
            0x67,
            0x89,
            0xab,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
        ];
        let message = self.cipher.encrypt(&message).unwrap();
        let _ = self
            .device
            .write(&self.write, &message, WriteType::WithResponse);
    }

    fn synced(&self) -> bool {
        *self.synced.lock().unwrap()
    }
}

pub(crate) fn gan_cube_connect<P: Peripheral + 'static>(
    device: P,
    move_listener: Box<dyn Fn(&[TimedMove], &Cube3x3x3) + Send + 'static>,
) -> Result<Box<dyn BluetoothCubeDevice>> {
    let characteristics = device.discover_characteristics()?;

    // Find characteristics for communicating with the cube. There are two different
    // versions of the GAN cubes with different characteristics.
    let mut v2_write = None;
    let mut v2_read = None;
    for characteristic in characteristics {
        if characteristic.uuid == Uuid::from_str("28be4a4a-cd67-11e9-a32f-2a2ae2dbcce4").unwrap() {
            v2_write = Some(characteristic);
        } else if characteristic.uuid
            == Uuid::from_str("28be4cb6-cd67-11e9-a32f-2a2ae2dbcce4").unwrap()
        {
            v2_read = Some(characteristic);
        }
    }

    // Create cube object based on available characteristics
    if v2_read.is_some() && v2_write.is_some() {
        Ok(Box::new(GANCubeVersion2::new(
            device,
            v2_read.unwrap(),
            v2_write.unwrap(),
            move_listener,
        )?))
    } else {
        Err(anyhow!("Unrecognized GAN cube version"))
    }
}
