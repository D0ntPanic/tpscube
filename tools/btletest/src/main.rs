use std::ops::Deref;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use tpscube_core::{BluetoothCube, BluetoothCubeState};

fn main() {
    let cube = BluetoothCube::new();
    let real_time_start: Mutex<Option<Instant>> = Mutex::new(None);
    let total_time = Mutex::new(0);
    loop {
        if let Some(device) = cube.available_devices().unwrap().iter().next() {
            cube.register_move_listener(move |moves, _state| {
                let mut total_time = total_time.lock().unwrap();
                let mut real_time_start = real_time_start.lock().unwrap();
                if let Some(real_time_start) = real_time_start.deref() {
                    let elapsed = Instant::now() - *real_time_start;
                    for mv in moves {
                        *total_time += mv.time();
                        println!(
                            "Move {}@{}  received at {}",
                            mv.move_().to_string(),
                            *total_time,
                            elapsed.as_millis()
                        );
                    }
                } else {
                    *real_time_start = Some(Instant::now());
                    for mv in moves {
                        println!("Move {}  (initial)", mv.move_().to_string());
                    }
                }
            });
            println!("Connecting to {:?}", device);
            cube.connect(device.address).unwrap();
            loop {
                if cube.state().unwrap() == BluetoothCubeState::Connected {
                    break;
                }
                std::thread::sleep(Duration::from_millis(100));
            }
            println!("Connected");
            loop {
                if !cube.synced().unwrap() {
                    println!("Lost cube sync");
                    return;
                }
                std::thread::sleep(Duration::from_millis(100));
            }
        }

        std::thread::sleep(Duration::from_millis(100));
    }
}
