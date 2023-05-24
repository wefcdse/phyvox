use bevy::prelude::*;

mod systems;
pub use systems::*;

#[derive(Debug, Clone, Copy, Component, PartialEq, Eq, Hash)]
pub struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(generate_quad_group)
            .add_system(generate_mesh)
            .add_system(insert_material)
            .add_system(insert_pbr::<Visibility>)
            //.add_system(insert_pbr::<GlobalTransform>)
            .add_system(insert_pbr::<ComputedVisibility>)
            .add_system(change_mesh)
            .add_startup_systems((setup, load_png))
            .add_system(set_png);
    }
}
