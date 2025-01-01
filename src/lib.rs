use bevy::prelude::*;

pub mod plugins;

#[derive(Component)]
pub struct BodyKinematics {
    pub start_pos: Vec3,
    pub speed: f32,
}
