use avian2d::prelude::{Collider, CollisionLayers};
use bevy::{ecs::bundle::Bundle, transform::components::Transform};

use crate::components::attributes;

#[derive(Bundle, Default)]
pub struct Body {
    pub transform: Transform,
    pub radius: attributes::Radius,
    pub collider: Collider,
    pub collision_layers: CollisionLayers,
}
