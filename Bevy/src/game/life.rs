use bevy::prelude::*;

use super::{shapes::CircleResource, GameState};

#[derive(Event, Debug)]
pub struct HitBomb;

#[derive(Component, Debug)]
pub struct LifeCounter {
    text: TextBundle,
}

impl LifeCounter {
    pub fn new(lives: String) -> Self {
        Self {
            text: TextBundle::from_section(
                lives,
                TextStyle {
                    font_size: 40.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ),
        }
    }

    pub fn update_lives(&mut self, lives: String) {
        self.text = TextBundle::from_section(
            lives,
            TextStyle {
                font_size: 40.0,
                color: Color::srgb(0.9, 0.9, 0.9),
                ..default()
            },
        )
    }
}

pub fn setup_lives(mut commands: Commands, game_state: Res<GameState>) {
    commands.spawn(LifeCounter::new(game_state.lives.to_string()));
}

pub fn draw_lives(mut life_counter: Query<(&LifeCounter)>, game_state: Res<GameState>) {
    let counter = life_counter.single_mut();
    // counter.update_lives();
}
