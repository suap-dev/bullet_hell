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
        if let Some(mut e) = commands.get_entity(entity) {
            if lifespan.0.tick(delta).finished() {
                e.despawn();
            }
        }
    }
}
