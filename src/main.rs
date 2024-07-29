mod camera;
mod entities;
mod level;
mod menus;
mod player;
mod state;

use bevy::{
    asset::AssetMetaCheck,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    ecs::schedule::{LogLevel, ScheduleBuildSettings},
    prelude::*,
    window::PresentMode,
};
use bevy_rapier2d::prelude::*;
use camera::CameraManagementPlugin;
use entities::EntityManagementPlugin;
use level::LevelManagementPlugin;
use menus::MenuManagementPlugin;
use player::PlayerManagementPlugin;
use state::StateManagementPlugin;

fn main() {
    App::new()
        // Enable ambiguity warnings for the Update schedule
        .edit_schedule(Startup, |schedule| {
            schedule.set_build_settings(ScheduleBuildSettings {
                ambiguity_detection: LogLevel::Warn,
                ..default()
            });
        })
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Dungeon Deja Vu".into(),
                        name: Some("dungeon.dejavu".into()),
                        // resolution: (500., .).into(),
                        present_mode: PresentMode::AutoVsync,
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                }),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
        ))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(24.))
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins((
            CameraManagementPlugin,
            LevelManagementPlugin,
            StateManagementPlugin,
            EntityManagementPlugin,
            PlayerManagementPlugin,
            MenuManagementPlugin,
        ))
        .run();
}
