use bevy::{
    ecs::{
        entity::Entity,
        query::With,
        system::{Commands, Query},
    },
    transform::components::Transform,
};

use crate::components::{attributes, markers};

pub fn deal_damage(
    mut commands: Commands,
    projectiles: Query<
        (
            Entity,
            &Transform,
            &attributes::CollisionRadius,
            &attributes::Damage,
        ),
        With<markers::Projectile>,
    >,
    mut enemies: Query<
        (
            &Transform,
            &attributes::CollisionRadius,
            &mut attributes::HitPoints,
        ),
        With<markers::Enemy>,
    >,
) {
    for (p_entity, p_transform, p_circumradius, damage) in &projectiles {
        for (e_transform, e_circumradius, mut hitpoints) in &mut enemies {
            if commands.get_entity(p_entity).is_some()
                && p_transform
                    .translation
                    .distance_squared(e_transform.translation)
                    < (p_circumradius.0 + e_circumradius.0).powi(2)
            {
                hitpoints.0 -= damage.0;
                commands.entity(p_entity).despawn();
            }
        }
    }
}
