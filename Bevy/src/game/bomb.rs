use bevy::prelude::*;

pub fn on_bomb_cut() {}

#[derive(Component)]
struct Bomb;

#[derive(Bundle)]
struct BombBundle {
    bomb: Bomb,
    cuttable: Cuttable,
    velocity: 
}
