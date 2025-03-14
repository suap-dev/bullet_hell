mod bundles;
mod components;
mod config;
mod plugins;
mod resources;
mod systems;

use bevy::{
    diagnostic::{Diagnostic, FrameTimeDiagnosticsPlugin, RegisterDiagnostic},
    prelude::*,
};

#[allow(clippy::wildcard_imports)]
use crate::systems::*;

fn main() {
    let mut game = App::new();

    game.insert_resource(resources::Borders::default())
        .insert_resource(resources::ShootCooldown::default());

    game.add_plugins(plugins::GamePlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .register_diagnostic(
            Diagnostic::new(FrameTimeDiagnosticsPlugin::FPS).with_smoothing_factor(0.5),
        );

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
        .add_systems(Startup, ui::spawn)
        .add_systems(Update, ui::fps_counter);
    //= collisions =//

    //== LET'S GO! ==//
    game.run();
}
