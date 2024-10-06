use bevy::{asset::transformer, prelude::*};

use super::{cut::CutEvent, movement::Velocity, GameState};

#[derive(Event)]
pub struct CutBombEvent(Vec2);

pub fn on_bomb_cut(
    mut commands: Commands,
    mut cut_event: EventReader<CutEvent>,
    bombs: Query<&Transform, With<Bomb>>,
    mut game_state: ResMut<GameState>,
    mut cut_bomb_event: EventWriter<CutBombEvent>,
) {
    for evt in cut_event.read() {
        if let Ok(bomb_transform) = bombs.get(evt.target) {
            game_state.decrement_lives();
            commands.entity(evt.target).despawn();
            cut_bomb_event.send(CutBombEvent(bomb_transform.translation.xy()));
        }
    }
}

pub fn explode_on_bomb_cut(
    mut evts: EventReader<CutBombEvent>,
    mut movables: Query<(&mut Velocity, &Transform)>,
) {
    for evt in evts.read() {
        for (mut vel, transform) in movables.iter_mut() {
            let delta = transform.translation.xy() - evt.0;
            vel.velocity += delta.normalize() * 1000.0;
        }
    }
}

#[derive(Component, Default, Clone, Debug)]
pub struct Bomb;
