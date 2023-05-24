//! A simple 3D scene with light shining over a cube sitting on a plane.

use std::{
    hint::black_box,
    time::{Duration, Instant},
};

use bevy::{
    pbr::Cascades,
    prelude::*,
    reflect::GetPath,
    render::{mesh::Indices, primitives::CascadesFrusta, render_resource::PrimitiveTopology},
};
use bevy_flycam::prelude::*;
use phyvox::{
    chunk::{
        blocks::IdMapping,
        chunk::{simple_generator::SimpleGenerator, Chunk, ChunkGenerator, Seed},
        generator_plugin::ChunkGeneratorPlugin,
        Pos,
    },
    controller::plugin::ControllerPlugin,
    drone::{plugin::DronePlugin, Drone},
    systems::TestPlugin,
};
use rand::prelude::*;

fn main() {
    run_with_time(|| {
        for _ in 0..5000 {
            //let mut c = Chunk::new_filled_with_id(1_u64.into());
            let mut c = Chunk::default();
            let id_mapping = IdMapping::default();
            c.blocks[0][0][0] = 1_u64.into();
            c.generate_quad_group(&id_mapping);
            c.generate_mesh(&id_mapping);
            black_box(c.get_bevy_mesh().unwrap());
        }
    });
    //return;

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(phyvox::chunk::plugin::ChunkPlugin)
        //.add_plugin(ControllerPlugin)
        // .add_plugin(DronePlugin)
        .add_plugin(TestPlugin)
        .add_plugin(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
        .add_plugin(ChunkGeneratorPlugin)
        .add_startup_system(setup)
        //.add_system(sleep)
        //.add_system(frame_time)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    images: ResMut<Assets<Image>>,
    server: Res<AssetServer>,
) {
    let mut r = rand::thread_rng();

    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(500.0).into()),
        material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(dbg!(Color::rgba(0., 0., 0., 0.2).into())),
        transform: Transform::from_xyz(0.0, 3.0, 0.0),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(1., 0., 0.).into()),
        transform: Transform::from_xyz(0.0 + 3., 3., 0.0),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0., 1., 0.).into()),
        transform: Transform::from_xyz(0.0, 3. + 3., 0.0),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0., 0., 1.).into()),
        transform: Transform::from_xyz(0.0, 3., 0.0 + 3.),
        ..default()
    });

    for i in 0..0 {
        let x: f32 = r.gen::<f32>() * 40.0 - 20.0;
        let y: f32 = r.gen::<f32>() * 40.0;
        let z: f32 = r.gen::<f32>() * 40.0 - 20.0;
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0 + x, y, 0.0 + z),
            ..default()
        });
    }
    if false {
        commands
            .spawn::<Chunk>({
                //let mut c = Chunk::new_filled_with_id(1_u64.into());
                let mut c =
                    SimpleGenerator.generate_chunk(Pos::from_xyz(0, 0, 0), Seed { seed: 0 });
                c.blocks[0][0][0] = 0_u64.into();
                c.blocks[Pos::from_xyz(5, 0, 4)] = 0_u64.into();
                c.blocks[Pos::from_xyz(8, 1, 9)] = 1_u64.into();
                c
            })
            .insert(Transform::from_xyz(3.0, 3.0, 3.0));
    }
    if false {
        commands.spawn(PbrBundle {
            mesh: {
                let mut m = meshes.add(run_with_time(|| {
                    //let mut c = Chunk::new_filled_with_id(1_u64.into());
                    let mut c =
                        SimpleGenerator.generate_chunk(Pos::from_xyz(0, 0, 0), Seed { seed: 0 });
                    let id_mapping = run_with_time(|| IdMapping::default());
                    c.blocks[0][0][0] = 0_u64.into();
                    c.blocks[Pos::from_xyz(5, 0, 4)] = 0_u64.into();
                    c.blocks[Pos::from_xyz(8, 1, 9)] = 1_u64.into();
                    run_with_time(|| c.generate_quad_group(&id_mapping));
                    run_with_time(|| c.generate_mesh(&id_mapping));
                    run_with_time(|| c.get_bevy_mesh().unwrap())
                }));
                dbg!(server.get_handle_path(&m));
                dbg!(server.get_handle_path(images.get_handle("a.png")));
                m
            },
            material: materials.add({
                dbg!(StandardMaterial {
                    base_color: Color::Rgba {
                        red: 1.0,
                        green: 1.0,
                        blue: 1.0,
                        alpha: 1.0,
                    },
                    base_color_texture: Some(images.get_handle("a.png")),
                    emissive: Color::Rgba {
                        red: 1.0,
                        green: 1.0,
                        blue: 1.0,
                        alpha: 1.0
                    },
                    emissive_texture: Some(images.get_handle("b.png")),
                    perceptual_roughness: 0.5,
                    metallic: 1.0,
                    metallic_roughness_texture: Some(images.get_handle("c.png")),
                    reflectance: 0.5,
                    normal_map_texture: Some(images.get_handle("d.png")),
                    flip_normal_map_y: false,
                    occlusion_texture: Some(images.get_handle("e.png")),
                    double_sided: false,
                    cull_mode: Some(bevy::render::render_resource::Face::Back),
                    unlit: false,
                    fog_enabled: true,
                    alpha_mode: AlphaMode::Opaque,
                    depth_bias: 0.0,
                })
            }),
            transform: Transform::from_xyz(3.0, 3.0, 3.0),
            ..default()
        });
    }
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
        material: materials.add(Color::rgb(0., 0., 0.).into()),
        transform: Transform::from_xyz(3.0, 3.0, 3.0),
        ..default()
    });

    // light
    let intensity = 0.0;
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: intensity,
            shadows_enabled: true,
            color: Color::rgb(1.0, 0.0, 0.0),
            ..default()
        },
        transform: Transform::from_xyz(4.0 + 0.5, 8.0, 4.0),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: intensity,
            shadows_enabled: true,
            color: Color::rgb(0.0, 1.0, 0.0),
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0 + 0.5),
        ..default()
    });

    for _ in 1..1 {
        commands.spawn(PointLightBundle {
            point_light: PointLight {
                intensity: intensity,
                shadows_enabled: false,
                color: Color::rgb(0.0, 1.0, 0.0),
                ..default()
            },
            transform: Transform::from_xyz(
                4.0 + r.gen_range(-10.0..10.0),
                8.0 + r.gen_range(-1.0..1.0),
                4.0 + r.gen_range(-10.0..10.0),
            ),
            ..default()
        });
    }

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: intensity,
            shadows_enabled: true,
            color: Color::rgb(0.0, 0.0, 1.0),
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands
        .spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                color: Color::rgb(1.0, 1.0, 1.0),
                illuminance: 100000.0f32,
                shadows_enabled: true,
                ..default()
            },
            ..default()
        })
        .insert({
            let mut t = Transform::from_xyz(1.0, 1.0, 1.0);
            let a =
                (Quat::from_axis_angle(Vec3::X.normalize(), 45.0 / -180.0 * std::f32::consts::PI));

            t.rotate(a);
            let a =
                (Quat::from_axis_angle(Vec3::Y.normalize(), 45.0 / -180.0 * std::f32::consts::PI));
            t.rotate(a);
            t
        });
    // dbg!(Mesh::from(shape::Cube { size: 1.0 }));
    return;
    // camera
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert({
            let mut d = Drone::new();
            d.drone.g.y = -9.8;
            d.drone.motor_max_force = 50.0;
            d
        });
}

fn new_mesh() -> Mesh {
    let vertices_pos_normal_uv = &[
        // Front
        ([0.0f32, 0.0f32, 0.0f32], [0., 1., 0.], [0., 0.]),
        ([0.0f32, 0.0f32, 1.0f32], [0., 1., 0.], [0., 1.0]),
        ([1.0f32, 0.0f32, 0.0f32], [0., 1., 0.], [1.0, 0.]),
    ];

    let positions: Vec<_> = vertices_pos_normal_uv
        .iter()
        .map(|(p, ..)| {
            let m = 2.0;
            let o = [p[0] * m, p[1] * m, p[2] * m];
            o
        })
        .collect();
    let normals: Vec<_> = vertices_pos_normal_uv.iter().map(|(_, n, _)| *n).collect();
    let uvs: Vec<_> = vertices_pos_normal_uv.iter().map(|(.., uv)| *uv).collect();

    let indices = Indices::U32(vec![
        0, 1, 2, // front
    ]);

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(indices));
    mesh
}

fn sleep() {
    std::thread::sleep(Duration::from_secs_f64(0.0));
}

fn frame_time(time: Res<Time>) {
    println!("{}", time.delta_seconds())
}

#[test]
fn t() {
    let mut c = run_with_time(|| Chunk::new_filled_with_id(1_u64.into()));
    //let mut c = run_with_time(|| Chunk::default());
    let id_mapping = run_with_time(|| IdMapping::default());
    run_with_time(|| c.generate_quad_group(&id_mapping));
    run_with_time(|| c.generate_mesh(&id_mapping));
    run_with_time(|| c.get_bevy_mesh().unwrap());
    //c.get_bevy_mesh().unwrap();
}

fn run_with_time<F, T>(f: F) -> T
where
    F: FnOnce() -> T,
{
    let t = Instant::now();
    let o = f();
    dbg!(t.elapsed());
    o
}
