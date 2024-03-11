use bevy::{ecs::component::Component, math::Vec2};

// TODO: check if bevy's 'pub struct Direction2d(Vec2)' isn't a better option to do this
#[derive(Component, Clone, Copy)]
pub struct Movement {
    pub direction: Vec2,
    pub speed: f32,
}
impl Movement {
    pub fn velocity(self) -> Vec2 {
        if self.direction == Vec2::ZERO {
            Vec2::ZERO
        } else {
            self.direction.normalize() * self.speed
        }
    }
}

#[derive(Component)]
pub struct AngularVelocity(pub f32);

#[derive(Component)]
pub struct Circumradius(pub f32);

#[derive(Component)]
pub struct HitPoints(pub f32);

#[derive(Component)]
pub struct LineOfSightRange(pub f32);

#[derive(Component)]
pub struct Range(pub f32);

#[derive(Component)]
pub struct LifeSpan(pub f32);
