use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use super::movement::Velocity;

pub fn on_fruit_cut() {}

#[derive(Component, Default, Clone, Debug)]
pub struct Fruit;
