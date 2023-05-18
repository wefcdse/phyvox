use bevy::prelude::*;

use super::systems::{simple_startup, update_input};
pub struct ControllerPlugin;
impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(simple_startup)
            .add_system(update_input);
        //.add_system(print_input);
    }
}
