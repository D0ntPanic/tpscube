mod gan;
mod giiker;
mod gocube;
mod moyu;

use crate::common::TimedMove;
use crate::cube3x3x3::Cube3x3x3;
use anyhow::{anyhow, Result};
use btleplug::api::{BDAddr, Central, Peripheral};
use gan::gan_cube_connect;
use giiker::giiker_connect;
use gocube::gocube_connect;
use moyu::moyu_connect;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[cfg(target_os = "linux")]
use btleplug::bluez::{adapter::Adapter, manager::Manager};
#[cfg(target_os = "macos")]
use btleplug::corebluetooth::manager::Manager;
#[cfg(target_os = "windows")]
use btleplug::winrtble::{adapter::Adapter, manager::Manager};

pub(crate) trait BluetoothCubeDevice: Send {
    fn cube_state(&self) -> Cube3x3x3;
    fn battery_percentage(&self) -> Option<u32>;
    fn battery_charging(&self) -> Option<bool>;
    fn reset_cube_state(&self);
    fn synced(&self) -> bool;
    fn update(&self) {}
    fn disconnect(&self);
    fn timer_only(&self) -> bool {
        false
    }

    fn estimated_clock_ratio(&self) -> f64 {
        1.0
    }
    fn clock_ratio_range(&self) -> (f64, f64) {
        (0.98, 1.02)
    }
}

#[derive(Clone, Debug)]
pub struct AvailableDevice {
    pub address: BDAddr,
    pub name: String,
    pub cube_type: BluetoothCubeType,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BluetoothCubeType {
    GAN,
    GoCube,
    Giiker,
    MoYu,
}

impl BluetoothCubeType {
    fn from_name(name: &str) -> Option<Self> {
        if name.starts_with("GAN") || name.starts_with("MG") {
            Some(BluetoothCubeType::GAN)
        } else if name.starts_with("GoCube") || name.starts_with("Rubiks") {
            Some(BluetoothCubeType::GoCube)
        } else if name.starts_with("Gi") || name.starts_with("Mi Smart") {
            Some(BluetoothCubeType::Giiker)
        } else if name.starts_with("MHC-") {
            Some(BluetoothCubeType::MoYu)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BluetoothCubeState {
    Discovering,
    Connecting,
    Connected,
    Desynced,
    Error,
}

#[derive(Clone)]
pub enum BluetoothCubeEvent {
    Move(Vec<TimedMove>, Cube3x3x3),
    HandsOnTimer,
    TimerStartCancel,
    TimerReady,
    TimerStarted,
    TimerFinished(u32),
}

pub struct BluetoothCube {
    discovered_devices: Arc<Mutex<Vec<AvailableDevice>>>,
    to_connect: Arc<Mutex<Option<BDAddr>>>,
    state: Arc<Mutex<BluetoothCubeState>>,
    connected_device: Arc<Mutex<Option<Box<dyn BluetoothCubeDevice>>>>,
    connected_name: Arc<Mutex<Option<String>>>,
    battery: Arc<Mutex<(Option<u32>, Option<bool>)>>,
    listeners: Arc<Mutex<HashMap<MoveListenerHandle, Box<dyn Fn(BluetoothCubeEvent) + Send>>>>,
    next_listener_id: AtomicU64,
    error: Arc<Mutex<Option<String>>>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct MoveListenerHandle {
    id: u64,
}

impl BluetoothCube {
    pub fn new() -> Self {
        let discovered_devices = Arc::new(Mutex::new(Vec::new()));
        let to_connect = Arc::new(Mutex::new(None));
        let state = Arc::new(Mutex::new(BluetoothCubeState::Discovering));
        let connected_device = Arc::new(Mutex::new(None));
        let connected_name = Arc::new(Mutex::new(None));
        let battery = Arc::new(Mutex::new((None, None)));
        let listeners = Arc::new(Mutex::new(HashMap::new()));
        let error = Arc::new(Mutex::new(None));

        let discovered_devices_copy = discovered_devices.clone();
        let to_connect_copy = to_connect.clone();
        let state_copy = state.clone();
        let connected_device_copy = connected_device.clone();
        let connected_name_copy = connected_name.clone();
        let battery_copy = battery.clone();
        let listeners_copy = listeners.clone();
        let error_copy = error.clone();
        std::thread::spawn(move || {
            match Self::discovery_handler(
                discovered_devices_copy,
                to_connect_copy,
                state_copy.clone(),
                connected_device_copy,
                connected_name_copy,
                battery_copy,
                listeners_copy,
            ) {
                Err(error) => {
                    *state_copy.lock().unwrap() = BluetoothCubeState::Error;
                    *error_copy.lock().unwrap() = Some(error.to_string());
                }
                _ => (),
            }
        });

        Self {
            discovered_devices,
            to_connect,
            state,
            connected_device,
            connected_name,
            battery,
            listeners,
            next_listener_id: AtomicU64::new(0),
            error,
        }
    }

    fn discovery_handler(
        discovered_devices: Arc<Mutex<Vec<AvailableDevice>>>,
        to_connect: Arc<Mutex<Option<BDAddr>>>,
        state: Arc<Mutex<BluetoothCubeState>>,
        connected_device: Arc<Mutex<Option<Box<dyn BluetoothCubeDevice>>>>,
        connected_name: Arc<Mutex<Option<String>>>,
        battery: Arc<Mutex<(Option<u32>, Option<bool>)>>,
        listeners: Arc<Mutex<HashMap<MoveListenerHandle, Box<dyn Fn(BluetoothCubeEvent) + Send>>>>,
    ) -> Result<()> {
        let manager = Manager::new()?;
        let adapter = manager.adapters()?;
        let central = adapter
            .into_iter()
            .nth(0)
            .ok_or_else(|| anyhow!("No Bluetooth adapters found"))?;
        central.start_scan()?;

        loop {
            // See if the client asked to connect to a cube
            let to_connect = to_connect.lock().unwrap().clone();
            if let Some(to_connect) = to_connect {
                // Look for the cube in the device list to get the Peripheral object
                for device in central.peripherals() {
                    if to_connect == device.address() {
                        let listeners_copy = listeners.clone();

                        // Set up time calibration state
                        struct TimeCalibrationState {
                            start_time: Option<Instant>,
                            last_move_time: Option<Instant>,
                            current_duration: Duration,
                            total_raw_ticks: u64,
                            total_real_ticks: u64,
                            clock_ratio: f64,
                            clock_ratio_range: (f64, f64),
                        }
                        let calibration_state = Arc::new(Mutex::new(TimeCalibrationState {
                            start_time: None,
                            last_move_time: None,
                            current_duration: Duration::from_secs(0),
                            total_raw_ticks: 0,
                            total_real_ticks: 0,
                            clock_ratio: 1.0,
                            clock_ratio_range: (0.98, 1.02),
                        }));
                        let init_calibration_state = calibration_state.clone();

                        let _ = Self::connect_handler(
                            state.clone(),
                            connected_device.clone(),
                            connected_name.clone(),
                            battery.clone(),
                            device,
                            Box::new(move |cube| {
                                init_calibration_state.lock().unwrap().clock_ratio =
                                    cube.estimated_clock_ratio();
                                init_calibration_state.lock().unwrap().clock_ratio_range =
                                    cube.clock_ratio_range();
                            }),
                            Box::new(move |event| {
                                match event {
                                    BluetoothCubeEvent::Move(moves, state) => {
                                        // We can't use the move timing data directly. Some cubes have very
                                        // uncalibrated clocks and we must adjust the timing to match real
                                        // time, with the host device as the reference source.
                                        let mut calibration_state =
                                            calibration_state.lock().unwrap();
                                        let now = Instant::now();
                                        let mut last_duration = calibration_state.current_duration;

                                        // Check length of time since last move
                                        let mut calibration_reset = false;
                                        if let Some(last_move_time) =
                                            calibration_state.last_move_time
                                        {
                                            let delta = now - last_move_time;
                                            if delta.as_secs() > 30 {
                                                // More than 30 seconds between moves, don't adjust clock ratio to
                                                // avoid issues with the range of the encodings of some cubes.
                                                // Adjust timestamp using real time.
                                                calibration_reset = true;
                                                calibration_state.current_duration += delta;
                                            }
                                        }

                                        // Go through the move list and adjust the timing information
                                        let mut adjusted_moves = Vec::new();
                                        let mut new_raw_ticks = 0;
                                        for raw_move in moves {
                                            let mv = raw_move.move_();
                                            let raw_time = raw_move.time();

                                            if !calibration_reset {
                                                new_raw_ticks += raw_time;

                                                // Adjust delta using clock ratio. This will be adjusted
                                                // over time to be calibrated to real time.
                                                let adjusted_delta = Duration::from_nanos(
                                                    ((raw_time as u64 * 1_000_000) as f64
                                                        / calibration_state.clock_ratio)
                                                        as u64,
                                                );
                                                calibration_state.current_duration +=
                                                    adjusted_delta;
                                            }

                                            // Add adjusted timing information to new move list
                                            let adjusted_time =
                                                (calibration_state.current_duration).as_millis()
                                                    - last_duration.as_millis();
                                            last_duration = calibration_state.current_duration;
                                            adjusted_moves
                                                .push(TimedMove::new(mv, adjusted_time as u32));
                                        }

                                        // Update calibration state
                                        if let Some(start_time_deref) = calibration_state.start_time
                                        {
                                            if calibration_reset {
                                                // Calibration is being reset because of too much time
                                                // between moves. Measure from this move forward.
                                                calibration_state.start_time = Some(now);
                                                calibration_state.total_raw_ticks = 0;
                                                calibration_state.total_real_ticks = 0;
                                            } else {
                                                // Update the calibration with the number of milliseconds
                                                // reported in the raw data and the number of milliseconds
                                                // that have actually passed.
                                                calibration_state.total_raw_ticks +=
                                                    new_raw_ticks as u64;
                                                calibration_state.total_real_ticks =
                                                    (now - start_time_deref).as_millis() as u64;

                                                // Compute ratio between raw time and real time
                                                let computed_clock_ratio = (calibration_state
                                                    .total_raw_ticks
                                                    as f64)
                                                    / (calibration_state.total_real_ticks as f64);

                                                // Clamp ratio to a range for sanity check
                                                calibration_state.clock_ratio =
                                                    computed_clock_ratio
                                                        .max(
                                                            (calibration_state.clock_ratio_range).0,
                                                        )
                                                        .min(
                                                            (calibration_state.clock_ratio_range).1,
                                                        );
                                            }
                                        } else {
                                            // First move, record start time
                                            calibration_state.start_time = Some(now);
                                        }

                                        // Keep track of last move's real time
                                        calibration_state.last_move_time = Some(now);

                                        // Notify clients of the move information
                                        for listener in listeners_copy.lock().unwrap().iter() {
                                            listener.1(BluetoothCubeEvent::Move(
                                                adjusted_moves.clone(),
                                                state.clone(),
                                            ));
                                        }
                                    }
                                    event => {
                                        // Notify clients of the event
                                        for listener in listeners_copy.lock().unwrap().iter() {
                                            listener.1(event.clone());
                                        }
                                    }
                                }
                            }),
                        );
                    }
                }
            }

            // Enumerate devices
            let mut new_devices = Vec::new();
            for device in central.peripherals() {
                if let Some(name) = device.properties().local_name {
                    match BluetoothCubeType::from_name(&name) {
                        Some(cube_type) => {
                            new_devices.push(AvailableDevice {
                                address: device.address(),
                                name: name.clone(),
                                cube_type,
                            });
                        }
                        None => (),
                    }
                }
            }
            *discovered_devices.lock().unwrap() = new_devices;

            // Wait before checking devices again. We can't use the event-based system
            // since we also need to check for client connection requests.
            std::thread::sleep(Duration::from_millis(100));
        }
    }

    fn connect_handler<P: Peripheral + 'static>(
        state: Arc<Mutex<BluetoothCubeState>>,
        connected_device: Arc<Mutex<Option<Box<dyn BluetoothCubeDevice>>>>,
        connected_name: Arc<Mutex<Option<String>>>,
        battery: Arc<Mutex<(Option<u32>, Option<bool>)>>,
        peripheral: P,
        init: Box<dyn Fn(&dyn BluetoothCubeDevice) + Send + 'static>,
        move_listener: Box<dyn Fn(BluetoothCubeEvent) + Send + 'static>,
    ) -> Result<()> {
        // Determine cube type
        let name = peripheral.properties().local_name.clone();
        let cube_type = if let Some(name) = &name {
            match BluetoothCubeType::from_name(&name) {
                Some(cube_type) => cube_type,
                None => return Err(anyhow!("Cube type not recognized")),
            }
        } else {
            return Err(anyhow!("Cube name missing"));
        };

        *state.lock().unwrap() = BluetoothCubeState::Connecting;

        // Connect to the cube
        peripheral.connect()?;

        let cube = match cube_type {
            BluetoothCubeType::GAN => gan_cube_connect(peripheral, move_listener)?,
            BluetoothCubeType::GoCube => gocube_connect(peripheral, move_listener)?,
            BluetoothCubeType::Giiker => giiker_connect(peripheral, move_listener)?,
            BluetoothCubeType::MoYu => moyu_connect(peripheral, move_listener)?,
        };

        init(cube.as_ref());

        *connected_device.lock().unwrap() = Some(cube);
        *connected_name.lock().unwrap() = name;
        *state.lock().unwrap() = BluetoothCubeState::Connected;

        loop {
            std::thread::sleep(Duration::from_millis(10));
            if let Some(device) = connected_device.lock().unwrap().deref() {
                device.update();
                if !device.synced() {
                    *state.lock().unwrap() = BluetoothCubeState::Desynced;
                }
                *battery.lock().unwrap() = (device.battery_percentage(), device.battery_charging())
            } else {
                // Connection was closed
                break;
            }
        }

        *state.lock().unwrap() = BluetoothCubeState::Discovering;
        *connected_device.lock().unwrap() = None;
        *connected_name.lock().unwrap() = None;
        *battery.lock().unwrap() = (None, None);

        Ok(())
    }

    fn check_for_error(&self) -> Result<()> {
        match self.error.lock().unwrap().deref() {
            Some(error) => Err(anyhow!("{}", error)),
            None => Ok(()),
        }
    }

    pub fn state(&self) -> Result<BluetoothCubeState> {
        self.check_for_error()?;
        Ok(*self.state.lock().unwrap())
    }

    pub fn available_devices(&self) -> Result<Vec<AvailableDevice>> {
        self.check_for_error()?;
        Ok(self.discovered_devices.lock().unwrap().clone())
    }

    pub fn connect(&self, address: BDAddr) -> Result<()> {
        self.check_for_error()?;
        *self.to_connect.lock().unwrap() = Some(address);
        Ok(())
    }

    pub fn disconnect(&self) {
        match self.connected_device.lock().unwrap().deref() {
            Some(device) => device.disconnect(),
            _ => (),
        }

        *self.to_connect.lock().unwrap() = None;
        *self.connected_device.lock().unwrap() = None;
    }

    pub fn name(&self) -> Result<Option<String>> {
        self.check_for_error()?;
        Ok(self.connected_name.lock().unwrap().clone())
    }

    pub fn timer_only(&self) -> Result<bool> {
        self.check_for_error()?;
        match self.connected_device.lock().unwrap().deref() {
            Some(device) => Ok(device.timer_only()),
            None => Err(anyhow!("Cube not connected")),
        }
    }

    pub fn cube_state(&self) -> Result<Cube3x3x3> {
        self.check_for_error()?;
        match self.connected_device.lock().unwrap().deref() {
            Some(device) => Ok(device.cube_state()),
            None => Err(anyhow!("Cube not connected")),
        }
    }

    pub fn battery_percentage(&self) -> Result<Option<u32>> {
        self.check_for_error()?;
        Ok(self.battery.lock().unwrap().0)
    }

    pub fn battery_charging(&self) -> Result<Option<bool>> {
        self.check_for_error()?;
        Ok(self.battery.lock().unwrap().1)
    }

    pub fn reset_cube_state(&self) -> Result<()> {
        self.check_for_error()?;
        match self.connected_device.lock().unwrap().deref() {
            Some(device) => {
                device.reset_cube_state();
                Ok(())
            }
            None => Err(anyhow!("Cube not connected")),
        }
    }

    pub fn synced(&self) -> Result<bool> {
        self.check_for_error()?;
        Ok(*self.state.lock().unwrap() == BluetoothCubeState::Connected)
    }

    pub fn register_move_listener<F: Fn(BluetoothCubeEvent) + Send + 'static>(
        &self,
        func: F,
    ) -> MoveListenerHandle {
        let id = self.next_listener_id.fetch_add(1, Ordering::SeqCst);
        let handle = MoveListenerHandle { id };
        self.listeners
            .lock()
            .unwrap()
            .insert(handle.clone(), Box::new(func));
        handle
    }

    pub fn unregister_move_listener(&self, handle: MoveListenerHandle) {
        self.listeners.lock().unwrap().remove(&handle);
    }
}

impl Drop for BluetoothCube {
    fn drop(&mut self) {
        // Clear connected device to force any polling threads to stop
        *self.connected_device.lock().unwrap() = None;
    }
}
