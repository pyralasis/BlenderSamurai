use bevy::prelude::*;

use crate::{
    custom_ui::CustomUIPlugin, game::GameLoopPlugin, main_menu::MainMenuPlugin,
    result_menu::ResultMenuPlugin,
};

pub struct GamePlugins;
impl Plugin for GamePlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MainMenuPlugin,
            GameLoopPlugin,
            ResultMenuPlugin,
            CustomUIPlugin,
        ));
    }
}
