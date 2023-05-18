//! A simple 3D scene with light shining over a cube sitting on a plane.

use std::time::Duration;

use bevy::{
    pbr::Cascades,
    prelude::*,
    render::{mesh::Indices, primitives::CascadesFrusta, render_resource::PrimitiveTopology},
};
use bevy_flycam::prelude::*;
use phyvox::{
    controller::plugin::ControllerPlugin,
    drone::{plugin::DronePlugin, Drone},
};
use rand::prelude::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        //.add_plugin(ControllerPlugin)
        // .add_plugin(DronePlugin)
        // .add_plugin(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
        .add_startup_system(setup)
        //.add_system(sleep)
        .add_system(frame_time)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 1.5, 0.0),
        ..default()
    });
    for i in 0..1000 {
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

    commands.spawn(PbrBundle {
        mesh: meshes.add(new_mesh()),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(2.0, 2.0, 0.0),
        ..default()
    });
    // light
    let intensity = 20.0;
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
    dbg!(Mesh::from(shape::Cube { size: 1.0 }));
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
