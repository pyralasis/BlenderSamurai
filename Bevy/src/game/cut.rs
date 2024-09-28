use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use super::movement::Velocity;

pub fn detect_cut() {}

#[derive(Component, Default, Clone, Debug)]
pub struct Cuttable {}

#[derive(Event, Default, Clone, Debug)]
pub struct CutEvent {}

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
            cuttable: Cuttable {},
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
