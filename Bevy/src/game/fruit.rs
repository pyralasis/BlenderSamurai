use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use super::{
    cut::{CutEvent, Cuttable},
    movement::Velocity,
    GameState,
};

pub fn on_fruit_cut(
    mut commands: Commands,
    mut cut_event: EventReader<CutEvent>,
    fruits: Query<(), With<Fruit>>,
    mut game_state: ResMut<GameState>,
) {
    for evt in cut_event.read() {
        if fruits.contains(evt.target) {
            commands.entity(evt.target).despawn();
            game_state.add_blend_time(evt.cuttable.time_score);
        }
    }
}

#[derive(Component, Default, Clone, Debug)]
pub struct Fruit;
