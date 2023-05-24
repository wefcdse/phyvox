use std::f32::consts::PI;

use bevy::{
    prelude::*,
    render::{
        render_resource::{FilterMode, SamplerDescriptor},
        texture::ImageSampler,
    },
};

pub struct TestPlugin;

#[derive(Debug, Clone, Component)]
pub struct C {
    h1: Handle<Image>,
}

pub fn load_png(mut commands: Commands, server: Res<AssetServer>) {
    let h: Handle<Image> = server.load("a.png");
    let h: Handle<Image> = server.load("b.png");
    let h: Handle<Image> = server.load("c.png");
    let h: Handle<Image> = server.load("d.png");
    let h: Handle<Image> = server.load("e.png");
    commands.spawn_empty().insert(C { h1: h });
}

pub fn get_png(mut materials: ResMut<Assets<Image>>) {
    let h = materials.get_handle("a.png");
    let h = match materials.get_mut(&h) {
        Some(v) => v,
        None => return,
    };
    h.sampler_descriptor = ImageSampler::Descriptor(SamplerDescriptor {
        mag_filter: FilterMode::Nearest,
        min_filter: FilterMode::Nearest,
        ..default()
    });

    for (_, h) in materials.iter_mut() {
        h.sampler_descriptor = ImageSampler::Descriptor(SamplerDescriptor {
            mag_filter: FilterMode::Nearest,
            min_filter: FilterMode::Nearest,
            ..default()
        });
    }

    //dbg!(h.size());
    //dbg!(h);
}

pub fn change_light(time: Res<Time>, mut light: Query<(&mut Transform, With<DirectionalLight>)>) {
    //return;
    for (mut t, _) in light.iter_mut() {
        t.rotate_axis(Vec3::X, time.delta_seconds() * 2. * PI / 20.);
    }
}

impl Plugin for TestPlugin {
    fn build(&self, app: &mut App) {
        app //.add_startup_system(load_png)
            // .add_system(get_png)
            .add_system(change_light);
    }
}
