use bevy::prelude::*;

use super::systems::{update_input, update_phy, update_transform};
pub struct DronePlugin;
impl Plugin for DronePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((update_input, update_phy, update_transform));
    }
}
