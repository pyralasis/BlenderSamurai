use bevy::prelude::*;

use super::GameState;

#[derive(Component, Debug)]
pub struct BlendTimeTimer;

#[derive(Component, Debug)]
pub struct TotalTimeTimer;

pub fn setup_timers(mut commands: Commands, game_state: Res<GameState>) {
    let timer_container = commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                //justify_content: JustifyContent::FlexEnd,
                align_items: AlignItems::FlexEnd,
                row_gap: Val::Px(5.0),

                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                right: Val::Px(10.0),
                ..default()
            },
            ..default()
        })
        .id();

    commands
        .spawn((
            TotalTimeTimer,
            Name::new("Total Time Timer"),
            TextBundle {
                text: Text::from_section(
                    game_state.total_game_time.remaining_secs().to_string(),
                    TextStyle {
                        font_size: 40.0,
                        color: Color::srgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ),
                ..default()
            },
        ))
        .set_parent(timer_container);

    commands
        .spawn((
            BlendTimeTimer,
            Name::new("Blend Time Timer"),
            TextBundle::from_section(
                game_state.blend_time.remaining_secs().to_string(), // format!("{}", game_state.lives)
                TextStyle {
                    font_size: 30.0,
                    color: Color::srgb(0.8, 0.8, 0.8),
                    ..default()
                },
            ),
        ))
        .set_parent(timer_container);
}

fn timer_to_str(timer: &Timer) -> String {
    let blend_dur = timer.remaining();
    let mins = blend_dur.as_secs() / 60;
    let secs = blend_dur.as_secs() % 60;
    let milis = (blend_dur.as_millis() % 1000) / 10;
    format!("{:0>2}:{:0>2}:{:0>2}", mins, secs, milis)
}

pub fn update_timers(
    mut total: Query<&mut Text, (With<TotalTimeTimer>, Without<BlendTimeTimer>)>,
    mut blend: Query<&mut Text, (With<BlendTimeTimer>, Without<TotalTimeTimer>)>,
    game_state: Res<GameState>,
) {
    let mut total_text = total.single_mut();
    total_text.sections[0].value = timer_to_str(&game_state.total_game_time);

    let mut blend_text = blend.single_mut();
    blend_text.sections[0].value = timer_to_str(&game_state.blend_time);
}

pub fn cleanup_timers(
    mut commands: Commands,
    total: Query<Entity, (With<TotalTimeTimer>, Without<BlendTimeTimer>)>,
    blend: Query<Entity, (With<BlendTimeTimer>, Without<TotalTimeTimer>)>,
) {
    for entity in total.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for entity in blend.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
