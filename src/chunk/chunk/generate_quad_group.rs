use super::Chunk;
use crate::chunk::{
    blocks::{BlockClient, IdMapping, Quad, QuadGroup},
    BlockFace, BlockVisibility,
};

impl Chunk {
    /// generate the QuadGroup
    ///
    pub fn generate_quad_group(&mut self, id_mapping: &IdMapping) {
        let mut quads = QuadGroup::default();
        for (pos, block) in self.iter() {
            //dbg!(pos);
            let block = match block.get_block_type(id_mapping) {
                Some(v) => v,
                None => continue,
            };
            match block.visibility() {
                BlockVisibility::Opaque => {}
                _ => continue,
            };
            for face in BlockFace::iter_all() {
                // dbg!(face);
                let neighber_visibility = match self.get_pos_in_chunk(pos + face) {
                    Some(v) => match v.get_block_type(id_mapping) {
                        Some(v) => v.visibility(),
                        None => BlockVisibility::Empty,
                    },
                    None => BlockVisibility::Empty,
                };
                match neighber_visibility {
                    BlockVisibility::Opaque => {}
                    _ => {
                        quads.insert_quad(Quad::new(pos, block, face));
                    }
                };
            }
        }
        self.quad_group = Some(quads);
        self.mesh_up_to_date = false;
    }

    pub fn get_quad_group(&self) -> Option<&QuadGroup> {
        match &self.quad_group {
            Some(q) => Some(q),
            None => None,
        }
    }

    pub fn get_quad_group_and_generate(&mut self, id_mapping: &IdMapping) -> &QuadGroup {
        if self.get_quad_group().is_none() {
            self.generate_quad_group(id_mapping);
        }
        self.get_quad_group().unwrap()
    }
}
