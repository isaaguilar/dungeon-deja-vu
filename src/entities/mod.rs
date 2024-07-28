use crate::state::LevelLoadingState;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

mod clock;
mod door;
mod double_jump;
mod goal;
mod jump_token;
mod key;
mod lever;
mod platform;
mod wall_jump;

use clock::{animate_clock, ClockBundle};
use door::{add_door_interaction, check_door_interacting, DoorBundle};
use double_jump::{add_double_jump_sensor, check_double_jump_acquire, DoubleJumpBundle};
use goal::{add_goal_sensor, check_goal_reached, GoalBundle};
use jump_token::{add_jump_token_sensor, check_jump_token_acquire, JumpTokenBundle};
use key::{add_key_sensor, check_key_interacting, KeyBundle};
use lever::{add_lever_interaction, check_lever_interacting, LeverBundle};
use platform::{insert_platform_colliders, PlatformBundle};
use wall_jump::{add_wall_jump_sensor, check_wall_jump_acquire, WallJumpBundle};

pub struct EntityManagementPlugin;

impl Plugin for EntityManagementPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<DoorBundle>("LockedDoor")
            .register_ldtk_entity::<KeyBundle>("Key")
            .register_ldtk_entity::<ClockBundle>("Clock")
            .register_ldtk_entity::<JumpTokenBundle>("JumpToken")
            .register_ldtk_entity::<DoubleJumpBundle>("DoubleJump")
            .register_ldtk_entity::<LeverBundle>("Lever")
            .register_ldtk_entity::<PlatformBundle>("LeverPlatform")
            .register_ldtk_entity::<GoalBundle>("Goal")
            .register_ldtk_entity::<WallJumpBundle>("WallJump")
            .add_systems(
                Update,
                (
                    add_door_interaction,
                    check_door_interacting,
                    add_key_sensor,
                    check_key_interacting,
                    animate_clock,
                    add_jump_token_sensor,
                    check_jump_token_acquire,
                    add_double_jump_sensor,
                    check_double_jump_acquire,
                    add_lever_interaction,
                    check_lever_interacting,
                    insert_platform_colliders,
                    add_goal_sensor,
                    check_goal_reached,
                    add_wall_jump_sensor,
                    check_wall_jump_acquire,
                )
                    .run_if(in_state(LevelLoadingState::Loaded)),
            );
    }
}
