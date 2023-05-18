use crate::chunk::{blocks::BlockId, chunk::CHUNK_SIZE, Pos};

use super::Chunk;

#[derive(Debug, PartialEq, Eq)]
pub struct ChunkIter<'a> {
    chunk: &'a Chunk,
    pos_in_chunk: Pos,
}

impl Chunk {
    pub fn iter(&self) -> ChunkIter {
        ChunkIter {
            chunk: self,
            pos_in_chunk: Pos { x: 0, y: 0, z: 0 },
        }
    }
}

impl Iterator for ChunkIter<'_> {
    type Item = (Pos, BlockId);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos_in_chunk.all_in_range(0..CHUNK_SIZE as i64) {
            let out_id = self.chunk.get_pos_in_chunk(self.pos_in_chunk).unwrap();
            let out_pos = self.pos_in_chunk;
            let _out_pos_world = self.chunk.base_pos_of_chunk + self.pos_in_chunk;

            self.pos_in_chunk.x += 1;

            if self.pos_in_chunk.x >= CHUNK_SIZE as i64 {
                self.pos_in_chunk.x = 0;
                self.pos_in_chunk.y += 1;
            }

            if self.pos_in_chunk.y >= CHUNK_SIZE as i64 {
                self.pos_in_chunk.y = 0;
                self.pos_in_chunk.z += 1;
            }

            Some((out_pos, out_id))
        } else {
            None
        }
    }
}

#[test]
pub fn a() {
    let mut c = Chunk::default();
    let mut id: u64 = 0;
    for z in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                c.blocks[x][y][z] = id.into();
                id += 1;
            }
        }
    }
    for i in c.iter() {
        println!("{:?}", i)
    }
}
