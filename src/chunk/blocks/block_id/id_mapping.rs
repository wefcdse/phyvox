use super::BlockType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdMapping {
    pub mapping: Vec<BlockType>,
}

impl Default for IdMapping {
    fn default() -> Self {
        Self {
            mapping: vec![BlockType::None],
        }
    }
}

impl IdMapping {
    pub fn get(&self, id: usize) -> Option<BlockType> {
        match self.mapping.get(id) {
            Some(v) => Some(*v),
            None => None,
        }
    }
}
