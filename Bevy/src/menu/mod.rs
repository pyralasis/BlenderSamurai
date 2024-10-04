use bevy::prelude::*;

use crate::{
    appstate::AppStates,
    custom_ui::{ButtonPressEvent, UICommands},
};

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppStates::MainMenu), setup_main_menu)
            .add_systems(OnExit(AppStates::MainMenu), cleanup_main_menu);
    }
}

#[derive(Resource)]
pub struct MenuData {
    menu_container: Entity,
}

pub fn setup_main_menu(mut commands: Commands) {
    let menu_container = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: Val::Px(20.0),
                ..default()
            },
            ..default()
        })
        .id();

    commands.insert_resource(MenuData { menu_container });

    commands
        .spawn_button(
            "30s".into(),
            |_trigger: Trigger<ButtonPressEvent>, mut next_state: ResMut<NextState<AppStates>>| {
                next_state.set(AppStates::InGame);
            },
        )
        .set_parent(menu_container);

    commands
        .spawn_button(
            "1m".into(),
            |_trigger: Trigger<ButtonPressEvent>, mut next_state: ResMut<NextState<AppStates>>| {
                next_state.set(AppStates::InGame);
            },
        )
        .set_parent(menu_container);

    commands
        .spawn_button(
            "2m".into(),
            |_trigger: Trigger<ButtonPressEvent>, mut next_state: ResMut<NextState<AppStates>>| {
                next_state.set(AppStates::InGame);
            },
        )
        .set_parent(menu_container);

    commands
        .spawn_button(
            "5m".into(),
            |_trigger: Trigger<ButtonPressEvent>, mut next_state: ResMut<NextState<AppStates>>| {
                next_state.set(AppStates::InGame);
            },
        )
        .set_parent(menu_container);
}

pub fn cleanup_main_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    commands
        .entity(menu_data.menu_container)
        .despawn_recursive();
}
