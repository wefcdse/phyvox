use bevy::prelude::*;
pub use systems::*;

mod systems;

#[derive(Debug, Clone, Copy, Component, PartialEq, Eq, Hash)]
pub struct ChunkGeneratorPlugin;

#[derive(Debug, Clone, Copy, Component, PartialEq, Eq, Hash)]
pub struct Player;

impl Plugin for ChunkGeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(new_chunks)
            .add_startup_system(startup)
            .add_system(delete_chunks);
    }
}
