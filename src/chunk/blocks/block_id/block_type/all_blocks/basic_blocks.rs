use crate::chunk::{blocks::block_id::block_type::BlockClient, BlockVisibility};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Stone {}

impl BlockClient for Stone {
    fn visibility(self) -> BlockVisibility {
        BlockVisibility::Opaque
    }

    fn get_uv(self, _face: crate::chunk::BlockFace) -> ((f32, f32), (f32, f32)) {
        ((0., 0.), (1., 1.))
    }
}
