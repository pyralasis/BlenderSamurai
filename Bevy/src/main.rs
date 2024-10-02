use appstate::{cleanup_main_menu, menu, setup_main_menu, AppStates};
use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use game::{
    game_loop,
    movement::{move_objects, Velocity},
    shapes::CircleResource,
    spawn_things,
    sword::{sword_position, Sword},
    GameState, SpawnEvent,
};

pub mod appstate;
pub mod game;
pub mod rng;
pub mod view;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WorldInspectorPlugin::new()))
        .init_state::<AppStates>()
        .add_systems(Startup, setup)
        // Main Menu Stuff
        .add_systems(OnEnter(AppStates::MainMenu), setup_main_menu)
        .add_systems(Update, menu.run_if(in_state(AppStates::MainMenu)))
        .add_systems(OnExit(AppStates::MainMenu), cleanup_main_menu)
        // Game Stuff
        .init_resource::<GameState>()
        .init_resource::<CircleResource>()
        .init_resource::<Sword>()
        .add_event::<SpawnEvent>()
        .add_systems(
            Update,
            (game_loop, move_objects, spawn_things, sword_position)
                .run_if(in_state(AppStates::InGame)),
        )
        // Run
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            near: -1000.0,
            scaling_mode: ScalingMode::AutoMin {
                min_width: view::WIDTH,
                min_height: view::HEIGHT,
            },
            viewport_origin: Vec2::new(0.0, 0.0),
            ..default()
        },
        ..default()
    });
}
