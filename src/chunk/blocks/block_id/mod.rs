mod block_type;
pub use block_type::*;
mod id_mapping;
pub use id_mapping::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct BlockId(u64);

impl BlockId {
    pub fn get_block_type(self, id_mapping: &IdMapping) -> Option<BlockType> {
        id_mapping.get(self.0 as usize)
    }
}

impl From<usize> for BlockId {
    fn from(value: usize) -> Self {
        Self(value as u64)
    }
}

impl From<BlockId> for usize {
    fn from(value: BlockId) -> Self {
        value.0 as Self
    }
}

impl From<u64> for BlockId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<BlockId> for u64 {
    fn from(value: BlockId) -> Self {
        value.0
    }
}
