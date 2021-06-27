use epi::RepaintSignal;
use std::sync::{Arc, Mutex, MutexGuard};

#[cfg(not(target_arch = "wasm32"))]
use std::time::Duration;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

const DEFAULT_MAX_FRAMERATE: u32 = 60;

struct CurrentFramerateData {
    framerate: u32,
    repaint: Arc<dyn RepaintSignal>,
    running: bool,
}

#[cfg(target_arch = "wasm32")]
struct CurrentFramerateInterval {
    _interval: IntervalHandle,
    data: Arc<Mutex<CurrentFramerateData>>,
}

struct CurrentFramerate {
    target: Option<u32>,
    #[cfg(target_arch = "wasm32")]
    interval: Option<CurrentFramerateInterval>,
    #[cfg(not(target_arch = "wasm32"))]
    thread: Option<Arc<Mutex<CurrentFramerateData>>>,
}

pub struct Framerate {
    current: Mutex<CurrentFramerate>,
    repaint: Arc<dyn RepaintSignal>,
    pending_request: Option<u32>,
    max: u32,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    fn setInterval(closure: &Closure<dyn FnMut()>, time: u32) -> i32;
    fn clearInterval(id: i32);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct IntervalHandle {
    interval_id: i32,
    _closure: Closure<dyn FnMut()>,
}

impl Framerate {
    pub fn new(repaint: Arc<dyn RepaintSignal>) -> Self {
        #[cfg(target_arch = "wasm32")]
        let current = CurrentFramerate {
            target: None,
            interval: None,
        };

        #[cfg(not(target_arch = "wasm32"))]
        let current = CurrentFramerate {
            target: None,
            thread: None,
        };

        Self {
            current: Mutex::new(current),
            repaint,
            pending_request: None,
            max: DEFAULT_MAX_FRAMERATE,
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn start_interval(&self, mut current: MutexGuard<'_, CurrentFramerate>) {
        if let Some(interval) = &current.interval {
            interval.data.lock().unwrap().running = false;
            current.interval = None;
        }

        if let Some(framerate) = current.target {
            let data = Arc::new(Mutex::new(CurrentFramerateData {
                framerate,
                repaint: self.repaint.clone(),
                running: true,
            }));
            let closure_data = data.clone();
            let closure = Closure::wrap(Box::new(move || {
                if closure_data.lock().unwrap().running {
                    closure_data.lock().unwrap().repaint.request_repaint();
                }
            }) as Box<dyn FnMut()>);
            let interval_id = setInterval(&closure, 1000 / framerate);
            current.interval = Some(CurrentFramerateInterval {
                _interval: IntervalHandle {
                    interval_id,
                    _closure: closure,
                },
                data,
            });
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn start_thread(&self, mut current: MutexGuard<'_, CurrentFramerate>) {
        if let Some(thread) = &current.thread {
            thread.lock().unwrap().running = false;
            current.thread = None;
        }

        if let Some(framerate) = current.target {
            let data = Arc::new(Mutex::new(CurrentFramerateData {
                framerate,
                repaint: self.repaint.clone(),
                running: true,
            }));
            current.thread = Some(data.clone());
            std::thread::spawn(move || {
                let framerate = data.lock().unwrap().framerate;
                let repaint = data.lock().unwrap().repaint.clone();

                while data.lock().unwrap().running {
                    repaint.request_repaint();
                    std::thread::sleep(Duration::from_secs_f32(1.0 / framerate as f32));
                }
            });
        }
    }

    pub fn request(&mut self, target: Option<u32>) {
        if let Some(existing_request) = &self.pending_request {
            if let Some(target) = target {
                if target > *existing_request {
                    self.pending_request = Some(target);
                }
            }
        } else {
            self.pending_request = target;
        }
    }

    pub fn request_max(&mut self) {
        self.request(Some(self.max));
    }

    pub fn commit(&mut self) {
        let target = self.pending_request;
        self.pending_request = None;

        let mut current = self.current.lock().unwrap();
        if current.target != target {
            current.target = target;

            #[cfg(target_arch = "wasm32")]
            self.start_interval(current);
            #[cfg(not(target_arch = "wasm32"))]
            self.start_thread(current);
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl Drop for IntervalHandle {
    fn drop(&mut self) {
        clearInterval(self.interval_id);
    }
}
