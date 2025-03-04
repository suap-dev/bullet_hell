use avian2d::prelude::*;
use bevy::prelude::*;

use crate::components::{attributes, markers};

pub fn hit_and_damage_enemy(
    mut commands: Commands,
    mut collision_events: EventReader<Collision>,
    mut enemy_hitpoints: Query<&mut attributes::Hitpoints, With<markers::Enemy>>,
    projectile_damage: Query<&attributes::Damage, With<markers::Projectile>>,
) {
    for Collision(contact) in collision_events.read() {
        let entity1 = contact.entity1;
        let entity2 = contact.entity2;

        if let (Ok(damage), Ok(mut hp)) = (
            projectile_damage.get(entity1),
            enemy_hitpoints.get_mut(entity2),
        ) {
            hp.0 -= damage.0;
            commands.entity(entity1).try_despawn();
        } else if let (Ok(damage), Ok(mut hp)) = (
            projectile_damage.get(entity2),
            enemy_hitpoints.get_mut(entity1),
        ) {
            hp.0 -= damage.0;
            commands.entity(entity2).try_despawn();
        }
    }
}

// pub fn deal_damage(
//     mut commands: Commands,
//     projectiles: Query<
//         (Entity, &Transform, &attributes::Radius, &attributes::Damage),
//         With<markers::Projectile>,
//     >,
//     mut enemies: Query<
//         (&Transform, &attributes::Radius, &mut attributes::HitPoints),
//         With<markers::Enemy>,
//     >,
// ) {
//     for (p_entity, p_transform, p_circumradius, damage) in &projectiles {
//         for (e_transform, e_circumradius, mut hitpoints) in &mut enemies {
//             if commands.get_entity(p_entity).is_some()
//                 && p_transform
//                     .translation
//                     .distance_squared(e_transform.translation)
//                     < (p_circumradius.0 + e_circumradius.0).powi(2)
//             {
//                 hitpoints.0 -= damage.0;
//                 commands.entity(p_entity).try_despawn();
//             }
//         }
//     }
// }
