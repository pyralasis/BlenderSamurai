use appstate::{cleanup_main_menu, menu, setup_main_menu, AppStates};
use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use game::{
    bomb::on_bomb_cut,
    cut::CutEvent,
    fruit::on_fruit_cut,
    game_loop,
    life::setup_lives,
    movement::move_objects,
    shapes::CircleResource,
    spawn::{spawn_things, SpawnEvent},
    sword::{check_for_end_cut, check_for_start_cut, sword_position, Sword},
    GameState,
};

pub mod appstate;
pub mod game;
pub mod rng;
pub mod view;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WorldInspectorPlugin::new()))
        .register_type::<Sword>()
        .register_type::<GameState>()
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
        .add_event::<CutEvent>()
        .add_systems(OnEnter(AppStates::InGame), setup_lives)
        .add_systems(
            Update,
            (
                game_loop,
                move_objects,
                spawn_things,
                sword_position,
                check_for_start_cut,
                check_for_end_cut,
                on_fruit_cut,
                on_bomb_cut,
            )
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
