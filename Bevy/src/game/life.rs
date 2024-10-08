use bevy::prelude::*;

use super::{bomb::CutBombEvent, GameState};

#[derive(Component, Debug)]
pub struct LifeCounter;

pub fn setup_lives(mut commands: Commands, game_state: Res<GameState>) {
    commands.spawn((
        LifeCounter,
        TextBundle::from_section(
            game_state.lives.to_string(), // format!("{}", game_state.lives)
            TextStyle {
                font_size: 40.0,
                color: Color::srgb(0.9, 0.9, 0.9),
                ..default()
            },
        ),
    ));
}

pub fn update_lives(
    mut life_counter: Query<&mut Text, With<LifeCounter>>,
    game_state: ResMut<GameState>,
    mut cut_bomb_event: EventReader<CutBombEvent>,
) {
    for _evt in cut_bomb_event.read() {
        let mut counter = life_counter.single_mut();
        counter.sections[0].value = game_state.lives.to_string().into();
    }
}

pub fn cleanup_lives(mut commands: Commands, life_counter: Query<Entity, With<LifeCounter>>) {
    for entity in life_counter.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
