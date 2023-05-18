use bevy::prelude::Component;

use super::{
    blocks::{BlockId, QuadGroup},
    Pos,
};

mod iter;
pub use iter::*;

mod generate_quad_group;
pub use generate_quad_group::*;

mod generate_mesh;
pub use generate_mesh::*;

const CHUNK_SIZE: usize = 16_usize;

#[derive(Debug, Component, Default, PartialEq, Eq)]
pub struct Chunk {
    pub base_pos_of_chunk: Pos,
    pub blocks: Box<[[[BlockId; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>,
    pub mesh: Option<()>,
    pub quad_group: Option<QuadGroup>,
}

impl Chunk {
    /// get the block in the chunk
    pub fn get_pos_in_chunk(&self, pos_in_chunk: Pos) -> Option<BlockId> {
        if pos_in_chunk.all_in_range(0..CHUNK_SIZE as i64) {
            Some(
                self.blocks[pos_in_chunk.x as usize][pos_in_chunk.y as usize]
                    [pos_in_chunk.z as usize],
            )
        } else {
            None
        }
    }
}
