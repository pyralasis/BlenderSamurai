use bevy::{prelude::*, utils::info};

use crate::game;

use super::{bomb::CutBombEvent, shapes::CircleResource, GameState};

#[derive(Event, Debug)]
pub struct HitBomb;

#[derive(Component, Debug)]
pub struct LifeCounter;

#[derive(Bundle, Debug)]
pub struct LifeCounterBundle {
    life_counter: LifeCounter,
    text: TextBundle,
}

pub fn setup_lives(mut commands: Commands, game_state: Res<GameState>) {
    commands.spawn(LifeCounterBundle {
        life_counter: LifeCounter,
        text: TextBundle::from_section(
            game_state.lives.to_string(), // format!("{}", game_state.lives)
            TextStyle {
                font_size: 40.0,
                color: Color::srgb(0.9, 0.9, 0.9),
                ..default()
            },
        ),
    });
}

pub fn update_lives(
    mut life_counter: Query<&mut Text, With<LifeCounter>>,
    mut game_state: ResMut<GameState>,
    mut cut_bomb_event: EventReader<CutBombEvent>,
) {
    for evt in cut_bomb_event.read() {
        let mut counter = life_counter.single_mut();
        counter.sections[0] = game_state.lives.to_string().into();
    }
}
