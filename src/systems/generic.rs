use bevy::{
    ecs::{
        entity::Entity,
        system::{Commands, Query, Res},
    },
    time::Time,
};

use crate::components::attributes;

pub fn living(
    mut commands: Commands,
    mut lifespans: Query<(Entity, &mut attributes::LifeSpan)>,
    time: Res<Time>,
) {
    let delta = time.delta();
    for (entity, mut lifespan) in &mut lifespans {
        if lifespan.0.tick(delta).finished() {
            commands.entity(entity).despawn();
        }
    }
}
