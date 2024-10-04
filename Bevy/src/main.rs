use appstate::AppStates;
use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use plugins::GamePlugins;

pub mod appstate;
pub mod custom_ui;
pub mod game;
pub mod menu;
pub mod plugins;
pub mod rng;
pub mod view;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WorldInspectorPlugin::new(), GamePlugins))
        .init_state::<AppStates>()
        .add_systems(Startup, setup)
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
