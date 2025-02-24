use bevy::{ecs::bundle::Bundle, sprite::Material2d, transform::components::Transform};

use crate::{
    bundles::ProtoSprite,
    components::{attributes, markers},
};

#[derive(Bundle, Default)]
pub struct Projectile<M: Material2d> {
    pub marker: markers::Projectile,
    pub sprite: ProtoSprite<M>,
    pub transform: Transform,
    pub damage: attributes::Damage,
    pub collision_radius: attributes::CollisionRadius,
    pub movement: attributes::Movement,
    // pub range: attributes::Range,
    pub lifespan: attributes::LifeSpan,
}
