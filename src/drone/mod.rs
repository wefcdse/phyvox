use bevy::prelude::*;
pub use rc_controller::simple_loader::simple_loader;
use rc_controller::{self};
#[derive(Component)]
pub struct Drone {
    pub drone: rc_controller::drone::Quadrotor,
    pub deg: f32,
}
#[derive(Bundle)]
pub struct DroneBundle {
    pub d: Drone,
}

impl Drone {
    pub fn new() -> Self {
        Self {
            drone: default(),
            deg: 30.0,
        }
    }
}

pub mod plugin;
pub mod systems;
