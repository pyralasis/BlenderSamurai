use bevy::prelude::*;

use super::{cut::CutEvent, GameState};

pub fn on_bomb_cut(
    mut commands: Commands,
    mut cut_event: EventReader<CutEvent>,
    bombs: Query<(), With<Bomb>>,
    mut game_state: ResMut<GameState>,
) {
    for evt in cut_event.read() {
        if bombs.contains(evt.target) {
            game_state.lives -= 1;
            commands.entity(evt.target).despawn();
        }
    }
}

#[derive(Component, Default, Clone, Debug)]
pub struct Bomb;
