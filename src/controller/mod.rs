use std::fmt;
use std::fmt::Display;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use bevy::prelude::Resource;
use rc_controller;
use rc_controller::fpv_controller::BasicFPVController;

#[derive(Resource)]
pub struct Controller {
    pub controller: Option<Arc<Mutex<rc_controller::fpv_controller::BasicFPVController<'static>>>>,
    pub last_input: Arc<Mutex<(f32, f32, f32, f32)>>,
    pub thread: Option<JoinHandle<()>>,
    pub flag_to_stop: Arc<AtomicBool>,
}
impl Display for Controller {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "()")
    }
}

impl self::Controller {
    pub fn init(&mut self, c: BasicFPVController<'static>) {
        self.controller = Some(Arc::new(Mutex::new(c)));
    }
}
impl Default for Controller {
    fn default() -> Self {
        Self {
            controller: None, //Some(Arc::new(Mutex::new(simple_loader(2.0)))),
            last_input: Arc::new(Mutex::new((0.0, 0.0, 0.0, 0.0))),
            thread: None,
            flag_to_stop: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl Drop for Controller {
    fn drop(&mut self) {
        self.flag_to_stop
            .store(true, std::sync::atomic::Ordering::Relaxed);
        if let Some(t) = self.thread.take() {
            t.join().unwrap();
            println!("Ok");
        }
    }
}

pub mod plugin;
pub mod systems;
