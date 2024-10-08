use std::usize;

use super::cut::{Cuttable, CuttableBundle};
use super::fruit::Fruit;
use super::movement::Velocity;
use super::shapes::CircleResource;
use super::{bomb::Bomb, movement};
use bevy::{ecs::system::EntityCommands, prelude::*};
use enum_iterator::{all, Sequence};
use rand::{
    distributions::{Distribution, Standard, WeightedIndex},
    Rng,
};

use crate::{rng::Random, view};

pub fn spawn_things(
    mut commands: Commands,
    mut evts: EventReader<SpawnEvent>,
    shapes: Res<CircleResource>,
) {
    for evt in evts.read() {
        let start_x = Random::between(0.0, view::WIDTH);
        let end_x = Random::between(0.0, view::WIDTH);
        let delta_x = end_x - start_x;

        let start_y = -50.0;
        let apex_y = Random::between(
            view::HEIGHT / 3.0,
            view::HEIGHT, /* - thingSpanwedRadius */
        );
        let delta_y = apex_y - start_y;

        let gravity = movement::BASE_GRAVITY; // * gravity scalar

        let y_velocity = f32::sqrt(-2.0 * gravity * delta_y);

        let time_in_flight = -2.0 * y_velocity / gravity;

        let x_velocity = delta_x / time_in_flight;

        evt.spawn_type.spawn(
            &mut commands,
            Vec3::new(start_x, start_y, Random::with_max(1.0)),
            Velocity {
                velocity: Vec2::new(x_velocity, y_velocity),
                gravity_scalar: 1.0,
            },
            &shapes,
        );
        // new_y = init_y + gravity * time
    }
}

pub fn check_despawn(
    mut commands: Commands,
    cuttables: Query<(&mut Transform, Entity), With<Cuttable>>,
) {
    for (transform, entity) in cuttables.iter() {
        if transform.translation.y < -50.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn cleanup_despawn(mut commands: Commands, cuttables: Query<Entity, With<Cuttable>>) {
    for entity in cuttables.iter() {
        commands.entity(entity).despawn();
    }
}

#[derive(Event)]
pub struct SpawnEvent {
    pub spawn_type: SpawnType,
}

impl SpawnEvent {
    pub fn new(spawn_type: SpawnType) -> SpawnEvent {
        SpawnEvent { spawn_type }
    }
}

#[derive(Sequence, Debug, Clone, Copy)]
pub enum SpawnType {
    Bomb = 0,
    Watermelon,
    Apple,
    Orange,
    Grape,
}

impl SpawnType {
    pub fn radius(&self) -> f32 {
        match self {
            SpawnType::Bomb => 20.0,
            SpawnType::Watermelon => 50.0,
            SpawnType::Apple => 20.0,
            SpawnType::Orange => 20.0,
            SpawnType::Grape => 5.0,
        }
    }

    pub fn time_score(&self) -> f32 {
        match self {
            SpawnType::Bomb => 0.0,
            SpawnType::Watermelon => 0.2,
            SpawnType::Apple => 0.5,
            SpawnType::Orange => 0.5,
            SpawnType::Grape => 1.0,
        }
    }

    pub fn weight(&self) -> f32 {
        match self {
            SpawnType::Bomb => 10.0,
            SpawnType::Watermelon => 10.0,
            SpawnType::Apple => 30.0,
            SpawnType::Orange => 30.0,
            SpawnType::Grape => 10.0,
        }
    }

    pub fn weights() -> Vec<f32> {
        all::<SpawnType>()
            .map(|spawn_type| spawn_type.weight())
            .collect()
    }

    pub fn color(&self) -> Color {
        match self {
            SpawnType::Bomb => Color::linear_rgb(0.0, 0.0, 0.0),
            SpawnType::Watermelon => Color::linear_rgb(0.0, 1.0, 0.0),
            SpawnType::Apple => Color::linear_rgb(1.0, 0.0, 0.0),
            SpawnType::Orange => Color::linear_rgb(1.0, 0.69, 0.0),
            SpawnType::Grape => Color::linear_rgb(0.16, 0.0, 0.32),
        }
    }

    pub fn spawn<'a>(
        &self,
        commands: &'a mut Commands,
        pos: Vec3,
        velocity: Velocity,
        shapes: &Res<CircleResource>,
    ) -> EntityCommands<'a> {
        let mesh = shapes.mesh.clone();
        let material = shapes.get_spawn_material(self);
        let mut entity = commands.spawn(CuttableBundle::new(
            pos,
            velocity,
            self.radius(),
            mesh,
            material,
            self.time_score(),
        ));

        match self {
            SpawnType::Bomb => entity.insert(Bomb),
            _ => entity.insert(Fruit),
        };

        entity
    }

    pub fn from_int(index: usize) -> Option<SpawnType> {
        all::<SpawnType>().nth(index)
    }
}

impl Distribution<SpawnType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SpawnType {
        let dist = WeightedIndex::new(&SpawnType::weights()).unwrap();
        return SpawnType::from_int(dist.sample(rng)).unwrap();
    }
}
