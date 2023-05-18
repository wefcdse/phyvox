mod all_blocks;
pub use all_blocks::*;

mod client;
pub use client::*;

use crate::chunk::BlockVisibility;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlockType {
    None,
    Stone(Stone),
}

impl Default for BlockType {
    fn default() -> Self {
        BlockType::None
    }
}

impl BlockClient for BlockType {
    fn visibility(self) -> BlockVisibility {
        match self {
            BlockType::None => BlockVisibility::Empty,
            BlockType::Stone(s) => s.visibility(),
        }
    }

    fn get_uv(self, face: crate::chunk::BlockFace) -> ((f32, f32), (f32, f32)) {
        match self {
            BlockType::None => ((0., 0.), (0., 0.)),
            BlockType::Stone(s) => s.get_uv(face),
        }
    }
}
