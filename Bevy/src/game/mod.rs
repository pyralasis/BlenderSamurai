use std::time::Duration;

use bevy::{prelude::*, utils::info};
use bomb::{explode_on_bomb_cut, on_bomb_cut, CutBombEvent};
use cut::{check_despawn, CutEvent};
use fruit::on_fruit_cut;
use life::{setup_lives, update_lives};
use movement::move_objects;
use shapes::CircleResource;
use spawn::{spawn_things, SpawnEvent, SpawnType};
use sword::{check_for_end_cut, check_for_start_cut, sword_position, Sword};
use timers::{setup_timers, update_timers};

use crate::{appstate::AppStates, rng::Random};

mod bomb;
mod cut;
mod fruit;
mod life;
mod movement;
mod shapes;
mod spawn;
mod sword;
mod timers;

pub struct GameLoopPlugin;
impl Plugin for GameLoopPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        app.register_type::<Sword>().register_type::<GameState>();

        app.init_resource::<GameState>()
            .init_resource::<CircleResource>()
            .init_resource::<Sword>()
            .add_event::<SpawnEvent>()
            .add_event::<CutEvent>()
            .add_event::<CutBombEvent>()
            .add_systems(OnEnter(AppStates::InGame), (setup_lives, setup_timers))
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
                    update_lives,
                    check_despawn,
                    update_timers,
                    explode_on_bomb_cut,
                )
                    .run_if(in_state(AppStates::InGame)),
            );
    }
}

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
            total_game_time: Timer::from_seconds(30.0, TimerMode::Once),
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
        info!(
            "AAAA{:?}",
            Duration::from_secs_f32(self.blend_time.remaining_secs() + additional_time,)
        );
        self.blend_time = Timer::from_seconds(
            self.blend_time.remaining_secs() + additional_time,
            TimerMode::Once,
        );
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
            .set_duration(Duration::from_secs_f32(Random::between(0.4, 0.6)));
    }

    if state.blend_time.remaining().as_millis() > 0 {
        // info!("{}", state.blend_time.remaining().as_millis().to_string());
        state.blend_time.tick(time.delta());
        state.total_game_time.tick(time.delta());
    }
}
