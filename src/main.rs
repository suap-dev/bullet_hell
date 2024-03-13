mod bundles;
mod components;
mod systems;

use bevy::{
    prelude::*,
    window::{EnabledButtons, WindowResolution},
};

#[allow(clippy::wildcard_imports)]
use crate::systems::*;

const LOGICAL_WIDTH: f32 = 800.0;
const LOGICAL_HEIGHT: f32 = 600.0;

#[derive(Resource)]
struct Borders {
    pub top: f32,
    pub bottom: f32,
    pub right: f32,
    pub left: f32,
}

fn main() {
    let mut app = App::new();

    app.insert_resource(Borders {
        top: LOGICAL_HEIGHT / 2.0,
        bottom: -LOGICAL_HEIGHT / 2.0,
        right: LOGICAL_WIDTH / 2.0,
        left: -LOGICAL_WIDTH / 2.0,
    })
    .insert_resource(Time::<Fixed>::from_hz(6.0));

    #[cfg(debug_assertions)]
    {
        use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
        app.add_plugins((LogDiagnosticsPlugin::default(), FrameTimeDiagnosticsPlugin));
    }

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(LOGICAL_WIDTH, LOGICAL_HEIGHT),
            resizable: false,
            enabled_buttons: EnabledButtons {
                maximize: false,
                ..default()
            },
            ..default()
        }),
        ..default()
    }))
    // level setup
    .add_systems(Startup, camera::setup)
    // player
    .add_systems(Startup, player::spawn)
    .add_systems(Update, player::movement_input)
    .add_systems(FixedUpdate, player::find_nearest_enemy)
    .add_systems(FixedUpdate, player::shoot_nearest_enemy)
    // enemies
    .add_systems(Startup, enemies::spawn)
    .add_systems(Update, enemies::seek_player)
    // projectiles
    .add_systems(Update, projectiles::deal_damage)
    // transforms
    .add_systems(
        Update,
        (
            transform::apply_velocity,
            transform::apply_angular_velocity,
            transform::teleport_at_borders,
        ),
    )
    // generic
    .add_systems(Update, generic::living)
    .run();
}
