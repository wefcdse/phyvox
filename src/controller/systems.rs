use std::{
    sync::{atomic::AtomicBool, Arc, Mutex},
    thread,
};

use bevy::prelude::*;
use rc_controller::{fpv_controller::BasicFPVController, simple_loader::simple_loader};

use super::Controller;

pub fn startup(commands: &mut Commands, c: BasicFPVController<'static>) {
    commands.insert_resource(Controller {
        controller: Some(Arc::new(Mutex::new(c))),
        last_input: default(),
        thread: None,
        flag_to_stop: Arc::new(AtomicBool::new(false)),
    });
}

pub fn simple_startup(mut commands: Commands) {
    let c = simple_loader(5.0);
    startup(&mut commands, c);
}

pub fn print_input(controller: ResMut<Controller>) {
    let i = { *controller.last_input.lock().unwrap() };
    println!("{:?}", i);
}

pub fn update_input(mut controller: ResMut<Controller>) {
    let c = match &controller.controller {
        Some(v) => v,
        None => return,
    };
    if controller.thread.is_none() {
        let t = {
            let c = c.clone();
            let input = controller.last_input.clone();
            let flag = controller.flag_to_stop.clone();

            thread::spawn(move || loop {
                let o = {
                    let mut c = c.lock().unwrap();
                    c.update().unwrap();
                    c.get_typr().unwrap()
                };

                {
                    let mut i = input.lock().unwrap();
                    *i = o;
                }
                if flag.load(std::sync::atomic::Ordering::Relaxed) {
                    return;
                }
            })
        };
        controller.thread = Some(t);
    }
}
