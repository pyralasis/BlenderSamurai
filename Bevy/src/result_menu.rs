use bevy::prelude::*;

use crate::{
    appstate::AppStates,
    custom_ui::{ButtonPressEvent, UICommands},
    game::GameState,
};

pub struct ResultMenuPlugin;
impl Plugin for ResultMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppStates::ResultMenu), setup_menu)
            .add_systems(OnExit(AppStates::ResultMenu), cleanup_menu);
    }
}

#[derive(Resource)]
struct MenuData {
    menu_container: Entity,
}

fn setup_menu(mut commands: Commands, game_state: Res<GameState>) {
    let menu_container = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(20.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .id();

    commands
        .spawn(
            TextBundle::from_section(
                if game_state.lives > 0 {
                    "You won!"
                } else {
                    "You lost :("
                },
                TextStyle {
                    font_size: 60.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ),
            // .with_style(Style {
            //     padding: UiRect {
            //         left: Val::Px(0.0),
            //         right: Val::Px(0.0),
            //         top: Val::Px(20.0),
            //         bottom: Val::Px(20.0),
            //     },
            //     ..default()
            // }),
        )
        .set_parent(menu_container);

    commands
        .spawn_button(
            "To Menu".into(),
            |_trigger: Trigger<ButtonPressEvent>, mut next_state: ResMut<NextState<AppStates>>| {
                next_state.set(AppStates::MainMenu);
            },
        )
        .set_parent(menu_container);

    commands.insert_resource(MenuData { menu_container });
}

fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    commands
        .entity(menu_data.menu_container)
        .despawn_recursive();

    commands.remove_resource::<MenuData>();
}
