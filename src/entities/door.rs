use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::player::{PlayerColliderMarker, PlayerInventory, PlayerMarker, SetCheckpointEvent};

#[derive(Component, Debug)]
pub struct DoorMarker;

#[derive(Component, Debug)]
pub struct DoorState {
    unlocked: bool,
}

#[derive(Component, Debug)]
struct DoorColliderMarker;

#[derive(Component, Debug)]
pub struct DoorSensorMarker;

#[derive(Bundle, LdtkEntity)]
pub struct DoorBundle {
    #[sprite_sheet_bundle("../assets/spritesheets/door.png", 16, 32, 2, 1, 0, 0, 0)]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
    door_marker: DoorMarker,
    door_state: DoorState,
    // rigid_body: RigidBody,
    // collider: Collider,
}

impl Default for DoorBundle {
    fn default() -> Self {
        Self {
            sprite_sheet_bundle: LdtkSpriteSheetBundle::default(),
            door_marker: DoorMarker,
            door_state: DoorState { unlocked: false },
            // rigid_body: RigidBody::Dynamic,
            // collider: Collider::cuboid(2., 16.),
            // collider: Collider::round_cuboid(5., 3., 2.),
        }
    }
}

pub fn add_door_interaction(mut commands: Commands, query_doors: Query<Entity, Added<DoorMarker>>) {
    for door in query_doors.iter() {
        commands.entity(door).with_children(|parent| {
            parent.spawn((
                Collider::cuboid(16., 16.),
                Sensor,
                ActiveEvents::COLLISION_EVENTS,
                TransformBundle::from_transform(Transform::from_xyz(-4., 0., 0.)),
                DoorSensorMarker,
            ));
            parent.spawn((
                Collider::cuboid(2., 16.),
                ActiveEvents::COLLISION_EVENTS,
                TransformBundle::from_transform(Transform::from_xyz(-4., 0., 0.)),
                DoorColliderMarker,
            ));
        });
    }
}

pub fn check_door_interacting(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    mut query_doors: Query<(&mut Parent, Entity), With<DoorSensorMarker>>,
    mut query_player: Query<&mut PlayerInventory, With<PlayerMarker>>,
    query_player_collider: Query<Entity, With<PlayerColliderMarker>>,
    mut query_door_state: Query<(Entity, &mut DoorState)>,
    mut query_door_texture: Query<&mut TextureAtlas, With<DoorMarker>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut checkpoint_event_writer: EventWriter<SetCheckpointEvent>,
) {
    let Ok(mut inventory) = query_player.get_single_mut() else {
        return;
    };
    let Ok(player_collider) = query_player_collider.get_single() else {
        return;
    };
    if !keys.just_pressed(KeyCode::KeyQ) {
        return;
    }

    for (door, door_collider) in query_doors.iter_mut() {
        let (door_entity, door_state) = &mut query_door_state.get_mut(door.get()).unwrap();
        let atlas = &mut query_door_texture.get_mut(door.get()).unwrap();
        if rapier_context.intersection_pair(player_collider, door_collider) == Some(true) {
            if inventory.num_keys >= 1 {
                println!("UNLOCKING DOOR");
                door_state.unlocked = true;
                commands.entity(*door_entity).despawn_descendants();
                inventory.num_keys -= 1;
                atlas.index = 1;
                checkpoint_event_writer.send(SetCheckpointEvent);
            } else {
                println!("NEED KEY FOR DOOR");
            }
        }
    }
}
