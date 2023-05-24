use bevy::{
    prelude::*,
    render::{
        render_resource::{FilterMode, SamplerDescriptor},
        texture::ImageSampler,
    },
};

use crate::chunk::{blocks::IdMapping, chunk::Chunk};

#[derive(Debug, Clone, PartialEq, Eq, Resource)]
pub struct ChunkInfo {
    pub id_mapping: IdMapping,
    pub material: Handle<StandardMaterial>,
}

pub fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    images: ResMut<Assets<Image>>,
) {
    let material = materials.add(StandardMaterial {
        base_color: Color::Rgba {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
            alpha: 1.0,
        },
        base_color_texture: Some(images.get_handle("base_color_texture.png")),
        emissive: Color::Rgba {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
            alpha: 1.0,
        },
        emissive_texture: Some(images.get_handle("emissive_texture.png")),
        perceptual_roughness: 1.0,
        metallic: 1.0,
        metallic_roughness_texture: Some(images.get_handle("metallic_roughness_texture.png")),
        reflectance: 0.5,
        normal_map_texture: Some(images.get_handle("normal_map_texture.png")),
        flip_normal_map_y: false,
        occlusion_texture: Some(images.get_handle("occlusion_texture.png")),
        double_sided: false,
        cull_mode: Some(bevy::render::render_resource::Face::Back),
        unlit: false,
        fog_enabled: true,
        alpha_mode: AlphaMode::Opaque,
        depth_bias: 0.0,
    });
    let m = materials.add(
        Color::Rgba {
            red: 1.,
            green: 0.,
            blue: 1.,
            alpha: 1.0,
        }
        .into(),
    );
    commands.insert_resource(ChunkInfo {
        id_mapping: default(),
        material,
    })
}

pub fn load_png(server: Res<AssetServer>) {
    let _h: Handle<Image> = server.load("base_color_texture.png");
    let _h: Handle<Image> = server.load("emissive_texture.png");
    let _h: Handle<Image> = server.load("metallic_roughness_texture.png");
    let _h: Handle<Image> = server.load("normal_map_texture.png");
    let _h: Handle<Image> = server.load("occlusion_texture.png");
}

pub fn set_png(mut materials: ResMut<Assets<Image>>) {
    for (_, h) in materials.iter_mut() {
        h.sampler_descriptor = ImageSampler::Descriptor(SamplerDescriptor {
            mag_filter: FilterMode::Nearest,
            min_filter: FilterMode::Linear,
            ..default()
        });
    }
}

pub fn generate_quad_group(mut chunks: Query<&mut Chunk, With<Chunk>>, chunk_info: Res<ChunkInfo>) {
    chunks.for_each_mut(|mut x| {
        if x.quad_group.is_none() {
            x.generate_quad_group(&chunk_info.id_mapping);
            //  println!("generate_quad_group for {:?}", &x.base_pos_of_chunk);
        }
    })
}

pub fn generate_mesh(mut chunks: Query<&mut Chunk, With<Chunk>>, chunk_info: Res<ChunkInfo>) {
    chunks.for_each_mut(|mut x| {
        if x.mesh.is_none() {
            x.generate_mesh(&chunk_info.id_mapping);
            //  println!("generate_mesh for {:?}", &x.base_pos_of_chunk);
        }
    })
}

pub fn insert_material(
    chunks: Query<Entity, (With<Chunk>, Without<Handle<StandardMaterial>>)>,
    mut commands: Commands,
    chunk_info: Res<ChunkInfo>,
) {
    chunks.for_each(|e| {
        commands.entity(e).insert(chunk_info.material.clone());
        //  println!("insert_material");
    })
}

pub fn insert_pbr<T: Default + Component>(
    chunks: Query<Entity, (With<Chunk>, Without<T>)>,
    mut commands: Commands,
) {
    chunks.for_each(|e| {
        commands.entity(e).insert(T::default());
        //  println!("insert for {:?}", e);
    })
}

pub fn change_mesh(
    mut chunks: Query<(&mut Chunk, Entity, Option<&Handle<Mesh>>)>,
    mut meshs: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    chunks.for_each_mut(|(mut chunk, e, m)| {
        if !chunk.mesh_up_to_date && chunk.mesh.is_some() {
            if let Some(mesh) = chunk.get_bevy_mesh() {
                if let Some(m) = m {
                    meshs.remove(m);
                }
                chunk.mesh_up_to_date = true;
                if mesh.count_vertices() == 0 {
                    return;
                }
                commands.entity(e).insert(meshs.add(mesh));
                //  println!("change_mesh for {:?}", &chunk.base_pos_of_chunk);
            };
        };
    });
}
