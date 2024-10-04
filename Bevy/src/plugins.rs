use bevy::prelude::*;

use crate::{custom_ui::CustomUIPlugin, game::GameLoopPlugin, menu::MainMenuPlugin};

pub struct GamePlugins;
impl Plugin for GamePlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MainMenuPlugin,
            GameLoopPlugin,
            LostGamePlugin,
            WonGamePlugin,
            CustomUIPlugin,
        ));
    }
}

struct LostGamePlugin;
impl Plugin for LostGamePlugin {
    fn build(&self, _app: &mut App) {
        //todo!()
    }
}

struct WonGamePlugin;
impl Plugin for WonGamePlugin {
    fn build(&self, _app: &mut App) {
        //todo!()
    }
}
