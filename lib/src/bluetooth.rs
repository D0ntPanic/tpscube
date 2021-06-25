mod gan;
mod gocube;

use crate::common::TimedMove;
use crate::cube3x3x3::Cube3x3x3;
use anyhow::{anyhow, Result};
use btleplug::api::{BDAddr, Central, Peripheral};
use gan::gan_cube_connect;
use gocube::gocube_connect;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

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

    fn needs_update(&self) -> bool {
        false
    }
    fn update(&self) {}
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
}

impl BluetoothCubeType {
    fn from_name(name: &str) -> Option<Self> {
        if name.starts_with("GAN") || name.starts_with("MG") {
            Some(BluetoothCubeType::GAN)
        } else if name.starts_with("GoCube") || name.starts_with("Rubiks") {
            Some(BluetoothCubeType::GoCube)
        } else if name.starts_with("Gi") || name.starts_with("Mi Smart") {
            Some(BluetoothCubeType::Giiker)
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
}

pub struct BluetoothCube {
    discovered_devices: Arc<Mutex<Vec<AvailableDevice>>>,
    to_connect: Arc<Mutex<Option<BDAddr>>>,
    connected_device: Arc<Mutex<Option<Box<dyn BluetoothCubeDevice>>>>,
    listeners:
        Arc<Mutex<HashMap<MoveListenerHandle, Box<dyn Fn(&[TimedMove], &Cube3x3x3) + Send>>>>,
    next_listener_id: AtomicU64,
    error: Arc<Mutex<Option<String>>>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct MoveListenerHandle {
    pub(crate) id: u64,
}

impl BluetoothCube {
    pub fn new() -> Self {
        let discovered_devices = Arc::new(Mutex::new(Vec::new()));
        let to_connect = Arc::new(Mutex::new(None));
        let connected_device = Arc::new(Mutex::new(None));
        let listeners = Arc::new(Mutex::new(HashMap::new()));
        let error = Arc::new(Mutex::new(None));

        let discovered_devices_copy = discovered_devices.clone();
        let to_connect_copy = to_connect.clone();
        let connected_device_copy = connected_device.clone();
        let listeners_copy = listeners.clone();
        let error_copy = error.clone();
        std::thread::spawn(move || {
            match Self::discovery_handler(
                discovered_devices_copy,
                to_connect_copy,
                connected_device_copy,
                listeners_copy,
            ) {
                Err(error) => *error_copy.lock().unwrap() = Some(error.to_string()),
                _ => (),
            }
        });

        Self {
            discovered_devices,
            to_connect,
            connected_device,
            listeners,
            next_listener_id: AtomicU64::new(0),
            error,
        }
    }

    fn discovery_handler(
        discovered_devices: Arc<Mutex<Vec<AvailableDevice>>>,
        to_connect: Arc<Mutex<Option<BDAddr>>>,
        connected_device: Arc<Mutex<Option<Box<dyn BluetoothCubeDevice>>>>,
        listeners: Arc<
            Mutex<HashMap<MoveListenerHandle, Box<dyn Fn(&[TimedMove], &Cube3x3x3) + Send>>>,
        >,
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
                        Self::connect_handler(
                            connected_device.clone(),
                            device,
                            Box::new(move |moves, state| {
                                for listener in listeners_copy.lock().unwrap().iter() {
                                    listener.1(moves, state);
                                }
                            }),
                        )?;
                        return Ok(());
                    }
                }
                return Err(anyhow!("Device no longer available"));
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
        connected_device: Arc<Mutex<Option<Box<dyn BluetoothCubeDevice>>>>,
        peripheral: P,
        move_listener: Box<dyn Fn(&[TimedMove], &Cube3x3x3) + Send + 'static>,
    ) -> Result<()> {
        // Determine cube type
        let cube_type = if let Some(name) = peripheral.properties().local_name {
            match BluetoothCubeType::from_name(&name) {
                Some(cube_type) => cube_type,
                None => return Err(anyhow!("Cube type not recognized")),
            }
        } else {
            return Err(anyhow!("Cube name missing"));
        };

        // Connect to the cube
        peripheral.connect()?;

        let cube = match cube_type {
            BluetoothCubeType::GAN => gan_cube_connect(peripheral, move_listener)?,
            BluetoothCubeType::GoCube => gocube_connect(peripheral, move_listener)?,
            _ => return Err(anyhow!("Cube type not supported")),
        };

        let needs_update = cube.needs_update();

        *connected_device.lock().unwrap() = Some(cube);

        if needs_update {
            // Cube protocol requires active polling
            loop {
                std::thread::sleep(Duration::from_millis(10));
                if let Some(device) = connected_device.lock().unwrap().deref() {
                    device.update();
                } else {
                    // Connection was closed
                    break;
                }
            }
        }

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
        if self.connected_device.lock().unwrap().is_some() {
            Ok(BluetoothCubeState::Connected)
        } else if self.to_connect.lock().unwrap().is_some() {
            Ok(BluetoothCubeState::Connecting)
        } else {
            Ok(BluetoothCubeState::Discovering)
        }
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

    pub fn cube_state(&self) -> Result<Cube3x3x3> {
        self.check_for_error()?;
        match self.connected_device.lock().unwrap().deref() {
            Some(device) => Ok(device.cube_state()),
            None => Err(anyhow!("Cube not connected")),
        }
    }

    pub fn battery_percentage(&self) -> Result<Option<u32>> {
        self.check_for_error()?;
        match self.connected_device.lock().unwrap().deref() {
            Some(device) => Ok(device.battery_percentage()),
            None => Err(anyhow!("Cube not connected")),
        }
    }

    pub fn battery_charging(&self) -> Result<Option<bool>> {
        self.check_for_error()?;
        match self.connected_device.lock().unwrap().deref() {
            Some(device) => Ok(device.battery_charging()),
            None => Err(anyhow!("Cube not connected")),
        }
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
        match self.connected_device.lock().unwrap().deref() {
            Some(device) => Ok(device.synced()),
            None => Err(anyhow!("Cube not connected")),
        }
    }

    pub fn register_move_listener<F: Fn(&[TimedMove], &Cube3x3x3) + Send + 'static>(
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
