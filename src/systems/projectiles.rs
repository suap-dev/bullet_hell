use bevy::{
    ecs::{
        entity::Entity,
        query::With,
        system::{Commands, Query},
    },
    transform::components::Transform,
};

use crate::components::{attributes, markers};

// TODO: this should actualy DEAL DAMAGE, not instant-kill an enemy
pub fn deal_damage(
    mut commands: Commands,
    projectiles: Query<(Entity, &Transform, &attributes::Circumradius), With<markers::Projectile>>,
    enemies: Query<(Entity, &Transform, &attributes::Circumradius), With<markers::Enemy>>,
) {
    for (p_entity, p_transform, p_circumradius) in &projectiles {
        for (e_entity, e_transform, e_circumradius) in &enemies {
            
            if commands.get_entity(p_entity).is_some()
                && commands.get_entity(e_entity).is_some()
                && p_transform
                    .translation
                    .distance_squared(e_transform.translation)
                    < (p_circumradius.0 + e_circumradius.0).powi(2)
            {
                commands.entity(p_entity).despawn();
                commands.entity(e_entity).despawn();
            }
        }
    }
}
