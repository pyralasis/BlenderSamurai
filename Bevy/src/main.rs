use std::time::Duration;

use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use game::{
    game_loop,
    movement::{move_objects, Velocity},
    shapes::CircleResource,
    spawn_things, GameState, SpawnEvent,
};

pub mod game;
pub mod rng;
pub mod view;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WorldInspectorPlugin::new()))
        .add_event::<SpawnEvent>()
        .add_systems(Startup, setup)
        .add_systems(Update, (game_loop, move_objects, spawn_things))
        .insert_resource(GameState {
            time_till_spawn: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
        })
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
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

    commands.insert_resource(CircleResource::create(materials, meshes));

    // commands.spawn((
    //     SpriteBundle::default(),
    //     Velocity {
    //         velocity: Vec2::new(0.0, 40.0),
    //         gravity_scalar: 1.0,
    //     },
    // ));
}
