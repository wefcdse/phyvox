use std::fmt::Debug;

use bevy::prelude::Component;

use super::{
    blocks::{BlockId, Mesh, QuadGroup},
    Pos,
};

mod iter;
pub use iter::*;

mod generate_quad_group;
pub use generate_quad_group::*;

mod generate_mesh;
pub use generate_mesh::*;

mod chunk_generator;
pub use chunk_generator::*;

pub const CHUNK_SIZE: usize = 16_usize;

#[derive(Debug, Component, Default, Clone)]
pub struct Chunk {
    pub base_pos_of_chunk: Pos,
    pub blocks: Box<[[[BlockId; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>,
    pub mesh: Option<Mesh>,
    pub quad_group: Option<QuadGroup>,
    pub quad_group_changed: bool,
    pub mesh_up_to_date: bool,
}

impl Chunk {
    /// get the block in the chunk
    pub fn get_pos_in_chunk(&self, pos_in_chunk: Pos) -> Option<BlockId> {
        if pos_in_chunk.all_in_range(0..CHUNK_SIZE as i64) {
            Some({
                self.blocks[pos_in_chunk.x() as usize][pos_in_chunk.y() as usize]
                    [pos_in_chunk.z() as usize]
            })
        } else {
            //return Some(1_u64.into());
            None
        }
    }

    pub fn new_filled_with_id(id: BlockId) -> Chunk {
        Chunk {
            blocks: { Box::from([[[id; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]) },
            ..Default::default()
        }
    }
}
