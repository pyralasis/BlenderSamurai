use std::time::Duration;

use bevy::prelude::*;
use spawn::{SpawnEvent, SpawnType};

use crate::rng::Random;

pub mod bomb;
pub mod cut;
pub mod fruit;
pub mod life;
pub mod movement;
pub mod shapes;
pub mod spawn;
pub mod sword;

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct GameState {
    pub time_till_spawn: Timer,
    pub lives: i32,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            time_till_spawn: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
            lives: 3,
        }
    }
}

impl GameState {
    pub fn decrement_lives(&mut self) -> i32 {
        self.lives -= 1;
        return self.lives;
    }
}

pub fn game_loop(
    mut state: ResMut<GameState>,
    time: Res<Time>,
    mut spawn: EventWriter<SpawnEvent>,
) {
    state.time_till_spawn.tick(time.delta());
    if state.time_till_spawn.finished() {
        spawn.send(SpawnEvent::new(rand::random::<SpawnType>()));
        state
            .time_till_spawn
            .set_duration(Duration::from_secs_f32(Random::between(1.0, 2.0)));
    }
}
