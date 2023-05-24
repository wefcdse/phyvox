use bevy::prelude::Mesh as BevyMesh;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;

use crate::chunk::blocks::{IdMapping, Mesh};

use super::Chunk;

impl Chunk {
    pub fn generate_mesh(&mut self, id_mapping: &IdMapping) {
        if self.quad_group.is_none() {
            self.generate_quad_group(id_mapping);
        }
        let mesh = {
            if let Some(q) = &self.quad_group {
                q.generate_mesh()
            } else {
                panic!()
            }
        };
        self.mesh = Some(mesh);
    }

    pub fn get_bevy_mesh(&mut self) -> Option<BevyMesh> {
        let mesh: Mesh = match self.mesh.take() {
            Some(m) => m,
            None => {
                return None;
            }
        };
        let mut m = BevyMesh::new(PrimitiveTopology::TriangleList);
        m.insert_attribute(BevyMesh::ATTRIBUTE_POSITION, mesh.vertices);
        m.insert_attribute(BevyMesh::ATTRIBUTE_NORMAL, mesh.normal);
        m.insert_attribute(BevyMesh::ATTRIBUTE_UV_0, mesh.uv);
        m.set_indices(Some(Indices::U32(mesh.indices)));
        Some(m)
    }
}
