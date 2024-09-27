use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Velocity {
    velocity: Vec2,
    gravity_scalar: f32,
}
