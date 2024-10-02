use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use super::{cut::CutEvent, movement::Velocity, GameState};

pub fn on_fruit_cut(
    mut commands: Commands,
    mut cut_event: EventReader<CutEvent>,
    bombs: Query<(), With<Fruit>>,
    mut game_state: ResMut<GameState>,
) {
    for evt in cut_event.read() {
        if bombs.contains(evt.target) {
            commands.entity(evt.target).despawn();
        }
    }
}

#[derive(Component, Default, Clone, Debug)]
pub struct Fruit;
