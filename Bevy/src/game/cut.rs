use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    transform::commands,
};

use super::movement::Velocity;

pub fn detect_cut() {}

#[derive(Component, Default, Clone, Debug)]
pub struct Cuttable {
    pub radius: f32,
    pub time_score: f32,
}

#[derive(Event, Clone, Debug)]
pub struct CutEvent {
    pub target: Entity,
    pub cuttable: Cuttable,
}

impl CutEvent {
    pub fn new(target: Entity, cuttable: Cuttable) -> Self {
        Self { target, cuttable }
    }
}

#[derive(Bundle, Default, Clone)]
pub struct CuttableBundle {
    cuttable: Cuttable,
    sprite: MaterialMesh2dBundle<ColorMaterial>,
    velocity: Velocity,
}

impl CuttableBundle {
    pub fn new(
        position: Vec3,
        velocity: Velocity,
        radius: f32,
        mesh: Mesh2dHandle,
        material: Handle<ColorMaterial>,
        time_score: f32,
    ) -> CuttableBundle {
        CuttableBundle {
            cuttable: Cuttable { radius, time_score },
            sprite: MaterialMesh2dBundle {
                mesh,
                material,
                transform: Transform::from_translation(position).with_scale(Vec3::ONE * radius),
                ..Default::default()
            },
            velocity,
        }
    }
}

#[derive(Component)]
pub struct IsCutting {
    pub enter_position: Vec2,
}

impl IsCutting {
    pub fn new(enter_position: Vec2) -> Self {
        IsCutting { enter_position }
    }
}

pub fn check_despawn(mut commands: Commands, cuttables: Query<(&mut Transform, Entity)>) {
    for (transform, entity) in cuttables.iter() {
        if transform.translation.y < -50.0 {
            commands.entity(entity).despawn();
        }
    }
}
