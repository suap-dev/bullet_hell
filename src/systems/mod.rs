pub mod camera;
pub mod enemies;
pub mod player;
pub mod projectiles;
pub mod transforms;
pub mod ui;

use bevy::prelude::*;

use crate::components::attributes;

pub fn death(
    mut commands: Commands,
    mut with_lifespan: Query<(Entity, &mut attributes::LifeSpan)>,
    with_hitpoints: Query<(Entity, &attributes::Hitpoints)>,
    time: Res<Time>,
) {
    let delta = time.delta();
    for (entity, mut lifespan) in &mut with_lifespan {
        if let Ok(mut e) = commands.get_entity(entity) {
            if lifespan.0.tick(delta).is_finished() {
                e.despawn();
            }
        }
    }
    for (entity, hitpoints) in &with_hitpoints {
        if let Ok(mut e) = commands.get_entity(entity) {
            if hitpoints.current() <= 0.0 {
                e.despawn();
            }
        }
    }
}
