use std::ops::Range;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlockFace {
    XP,
    XN,
    YP,
    YN,
    ZP,
    ZN,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockFaceIter(BlockFace);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlockVisibility {
    Empty,
    Transparent,
    Opaque,
    Special,
}

impl Pos {
    pub fn all_in_range(self, range: Range<i64>) -> bool {
        range.contains(&self.x) && range.contains(&self.y) && range.contains(&self.z)
    }

    pub fn to_f32_truple(self) -> (f32, f32, f32) {
        (self.x as f32, self.y as f32, self.z as f32)
    }

    pub fn from_xyz(x: i64, y: i64, z: i64) -> Pos {
        Pos { x, y, z }
    }
}

impl std::ops::Add<BlockFace> for Pos {
    type Output = Pos;

    fn add(self, rhs: BlockFace) -> Self::Output {
        let mut s = self;
        match rhs {
            BlockFace::XP => s.x += 1,
            BlockFace::XN => s.x -= 1,
            BlockFace::YP => s.y += 1,
            BlockFace::YN => s.y -= 1,
            BlockFace::ZP => s.z += 1,
            BlockFace::ZN => s.z -= 1,
        }
        s
    }
}

impl std::ops::Add<Pos> for BlockFace {
    type Output = Pos;

    fn add(self, rhs: Pos) -> Self::Output {
        rhs + self
    }
}

impl std::ops::Add<Pos> for Pos {
    type Output = Pos;
    fn add(self, rhs: Pos) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub<Pos> for Pos {
    type Output = Pos;
    fn sub(self, rhs: Pos) -> Self::Output {
        Pos {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::AddAssign<Pos> for Pos {
    fn add_assign(&mut self, rhs: Pos) {
        *self = *self + rhs;
    }
}

impl std::ops::SubAssign<Pos> for Pos {
    fn sub_assign(&mut self, rhs: Pos) {
        *self = *self - rhs;
    }
}

impl Iterator for BlockFaceIter {
    type Item = BlockFace;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            BlockFace::XP => Some(BlockFace::XN),
            BlockFace::XN => Some(BlockFace::YP),
            BlockFace::YP => Some(BlockFace::YN),
            BlockFace::YN => Some(BlockFace::ZP),
            BlockFace::ZP => Some(BlockFace::ZN),
            BlockFace::ZN => None,
        }
    }
}

impl BlockFace {
    pub fn iter_all() -> BlockFaceIter {
        BlockFaceIter(BlockFace::XP)
    }
}

#[test]
fn t() {
    let p = Pos { x: 3, y: 4, z: 5 };
    let f = BlockFace::ZN;
    dbg!(p + f);
    dbg!(f + p);
}
