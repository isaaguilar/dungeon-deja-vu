use crate::state::LevelLoadingState;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

mod door;
mod key;

use door::{add_door_interaction, check_door_interacting, DoorBundle};
use key::{add_key_sensor, check_key_interacting, KeyBundle};

pub struct EntityManagementPlugin;

impl Plugin for EntityManagementPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<DoorBundle>("Door")
            .register_ldtk_entity::<KeyBundle>("Key")
            .add_systems(
                Update,
                (
                    add_door_interaction,
                    check_door_interacting,
                    add_key_sensor,
                    check_key_interacting,
                )
                    .run_if(in_state(LevelLoadingState::Loaded)),
            );
    }
}
