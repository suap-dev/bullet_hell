mod bundles;
mod components;
mod systems;

use bevy::{
    app::{App, PluginGroup, Startup, Update},
    diagnostic::FrameTimeDiagnosticsPlugin,
    utils::default,
    window::{EnabledButtons, Window, WindowPlugin, WindowResolution},
    DefaultPlugins,
};

use crate::systems::{camera, enemies, player};

const LOGICAL_WIDTH: f32 = 800.0;
const LOGICAL_HEIGHT: f32 = 600.0;

fn main() {
    let mut app = App::new();

    #[cfg(debug_assertions)] // debug/dev builds only
    {
        use bevy::diagnostic::LogDiagnosticsPlugin;
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
    .add_systems(Startup, camera::setup)
    .add_systems(Startup, player::spawn)
    .add_systems(Startup, enemies::spawn)
    .add_systems(Update, enemies::update);

    app.run();
}
