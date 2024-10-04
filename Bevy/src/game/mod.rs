use std::time::Duration;

use bevy::{prelude::*, scene::ron::de};
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
pub mod timers;

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct GameState {
    pub time_till_spawn: Timer,
    pub lives: i32,
    pub total_game_time: Timer,
    pub blend_time: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            time_till_spawn: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
            lives: 3,
            total_game_time: Timer::from_seconds(0.0, TimerMode::Once),
            blend_time: Timer::from_seconds(0.0, TimerMode::Once),
        }
    }
}

impl GameState {
    pub fn decrement_lives(&mut self) {
        self.lives -= 1;
    }

    pub fn get_lives(&mut self) -> String {
        return self.lives.to_string();
    }

    pub fn add_blend_time(&mut self, additional_time: f32) {
        self.blend_time.set_duration(Duration::from_secs_f32(
            self.blend_time.remaining_secs() + additional_time,
        ));
    }

    pub fn reset_blend_time(&mut self) {
        self.blend_time.reset();
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
