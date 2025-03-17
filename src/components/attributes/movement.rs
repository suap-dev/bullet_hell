use bevy::{ecs::component::Component, math::Vec2};

// TODO: check if bevy's 'pub struct Direction2d(Vec2)' isn't a better option to do this
#[derive(Component, Clone, Copy, Default)]
pub struct Movement {
    direction: Vec2,
    pub max_speed: f32,
}
impl Movement {
    pub fn new(direction: Vec2, max_speed: f32) -> Self {
        let mut new = Self::from_max_speed(max_speed);
        new.set_direction(direction);
        new
    }

    pub const fn from_max_speed(max_speed: f32) -> Self {
        Self {
            direction: Vec2::ZERO,
            max_speed,
        }
    }

    pub fn from_velocity(velocity: Vec2) -> Self {
        Self {
            direction: velocity.normalize_or_zero(),
            max_speed: velocity.length(),
        }
    }

    pub fn set_direction(&mut self, from: Vec2) {
        self.direction = from.normalize_or_zero();
    }

    #[must_use]
    pub fn get_velocity(&self) -> Vec2 {
        self.direction * self.max_speed
    }
}
