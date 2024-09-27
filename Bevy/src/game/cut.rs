use bevy::prelude::*;

pub fn detect_cut() {}

#[derive(Component)]
struct Cuttable {}

#[derive(Event)]
struct CutEvent {}

#[derive(Bundle)]
struct CuttableBundle {
    cuttable: Cuttable,
    velocity: Velocity,
}
