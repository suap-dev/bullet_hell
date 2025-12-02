use avian2d::prelude::{Collider, CollisionLayers, CollisionMargin, RigidBody};
use bevy::{ecs::bundle::Bundle, transform::components::Transform};

use crate::components::attributes;

#[derive(Bundle)]
pub struct Body {
    pub transform: Transform,
    pub radius: attributes::Radius,
    pub collider: Collider,
    // TODO: create constants for collision layers
    // TODO: unify collision layers definitions in the project
    pub collision_layers: CollisionLayers,
    pub collision_margin: CollisionMargin,
    pub rigid_body: RigidBody,
}
