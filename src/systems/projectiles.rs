use avian2d::prelude::CollisionStart;
// use avian2d::prelude::*;
use bevy::prelude::*;

use crate::components::{attributes, markers};

pub fn hit_and_damage_enemy(
    mut commands: Commands,
    // mut collision_events: EventReader<Collision>,
    mut collision_events: MessageReader<CollisionStart>,
    mut enemy_hitpoints: Query<&mut attributes::Hitpoints, With<markers::Enemy>>,
    projectile_damage: Query<&attributes::Damage, With<markers::Projectile>>,
) {
    // for contact in collision_events.read() {
    for CollisionStart {
        collider1: entity1,
        collider2: entity2,
        ..
    } in collision_events.read()
    {
        if let (Ok(damage), Ok(mut hp)) = (
            projectile_damage.get(*entity1),
            enemy_hitpoints.get_mut(*entity2),
        ) {
            hp.damage(damage.0);
            commands.entity(*entity1).try_despawn();
        } else if let (Ok(damage), Ok(mut hp)) = (
            projectile_damage.get(*entity2),
            enemy_hitpoints.get_mut(*entity1),
        ) {
            hp.damage(damage.0);
            commands.entity(*entity2).try_despawn();
        }
    }
}
