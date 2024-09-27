use bevy::{ecs::system::EntityCommands, prelude::*};

pub mod bomb;
pub mod cut;
pub mod fruit;
pub mod movement;

pub fn game_loop() {}

pub fn spawn_things(commands: Commands, mut evts: EventReader<SpawnEvent>) {
    for evt in evts.read() {
        evt.spawn_type.spawn(&commands);
    }
}

#[derive(Event)]
struct SpawnEvent {
    spawn_type: SpawnType,
}

enum SpawnType {
    Bomb,
    Watermelon,
    Apple,
    Orange,
    Grape,
}

impl SpawnType {
    fn radius(&self) -> f32 {
        match self {
            SpawnType::Bomb => 20.0,
            SpawnType::Watermelon => 50,
            SpawnType::Apple => 20,
            SpawnType::Orange => 20,
            SpawnType::Grape => 5,
        }
    }

    fn spawn(&self, mut commands: &Commands) -> EntityCommands {
        match self {
            SpawnType::Bomb => commands.spawn(BombBundle {
                cuttable: Cuttable {
                    radius: self.radius(),
                },
            }),
            SpawnType::Watermelon => todo!(),
            SpawnType::Apple => todo!(),
            SpawnType::Orange => todo!(),
            SpawnType::Grape => todo!(),
        }
    }
}
