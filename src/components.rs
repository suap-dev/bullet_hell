use bevy::{ecs::component::Component, math::Vec3};

// Markers:
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

// Stats:
#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct AngularVelocity(pub f32);

#[derive(Component)]
pub struct Circumradius(pub f32);

#[derive(Component)]
pub struct HitPoints(pub f32);

#[derive(Component)]
pub struct LineOfSightRange(pub f32);
