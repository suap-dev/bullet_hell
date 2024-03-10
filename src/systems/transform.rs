use bevy::{
    ecs::system::{Query, Res},
    time::Time,
    transform::components::Transform,
};

use crate::components::attributes;

pub fn apply_velocity(time: Res<Time>, mut query: Query<(&mut Transform, &attributes::Movement)>) {
    for (mut transform, movement) in &mut query {
        transform.translation += (movement.velocity() * time.delta_seconds()).extend(0.0);
    }
}
