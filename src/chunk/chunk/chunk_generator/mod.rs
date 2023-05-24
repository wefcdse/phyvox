use super::{Chunk, CHUNK_SIZE};
use crate::chunk::{blocks::BlockId, Pos};

pub mod simple_generator;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Seed {
    pub seed: u64,
}

pub trait ChunkGenerator: ChunkGeneratorBasic {
    fn generate_chunk(&self, chunk_base_position: Pos, seed: Seed) -> Chunk {
        let blocks = self.generate_base_blocks(chunk_base_position, seed);
        Chunk {
            blocks,
            base_pos_of_chunk: chunk_base_position,
            ..Default::default()
        }
    }
}

pub trait ChunkGeneratorBasic {
    fn generate_base_blocks(
        &self,
        chunk_base_position: Pos,
        seed: Seed,
    ) -> Box<[[[BlockId; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>;
}

impl<T> ChunkGenerator for T where T: ChunkGeneratorBasic {}
