use bevy::prelude::*;

pub const BASE_GRAVITY: f32 = -250.0;

#[derive(Component, Default, Clone, Debug)]
pub struct Velocity {
    pub velocity: Vec2,
    pub gravity_scalar: f32,
}

pub fn move_objects(mut objs: Query<(&mut Transform, &mut Velocity)>, time: Res<Time>) {
    for (mut transform, mut velocity) in objs.iter_mut() {
        velocity.velocity.y += BASE_GRAVITY * time.delta_seconds();
        transform.translation.x += velocity.velocity.x * time.delta_seconds();
        transform.translation.y += velocity.velocity.y * time.delta_seconds();
    }
}
