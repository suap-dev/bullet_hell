mod bundles;
mod components;
mod config;
mod resources;
mod systems;

use avian2d::prelude::*;
use bevy::{
    prelude::*,
    window::{EnabledButtons, WindowResolution},
};

#[allow(clippy::wildcard_imports)]
use crate::systems::*;

fn main() {
    let mut game = App::new();

    //== RESOURCES ==//
    game.insert_resource(Time::<Fixed>::from_hz(config::FIXED_UPDATE_HZ))
        .insert_resource(resources::Borders {
            top: config::WINDOW_LOGICAL_HEIGHT / 2.0,
            bottom: -config::WINDOW_LOGICAL_HEIGHT / 2.0,
            right: config::WINDOW_LOGICAL_WIDTH / 2.0,
            left: -config::WINDOW_LOGICAL_WIDTH / 2.0,
        })
        .insert_resource(resources::ShootCooldown(Timer::from_seconds(
            config::SHOOT_COOLDOWN,
            TimerMode::Repeating,
        )));

    //== PLUGINS ==//
    game.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(
                config::WINDOW_LOGICAL_WIDTH,
                config::WINDOW_LOGICAL_HEIGHT,
            ),
            resizable: false,
            enabled_buttons: EnabledButtons {
                maximize: false,
                ..default()
            },
            ..default()
        }),
        ..default()
    }))
    .add_plugins(PhysicsPlugins::default());

    #[cfg(debug_assertions)]
    {
        use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
        game.add_plugins((LogDiagnosticsPlugin::default(), FrameTimeDiagnosticsPlugin));
    }

    //== SYSTEMS ==//
    game
        //= level setup =//
        .add_systems(Startup, camera::setup)
        //= player =//
        .add_systems(Startup, player::spawn)
        .add_systems(Update, player::handle_input)
        .add_systems(Update, player::target_closest_enemy)
        .add_systems(Update, player::shoot_target)
        //= enemies =//
        .add_systems(Startup, enemies::spawn)
        .add_systems(Update, enemies::seek_and_follow_player)
        //= projectiles =//
        // .add_systems(Update, projectiles::deal_damage)
        .add_systems(FixedUpdate, projectiles::hit_and_damage_enemy)
        //= transforms =//
        .add_systems(
            Update,
            (
                transform::apply_velocity,
                transform::apply_angular_velocity,
                transform::teleport_at_borders,
            ),
        )
        //= generic =//
        .add_systems(Update, death)
        //= ui =//
        .add_systems(Startup, ui::spawn);
    //= collisions =//

    //== LET'S GO! ==//
    game.run();
}
