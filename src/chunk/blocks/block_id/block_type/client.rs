use crate::chunk::{BlockFace, BlockVisibility};

pub trait BlockClient: std::fmt::Debug + Send + Copy + Clone + Eq + PartialEq {
    fn visibility(self) -> BlockVisibility;
    fn get_uv(self, face: BlockFace) -> ((f32, f32), (f32, f32));
}
