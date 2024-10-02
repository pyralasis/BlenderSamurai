use bevy::prelude::*;

use super::{cut::CutEvent, GameState};

#[derive(Event)]
pub struct CutBombEvent;

pub fn on_bomb_cut(
    mut commands: Commands,
    mut cut_event: EventReader<CutEvent>,
    bombs: Query<(), With<Bomb>>,
    mut game_state: ResMut<GameState>,
    mut cut_bomb_event: EventWriter<CutBombEvent>,
) {
    for evt in cut_event.read() {
        if bombs.contains(evt.target) {
            game_state.decrement_lives();
            commands.entity(evt.target).despawn();
            cut_bomb_event.send(CutBombEvent);
        }
    }
}

#[derive(Component, Default, Clone, Debug)]
pub struct Bomb;
