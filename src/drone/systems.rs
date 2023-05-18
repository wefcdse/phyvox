use std::f32::consts::PI;

use bevy::prelude::*;

use crate::controller::Controller;

use super::Drone;

pub fn update_input(c: Res<Controller>, mut drone: Query<&mut Drone>) {
    let i = { *c.last_input.lock().unwrap() };
    drone.for_each_mut(|mut d| {
        let d = d.as_mut();
        d.drone.update_input_typr(i);
    });
}

pub fn update_phy(time: Res<Time>, mut drone: Query<&mut Drone>) {
    drone.for_each_mut(|mut d| {
        let d = d.as_mut();
        d.drone.update_phy(time.delta());
    });
}

pub fn update_transform(time: Res<Time>, mut drone: Query<(&Drone, &mut Transform)>) {
    drone.for_each_mut(|(d, mut t)| {
        let t = t.as_mut();
        let r = d.drone.rotation;
        let r = r.to_array();
        let r = [r[0] as f32, r[1] as f32, r[2] as f32, r[3] as f32];
        let r = Quat::from_array(r);
        let up = r.mul_vec3(Vec3::Y);
        let r = Quat::from_axis_angle(up, PI / 2.0).mul_quat(r);

        let side = r.mul_vec3(Vec3::X);
        let r = Quat::from_axis_angle(side, d.deg * PI / 180.0).mul_quat(r);

        t.rotation = r;

        let v = d.drone.velocity;
        let v = v.to_array();
        let v = [v[0] as f32, v[1] as f32, v[2] as f32];
        let v = Vec3::from_array(v);
        // println!("{}", v.length());

        t.translation += v * time.delta_seconds();
    });
}
