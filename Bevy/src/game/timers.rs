use bevy::prelude::*;

use super::GameState;

#[derive(Component, Debug)]
pub struct BlendTimeTimer;

#[derive(Component, Debug)]
pub struct TotalTimeTimer;

#[derive(Bundle, Debug)]
pub struct BlendTimeTimerBundle {
    life_counter: BlendTimeTimer,
    text: TextBundle,
}

#[derive(Bundle, Debug)]
pub struct TotalTimeTimerBundle {
    life_counter: TotalTimeTimer,
    text: TextBundle,
}

pub fn setup_timers(mut commands: Commands, game_state: Res<GameState>) {
    commands.spawn(BlendTimeTimerBundle {
        life_counter: BlendTimeTimer,
        text: TextBundle::from_section(
            game_state.blend_time.remaining_secs().to_string(), // format!("{}", game_state.lives)
            TextStyle {
                font_size: 40.0,
                color: Color::srgb(0.9, 0.9, 0.9),
                ..default()
            },
        ),
    });
    commands.spawn(TotalTimeTimerBundle {
        life_counter: TotalTimeTimer,
        text: TextBundle {
            text: Text::from_section(
                game_state.total_game_time.remaining_secs().to_string(),
                TextStyle {
                    font_size: 40.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ),
            transform: Transform::from_xyz(10.0, 10.0, 10.0),
            ..default()
        },
    });
}
