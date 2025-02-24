use bevy::{ecs::bundle::Bundle, sprite::Material2d, transform::components::Transform};

use crate::{
    bundles::ProtoSprite,
    components::{attributes, markers},
};

#[derive(Bundle, Default)]
pub struct Enemy<M: Material2d> {
    pub marker: markers::Enemy,
    pub sprite: ProtoSprite<M>,
    pub transform: Transform,
    pub collision_radius: attributes::CollisionRadius,
    pub movement: attributes::Movement,
    pub hitpoints: attributes::HitPoints,
    pub los_range: attributes::LineOfSightRange,
    pub angular_velocity: attributes::AngularVelocity,
}
