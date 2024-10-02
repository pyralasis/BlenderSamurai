use bevy::{prelude::*, sprite::Mesh2dHandle};
use enum_iterator::all;

use crate::game::spawn::SpawnType;

#[derive(Resource)]
pub struct CircleResource {
    pub mesh: Mesh2dHandle,
    pub colors: Vec<Handle<ColorMaterial>>,
}

impl CircleResource {
    pub fn get_spawn_material(&self, spawn_type: &SpawnType) -> Handle<ColorMaterial> {
        let index = *spawn_type as usize;
        return self.colors[index].clone();
    }
}

impl FromWorld for CircleResource {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.resource_mut::<Assets<ColorMaterial>>();
        let mut colors = Vec::new();
        for spawn_type in all::<SpawnType>() {
            colors.push(materials.add(spawn_type.color()));
        }

        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        CircleResource {
            mesh: Mesh2dHandle(meshes.add(Circle::new(1.0))),
            colors,
        }
    }
}
