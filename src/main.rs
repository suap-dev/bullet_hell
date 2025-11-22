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
        .add_plugins(FrameTimeDiagnosticsPlugin::default());

    game.register_diagnostic(
        Diagnostic::new(FrameTimeDiagnosticsPlugin::FPS).with_smoothing_factor(0.5),
    );

    game.add_systems(
        Startup,
        (camera::setup, ui::spawn, player::spawn, enemies::spawn),
    )
    .add_systems(
        Update,
        (
            player::handle_input,
            player::target_closest_enemy,
            player::shoot_target,
            // player::report_hp,
            projectiles::hit_and_damage_enemy,
            enemies::seek_and_follow_player,
            enemies::hit_and_damage_player,
            transforms::apply_velocity,
            transforms::apply_angular_velocity,
            transforms::teleport_at_borders,
            death,
        ),
    )
    .add_systems(PostUpdate, (ui::fps_counter, ui::player_healthbar));

    game.run();
}
