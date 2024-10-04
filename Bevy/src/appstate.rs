use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppStates {
    #[default]
    MainMenu,
    InGame,
    LostGame,
    WonGame,
}
