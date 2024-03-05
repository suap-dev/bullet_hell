#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod components;
mod systems;

use bevy::{
    app::{App, PluginGroup, Startup, Update},
    diagnostic::FrameTimeDiagnosticsPlugin,
    utils::default,
    window::{Window, WindowPlugin, WindowResolution},
    DefaultPlugins,
};

use crate::systems::{mfkers, setup};

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(800., 600.),
                resizable: false,
                ..default()
            }),
            ..default()
        }),
        FrameTimeDiagnosticsPlugin,
    ))
    .add_systems(Startup, (setup::camera, mfkers::spawn))
    .add_systems(Update, mfkers::update);

    // #[cfg(debug_assertions)] // debug/dev builds only
    {
        use bevy::diagnostic::LogDiagnosticsPlugin;
        app.add_plugins(LogDiagnosticsPlugin::default());
    }

    app.run();
}
