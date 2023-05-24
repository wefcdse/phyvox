use std::ops::Range;

use mlua::{Lua, MetaMethod, ToLua, UserData, Value};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pos {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PosIterator {
    base_pos: Pos,
    now_delta: Pos,
    v: (i64, i64, i64),
    range: Pos,
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
pub enum BlockVisibility {
    Empty,
    Transparent,
    Opaque,
    Special,
}

impl Pos {
    pub fn x(self) -> i64 {
        self.x
    }
    pub fn y(self) -> i64 {
        self.y
    }
    pub fn z(self) -> i64 {
        self.z
    }

    pub fn x_mut(&mut self) -> &mut i64 {
        &mut self.x
    }
    pub fn y_mut(&mut self) -> &mut i64 {
        &mut self.y
    }
    pub fn z_mut(&mut self) -> &mut i64 {
        &mut self.z
    }

    pub fn all_in_range(self, range: Range<i64>) -> bool {
        range.contains(&self.x) && range.contains(&self.y) && range.contains(&self.z)
    }

    pub fn in_range(self, x: Range<i64>, y: Range<i64>, z: Range<i64>) -> bool {
        x.contains(&self.x) && y.contains(&self.y) && z.contains(&self.z)
    }

    pub fn to_f32_truple(self) -> (f32, f32, f32) {
        (self.x as f32, self.y as f32, self.z as f32)
    }

    pub fn from_xyz(x: i64, y: i64, z: i64) -> Pos {
        Pos { x, y, z }
    }

    pub fn iter_cube(self, x: i64, y: i64, z: i64) -> PosIterator {
        PosIterator {
            base_pos: self,
            now_delta: Pos { x: 0, y: 0, z: 0 },
            v: (
                match x {
                    x if x > 0 => 1,
                    x if x < 0 => -1,
                    _ => 0,
                },
                match y {
                    x if x > 0 => 1,
                    x if x < 0 => -1,
                    _ => 0,
                },
                match z {
                    x if x > 0 => 1,
                    x if x < 0 => -1,
                    _ => 0,
                },
            ),
            range: Pos::from_xyz(x.abs(), y.abs(), z.abs()),
        }
    }

    pub fn iter_range(from: Pos, to: Pos) -> PosIterator {
        from.iter_cube(to.x() - from.x(), to.y() - from.y(), to.z() - from.z())
    }
}

impl BlockFace {
    pub fn iter_all() -> impl Iterator<Item = BlockFace> {
        [
            BlockFace::XP,
            BlockFace::XN,
            BlockFace::YP,
            BlockFace::YN,
            BlockFace::ZP,
            BlockFace::ZN,
        ]
        .into_iter()
    }
}

impl Iterator for PosIterator {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        if self.now_delta.z <= self.range.z {
            let mut out_d = self.now_delta;
            out_d.x *= self.v.0;
            out_d.y *= self.v.1;
            out_d.z *= self.v.2;

            self.now_delta.x += 1;

            if self.now_delta.x > self.range.x {
                self.now_delta.x = 0;
                self.now_delta.y += 1;
            }
            if self.now_delta.y > self.range.y {
                self.now_delta.y = 0;
                self.now_delta.z += 1;
            }

            Some(out_d + self.base_pos)
        } else {
            None
        }
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

impl std::ops::Mul<Pos> for i64 {
    type Output = Pos;

    fn mul(self, rhs: Pos) -> Self::Output {
        Pos::from_xyz(self * rhs.x(), self * rhs.y(), self * rhs.z())
    }
}

impl std::ops::Mul<i64> for Pos {
    type Output = Pos;

    fn mul(self, rhs: i64) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Neg for Pos {
    type Output = Pos;

    fn neg(self) -> Self::Output {
        Pos::from_xyz(0, 0, 0) - self
    }
}

impl<T, const X: usize, const Y: usize, const Z: usize> std::ops::Index<Pos> for [[[T; Z]; Y]; X] {
    type Output = T;

    fn index(&self, index: Pos) -> &Self::Output {
        assert!(index.in_range(0_i64..(X as i64), 0_i64..(Y as i64), 0_i64..(Z as i64)));
        &self[index.x() as usize][index.y() as usize][index.z() as usize]
    }
}

impl<T, const X: usize, const Y: usize, const Z: usize> std::ops::IndexMut<Pos>
    for [[[T; Z]; Y]; X]
{
    fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
        assert!(index.in_range(0_i64..(X as i64), 0_i64..(Y as i64), 0_i64..(Z as i64)));
        &mut self[index.x() as usize][index.y() as usize][index.z() as usize]
    }
}

impl UserData for Pos {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_set("x", |_lua, s, value: i64| {
            *s.x_mut() = value;
            Ok(())
        });
        fields.add_field_method_get("x", |_lua, s| Ok(s.x()));

        fields.add_field_method_set("y", |_lua, s, value: i64| {
            *s.y_mut() = value;
            Ok(())
        });
        fields.add_field_method_get("y", |_lua, s| Ok(s.y()));

        fields.add_field_method_set("z", |_lua, s, value: i64| {
            *s.z_mut() = value;
            Ok(())
        });
        fields.add_field_method_get("z", |_lua, s| Ok(s.z()));
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_function(MetaMethod::Add, |_lua, (a, b): (Pos, Pos)| {
            return Ok(a + b);
        });

        methods.add_meta_function(MetaMethod::Sub, |_lua, (a, b): (Pos, Pos)| {
            return Ok(a - b);
        });

        methods.add_meta_function(MetaMethod::Mul, |lua, (a, b): (Value, Value)| {
            enum PosOrI64 {
                Pos(Pos),
                I64(i64),
            }
            let a: PosOrI64 = match a {
                Value::Integer(v) => PosOrI64::I64(v),
                Value::Number(v) => PosOrI64::I64(v as i64),
                Value::UserData(ud) => PosOrI64::Pos(*ud.borrow::<Pos>()?),
                _ => {
                    return Err(mlua::Error::RuntimeError(
                        "cannot mul the Pos value".to_string(),
                    ));
                }
            };

            let b: PosOrI64 = match b {
                Value::Integer(v) => PosOrI64::I64(v),
                Value::Number(v) => PosOrI64::I64(v as i64),
                Value::UserData(ud) => PosOrI64::Pos(*ud.borrow::<Pos>()?),
                _ => {
                    return Err(mlua::Error::RuntimeError(
                        "cannot mul the Pos value".to_string(),
                    ));
                }
            };

            let out: Value = match (a, b) {
                (PosOrI64::Pos(_), PosOrI64::Pos(_)) => {
                    return Err(mlua::Error::RuntimeError(
                        "cannot mul two Pos values together".to_string(),
                    ));
                }
                (PosOrI64::Pos(a), PosOrI64::I64(b)) => (a * b).to_lua(lua)?,
                (PosOrI64::I64(a), PosOrI64::Pos(b)) => (a * b).to_lua(lua)?,
                (PosOrI64::I64(a), PosOrI64::I64(b)) => Value::Integer(a * b),
            };
            Ok(out)
        });

        methods.add_method("clone", |lua, s, _: Value| Ok(s.to_lua(lua)?));
    }
}

#[test]
fn t() {
    let p = Pos { x: 3, y: 4, z: 5 };
    let f = BlockFace::ZN;
    dbg!(p + f);
    dbg!(f + p);
}

#[test]
fn test_pos_iter() {
    let p = Pos::from_xyz(1, 1, 1);
    for a in p.iter_cube(-1, 1, -1).enumerate() {
        dbg!(a);
    }
}

#[test]
fn test_pos_iter_range() {
    let p = Pos::from_xyz(-1, -1, -1);
    let p1 = Pos::from_xyz(1, 1, 1);
    for i in Pos::iter_range(p1, p).enumerate() {
        dbg!(i);
    }
}

#[test]
fn test_mul() {
    let p = Pos::from_xyz(1, 2, 3);
    let p1 = Pos::from_xyz(4, 8, 12);
    assert_eq!(p * 4, p1);
    assert_eq!(4 * p, p1)
}

#[test]
fn test_lua() {
    let lua = Lua::new();
    let globals = lua.globals();
    globals.set("Pos1", Pos::from_xyz(1, 2, 4)).unwrap();
    assert_eq!(
        lua.load(r#"Pos1"#).eval::<Pos>().unwrap(),
        Pos::from_xyz(1, 2, 4)
    );
    assert_eq!(lua.load(r#"Pos1.x"#).eval::<i64>().unwrap(), 1);
    assert_eq!(
        lua.load(
            r#"
            Pos1.x = 32
            return Pos1
        "#
        )
        .eval::<Pos>()
        .unwrap(),
        Pos::from_xyz(32, 2, 4)
    );

    globals.set("Pos2", Pos::from_xyz(1, 2, 4)).unwrap();
    assert_eq!(
        lua.load(r#"Pos2"#).eval::<Pos>().unwrap(),
        Pos::from_xyz(1, 2, 4)
    );
    assert_eq!(lua.load(r#"Pos2.y"#).eval::<i64>().unwrap(), 2);
    assert_eq!(
        lua.load(
            r#"
            Pos2.y = 32
            return Pos2
        "#
        )
        .eval::<Pos>()
        .unwrap(),
        Pos::from_xyz(1, 32, 4)
    );

    globals.set("Pos3", Pos::from_xyz(1, 2, 4)).unwrap();
    assert_eq!(
        lua.load(r#"Pos3"#).eval::<Pos>().unwrap(),
        Pos::from_xyz(1, 2, 4)
    );
    assert_eq!(lua.load(r#"Pos3.z"#).eval::<i64>().unwrap(), 4);
    assert_eq!(
        lua.load(
            r#"
            Pos3.z = 32
            return Pos3
        "#
        )
        .eval::<Pos>()
        .unwrap(),
        Pos::from_xyz(1, 2, 32)
    );

    globals.set("Pos4", Pos::from_xyz(1, 2, 4)).unwrap();
    globals.set("Pos5", Pos::from_xyz(5, 6, 1)).unwrap();
    assert_eq!(
        lua.load(r#"Pos4 + Pos5"#).eval::<Pos>().unwrap(),
        Pos::from_xyz(6, 8, 5)
    );
    assert_eq!(
        lua.load(r#"Pos4 - Pos5"#).eval::<Pos>().unwrap(),
        Pos::from_xyz(-4, -4, 3)
    );

    assert_eq!(
        lua.load(r#"Pos4 * 3"#).eval::<Pos>().unwrap(),
        Pos::from_xyz(3, 6, 12)
    );

    assert_eq!(
        lua.load(r#"3 * Pos4"#).eval::<Pos>().unwrap(),
        Pos::from_xyz(3, 6, 12)
    );

    globals.set("Pos6", Pos::from_xyz(5, 6, 1)).unwrap();
    assert_eq!(
        lua.load(
            r#"
        p1 = Pos6
        Pos6.x = 33
        return p1
        "#
        )
        .eval::<Pos>()
        .unwrap(),
        Pos::from_xyz(33, 6, 1)
    );

    globals.set("Pos7", Pos::from_xyz(5, 6, 1)).unwrap();
    assert_eq!(
        lua.load(
            r#"
        p2 = Pos7:clone()
        Pos7.x = 33
        return p2
        "#
        )
        .eval::<Pos>()
        .unwrap(),
        Pos::from_xyz(5, 6, 1)
    );
}
