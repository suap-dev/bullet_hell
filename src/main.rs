#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod systems;

use bevy::{prelude::*, window::WindowResolution};

use crate::systems::{setup, spawn_mfkers, update_mfkers};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(800., 600.),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup, spawn_mfkers))
        .add_systems(Update, update_mfkers)
        .run();
}
