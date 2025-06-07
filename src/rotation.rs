use bevy::prelude::ops::{cos, sin};
use bevy::prelude::*;

pub fn rad_to_vec2(rad: f32) -> Vec2 {
    Vec2::new(cos(rad), sin(rad))
}

pub fn rad_to_quat(rad: f32) -> Quat {
    Quat::from_rotation_z(rad)
}

pub fn vec2_to_quat(vec2: Vec2) -> Quat {
    Quat::from_euler(EulerRot::XYZ, vec2.x, vec2.y, 0.)
}

pub fn quat_to_vec3(quat: Quat) -> Vec3 {
    //Vec3::from(quat.to_euler(EulerRot::XYZ)).normalize()
    quat.mul_vec3(Vec3::X).normalize()
}

pub fn quat_to_vec2(quat: Quat) -> Vec2 {
    quat_to_vec3(quat).xy()
}
