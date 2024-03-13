use bevy::{ecs::bundle::Bundle, sprite};

use crate::components::{attributes, markers};

#[derive(Bundle, Default)]
pub struct Projectile {
    pub marker: markers::Projectile,
    pub material_mesh_bundle: sprite::MaterialMesh2dBundle<sprite::ColorMaterial>,
    pub circumradius: attributes::Circumradius,
    pub movement: attributes::Movement,
    pub range: attributes::Range,
    pub lifespan: attributes::LifeSpan,
}
