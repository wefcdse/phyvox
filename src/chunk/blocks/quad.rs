//use ahash::AHashMap;
use rustc_hash::FxHashMap;
//use std::collections::HashMap;

use crate::chunk::{blocks::BlockClient, BlockFace, Pos};

use super::block_id::BlockType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Quad {
    pub pos: Pos,
    pub block_type: BlockType,
    pub face: BlockFace,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct QuadGroup {
    quad_group: [FxHashMap<Pos, BlockType>; 6], // x+,x-,y+,y-,z+,z-
}
#[derive(Debug, Clone, Default)]
pub struct Mesh {
    pub vertices: Vec<[f32; 3]>,
    pub normal: Vec<[f32; 3]>,
    pub uv: Vec<[f32; 2]>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn new_with_capacity(v: usize, i: usize) -> Mesh {
        Mesh {
            vertices: Vec::with_capacity(v),
            normal: Vec::with_capacity(v),
            uv: Vec::with_capacity(v),
            indices: Vec::with_capacity(i),
        }
    }
}

impl Quad {
    pub fn new(pos: Pos, block_type: BlockType, face: BlockFace) -> Quad {
        Quad {
            pos,
            block_type,
            face,
        }
    }

    /// ([(vertice, normal, uv); 4], [indices; 6])
    pub fn generate_mesh(
        &self,
    ) -> (
        [((f32, f32, f32), (f32, f32, f32), (f32, f32)); 4],
        [u32; 6],
    ) {
        match self.face {
            BlockFace::XP => {
                let v0 = (self.pos + Pos::from_xyz(1, 0, 0)).to_f32_truple();
                let v1 = (self.pos + Pos::from_xyz(1, 1, 0)).to_f32_truple();
                let v2 = (self.pos + Pos::from_xyz(1, 0, 1)).to_f32_truple();
                let v3 = (self.pos + Pos::from_xyz(1, 1, 1)).to_f32_truple();

                let normal = (1., 0., 0.);

                let uv = self.block_type.get_uv(self.face);
                let uv0 = uv.0;
                let uv1 = (uv.0 .0, uv.1 .1);
                let uv2 = (uv.1 .0, uv.0 .1);
                let uv3 = uv.1;

                let indices: [u32; 6] = [0, 1, 2, 1, 3, 2];
                (
                    [
                        (v0, normal, uv0),
                        (v1, normal, uv1),
                        (v2, normal, uv2),
                        (v3, normal, uv3),
                    ],
                    indices,
                )
            }
            BlockFace::XN => {
                let v0 = (self.pos + Pos::from_xyz(0, 0, 0)).to_f32_truple();
                let v1 = (self.pos + Pos::from_xyz(0, 1, 0)).to_f32_truple();
                let v2 = (self.pos + Pos::from_xyz(0, 0, 1)).to_f32_truple();
                let v3 = (self.pos + Pos::from_xyz(0, 1, 1)).to_f32_truple();

                let normal = (-1., 0., 0.);

                let uv = self.block_type.get_uv(self.face);
                let uv0 = uv.0;
                let uv1 = (uv.0 .0, uv.1 .1);
                let uv2: (f32, f32) = (uv.1 .0, uv.0 .1);
                let uv3 = uv.1;

                let indices: [u32; 6] = [0, 2, 1, 1, 2, 3];
                (
                    [
                        (v0, normal, uv0),
                        (v1, normal, uv2),
                        (v2, normal, uv1),
                        (v3, normal, uv3),
                    ],
                    indices,
                )
            }
            BlockFace::YP => {
                let v0 = (self.pos + Pos::from_xyz(0, 1, 0)).to_f32_truple();
                let v1 = (self.pos + Pos::from_xyz(1, 1, 0)).to_f32_truple();
                let v2 = (self.pos + Pos::from_xyz(0, 1, 1)).to_f32_truple();
                let v3 = (self.pos + Pos::from_xyz(1, 1, 1)).to_f32_truple();

                let normal = (0., 1., 0.);

                let uv = self.block_type.get_uv(self.face);
                let uv0 = uv.0;
                let uv1 = (uv.0 .0, uv.1 .1);
                let uv2 = (uv.1 .0, uv.0 .1);
                let uv3 = uv.1;

                let indices: [u32; 6] = [0, 2, 1, 1, 2, 3];
                (
                    [
                        (v0, normal, uv0),
                        (v1, normal, uv2),
                        (v2, normal, uv1),
                        (v3, normal, uv3),
                    ],
                    indices,
                )
            }
            BlockFace::YN => {
                let v0 = (self.pos + Pos::from_xyz(0, 0, 0)).to_f32_truple();
                let v1 = (self.pos + Pos::from_xyz(1, 0, 0)).to_f32_truple();
                let v2 = (self.pos + Pos::from_xyz(0, 0, 1)).to_f32_truple();
                let v3 = (self.pos + Pos::from_xyz(1, 0, 1)).to_f32_truple();

                let normal = (0., -1., 0.);

                let uv = self.block_type.get_uv(self.face);
                let uv0 = uv.0;
                let uv1 = (uv.0 .0, uv.1 .1);
                let uv2 = (uv.1 .0, uv.0 .1);
                let uv3 = uv.1;

                let indices: [u32; 6] = [0, 1, 2, 1, 3, 2];
                (
                    [
                        (v0, normal, uv0),
                        (v1, normal, uv1),
                        (v2, normal, uv2),
                        (v3, normal, uv3),
                    ],
                    indices,
                )
            }
            BlockFace::ZP => {
                let v0 = (self.pos + Pos::from_xyz(0, 0, 1)).to_f32_truple();
                let v1 = (self.pos + Pos::from_xyz(1, 0, 1)).to_f32_truple();
                let v2 = (self.pos + Pos::from_xyz(0, 1, 1)).to_f32_truple();
                let v3 = (self.pos + Pos::from_xyz(1, 1, 1)).to_f32_truple();

                let normal = (0., 0., 1.);

                let uv = self.block_type.get_uv(self.face);
                let uv0 = uv.0;
                let uv1 = (uv.0 .0, uv.1 .1);
                let uv2 = (uv.1 .0, uv.0 .1);
                let uv3 = uv.1;

                let indices: [u32; 6] = [0, 1, 2, 1, 3, 2];
                (
                    [
                        (v0, normal, uv0),
                        (v1, normal, uv1),
                        (v2, normal, uv2),
                        (v3, normal, uv3),
                    ],
                    indices,
                )
            }
            BlockFace::ZN => {
                let v0 = (self.pos + Pos::from_xyz(0, 0, 0)).to_f32_truple();
                let v1 = (self.pos + Pos::from_xyz(1, 0, 0)).to_f32_truple();
                let v2 = (self.pos + Pos::from_xyz(0, 1, 0)).to_f32_truple();
                let v3 = (self.pos + Pos::from_xyz(1, 1, 0)).to_f32_truple();

                let normal = (0., 0., 1.);

                let uv = self.block_type.get_uv(self.face);
                let uv0 = uv.0;
                let uv1 = (uv.0 .0, uv.1 .1);
                let uv2 = (uv.1 .0, uv.0 .1);
                let uv3 = uv.1;

                let indices: [u32; 6] = [0, 2, 1, 1, 2, 3];
                (
                    [
                        (v0, normal, uv0),
                        (v1, normal, uv2),
                        (v2, normal, uv1),
                        (v3, normal, uv3),
                    ],
                    indices,
                )
            }
        }
    }
}

impl QuadGroup {
    /// insert a quad into quad group
    ///
    /// returns the previously value
    pub fn insert_quad(&mut self, quad: Quad) -> Option<BlockType> {
        match quad.face {
            BlockFace::XP => self.quad_group[0].insert(quad.pos, quad.block_type),
            BlockFace::XN => self.quad_group[1].insert(quad.pos, quad.block_type),
            BlockFace::YP => self.quad_group[2].insert(quad.pos, quad.block_type),
            BlockFace::YN => self.quad_group[3].insert(quad.pos, quad.block_type),
            BlockFace::ZP => self.quad_group[4].insert(quad.pos, quad.block_type),
            BlockFace::ZN => self.quad_group[5].insert(quad.pos, quad.block_type),
        }
    }

    /// returns if the group contains this value
    pub fn contains(&mut self, quad: Quad) -> bool {
        match quad.face {
            BlockFace::XP => self.quad_group[0].contains_key(&quad.pos),
            BlockFace::XN => self.quad_group[1].contains_key(&quad.pos),
            BlockFace::YP => self.quad_group[2].contains_key(&quad.pos),
            BlockFace::YN => self.quad_group[3].contains_key(&quad.pos),
            BlockFace::ZP => self.quad_group[4].contains_key(&quad.pos),
            BlockFace::ZN => self.quad_group[5].contains_key(&quad.pos),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = Quad> + '_ {
        self.quad_group[0]
            .iter()
            .map(|(p, t)| Quad::new(*p, *t, BlockFace::XP))
            .chain(
                self.quad_group[1]
                    .iter()
                    .map(|(p, t)| Quad::new(*p, *t, BlockFace::XN)),
            )
            .chain(
                self.quad_group[2]
                    .iter()
                    .map(|(p, t)| Quad::new(*p, *t, BlockFace::YP)),
            )
            .chain(
                self.quad_group[3]
                    .iter()
                    .map(|(p, t)| Quad::new(*p, *t, BlockFace::YN)),
            )
            .chain(
                self.quad_group[4]
                    .iter()
                    .map(|(p, t)| Quad::new(*p, *t, BlockFace::ZP)),
            )
            .chain(
                self.quad_group[5]
                    .iter()
                    .map(|(p, t)| Quad::new(*p, *t, BlockFace::ZN)),
            )
    }

    pub fn generate_mesh(&self) -> Mesh {
        let mut mesh = Mesh::new_with_capacity(8192, 8192);
        let mut indices_base = 0;
        for i in self.iter() {
            let (p, i) = i.generate_mesh();
            for indices in i {
                mesh.indices.push(indices + indices_base);
            }
            for (vertice, normal, uv) in p {
                mesh.vertices.push([vertice.0, vertice.1, vertice.2]);
                mesh.normal.push([normal.0, normal.1, normal.2]);
                mesh.uv.push([uv.0, uv.1]);
                indices_base += 1;
            }
        }
        mesh
    }

    // pub fn update_block()
}
