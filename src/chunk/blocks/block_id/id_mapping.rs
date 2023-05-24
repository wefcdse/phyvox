use bevy::prelude::Resource;

use super::{BlockType, Stone};

#[derive(Debug, Clone, PartialEq, Eq, Resource)]
pub struct IdMapping {
    pub mapping: Vec<BlockType>,
}

impl IdMapping {
    pub fn get(&self, id: usize) -> Option<BlockType> {
        match self.mapping.get(id) {
            Some(v) => Some(*v),
            None => None,
        }
    }
}

impl Default for IdMapping {
    fn default() -> Self {
        Self {
            mapping: vec![BlockType::None, BlockType::Stone(Stone)],
        }
    }
}
