use avian2d::prelude::*;
use bevy::prelude::*;

use crate::components::{attributes, markers};

pub fn damage_enemy(
    mut commands: Commands,
    collisions: Collisions,
    projectiles: Query<(Entity, &attributes::Damage), With<markers::Projectile>>,
    mut enemy_hitpoints: Query<&mut attributes::Hitpoints, With<markers::Enemy>>,
) {
    for (projectile_entity, projectile_damage) in &projectiles {
        for enemy_entity in collisions.entities_colliding_with(projectile_entity) {
            if let Ok(mut hp) = enemy_hitpoints.get_mut(enemy_entity) {
                hp.damage(projectile_damage.0);
                commands.entity(projectile_entity).try_despawn();
            }
        }
    }
}
