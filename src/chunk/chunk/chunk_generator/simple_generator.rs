use crate::chunk::{blocks::BlockId, chunk::CHUNK_SIZE, Pos};

use super::{ChunkGeneratorBasic, Seed};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SimpleGenerator;

impl ChunkGeneratorBasic for SimpleGenerator {
    fn generate_base_blocks(
        &self,
        chunk_base_position: Pos,
        seed: Seed,
    ) -> Box<[[[BlockId; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]> {
        let mut b = Box::new([[[BlockId::from(0_u64); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]);

        let h: i64 = seed.seed as i64;

        if chunk_base_position.y() > h {
            b
        } else {
            let max_y = (h - chunk_base_position.y()).min(CHUNK_SIZE as i64 - 1);
            for p in Pos::default().iter_cube(CHUNK_SIZE as i64 - 1, max_y, CHUNK_SIZE as i64 - 1) {
                b[p] = 1_u64.into();
            }
            b
        }
    }
}
