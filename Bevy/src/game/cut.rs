use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use super::movement::Velocity;

pub fn detect_cut() {}

#[derive(Component, Default, Clone, Debug)]
pub struct Cuttable {
    pub radius: f32,
}

#[derive(Event, Clone, Debug)]
pub struct CutEvent {
    pub target: Entity,
}

impl CutEvent {
    pub fn new(target: Entity) -> Self {
        Self { target }
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
    ) -> CuttableBundle {
        CuttableBundle {
            cuttable: Cuttable { radius },
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
