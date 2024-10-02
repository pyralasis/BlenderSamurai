use bevy::{asset::transformer, ecs::reflect, prelude::*, window::PrimaryWindow};

use super::cut::{CutEvent, Cuttable, IsCutting};

#[derive(Resource, Default, Reflect, Debug)]
#[reflect(Resource)]
pub struct Sword {
    pub position: Vec2,
}

pub fn sword_position(
    mut sword: ResMut<Sword>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = q_window.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        sword.position = world_position;
    }
}

pub fn check_for_start_cut(
    mut commands: Commands,
    sword: Res<Sword>,
    cuttables: Query<(&Cuttable, &Transform, Entity), Without<IsCutting>>,
) {
    for (cuttable, transform, entity) in cuttables.iter() {
        let cuttable_position: Vec2 = transform.translation.xy();
        if sword.position.distance(cuttable_position) < cuttable.radius {
            let local_sword_position = sword.position - cuttable_position;
            commands
                .entity(entity)
                .insert(IsCutting::new(local_sword_position));
        }
    }
}

pub fn check_for_end_cut(
    mut commands: Commands,
    sword: Res<Sword>,
    cuttables: Query<(&Cuttable, &Transform, Entity, &IsCutting)>,
    mut cut_event: EventWriter<CutEvent>,
) {
    for (cuttable, transform, entity, is_cutting) in cuttables.iter() {
        let cuttable_position: Vec2 = transform.translation.xy();

        if sword.position.distance(cuttable_position) > cuttable.radius {
            let local_sword_position = sword.position - cuttable_position;
            if is_cutting.enter_position.distance(local_sword_position)
                > (cuttable.radius * 2.0 * 0.7)
            {
                info!("YOU ARE A TRUE BLENDER SAMURAI");
                cut_event.send(CutEvent::new(entity));
            } else {
                commands.entity(entity).remove::<IsCutting>();
            }
        }
    }
}
