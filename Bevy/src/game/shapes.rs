use bevy::{prelude::*, sprite::Mesh2dHandle};
use enum_iterator::all;

use crate::game::SpawnType;

#[derive(Resource)]
pub struct CircleResource {
    pub mesh: Mesh2dHandle,
    pub colors: Vec<Handle<ColorMaterial>>,
}

impl CircleResource {
    pub fn create(
        mut materials: ResMut<Assets<ColorMaterial>>,
        mut meshes: ResMut<Assets<Mesh>>,
    ) -> CircleResource {
        let mut colors = Vec::new();
        for spawn_type in all::<SpawnType>() {
            colors.push(materials.add(spawn_type.color()));
        }

        CircleResource {
            mesh: Mesh2dHandle(meshes.add(Circle::new(1.0))),
            colors,
        }
    }

    pub fn get_spawn_material(&self, spawn_type: &SpawnType) -> Handle<ColorMaterial> {
        let index = *spawn_type as usize;
        return self.colors[index].clone();
    }
}
