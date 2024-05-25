use bevy::{
    ecs::{
        entity::Entity,
        system::{Commands, Query, Res},
    },
    time::Time,
};

use crate::components::attributes;

pub fn death(
    mut commands: Commands,
    mut with_lifespan: Query<(Entity, &mut attributes::LifeSpan)>,
    with_hitpoints: Query<(Entity, &attributes::HitPoints)>,
    time: Res<Time>,
) {
    let delta = time.delta();
    for (entity, mut lifespan) in &mut with_lifespan {
        if let Some(mut e) = commands.get_entity(entity) {
            if lifespan.0.tick(delta).finished() {
                e.despawn();
            }
        }
    }
    for (entity, hitpoints) in &with_hitpoints {
        if let Some(mut e) = commands.get_entity(entity) {
            if hitpoints.0 <= 0.0 {
                e.despawn();
            }
        }
    }
}


