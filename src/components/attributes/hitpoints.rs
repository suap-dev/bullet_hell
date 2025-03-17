use bevy::ecs::component::Component;

#[derive(Component, Default)]
pub struct Hitpoints {
    current: f32,
    max: f32,
}
// TODO: create tests for Hitpoints implementation
impl Hitpoints {
    #[must_use]
    pub fn from_max(max: f32) -> Self {
        Self { current: max, max }
    }

    #[must_use]
    pub const fn current(&self) -> f32 {
        self.current
    }

    #[must_use]
    pub const fn max(&self) -> f32 {
        self.max
    }

    pub fn damage(&mut self, damage: f32) -> f32 {
        if damage > 0. {
            let damaged_hitpoints = self.current - damage;
            if damaged_hitpoints < 0. {
                self.current = 0.;
                damage + damaged_hitpoints
            } else {
                self.current = damaged_hitpoints;
                damage
            }
        } else {
            0.
        }
    }

    pub fn heal(&mut self, heal: f32) -> f32 {
        if heal > 0. {
            let healed_hitpoints = self.current + heal;
            if healed_hitpoints > self.max {
                self.current = self.max;
                heal - (healed_hitpoints - self.max)
            } else {
                self.current = healed_hitpoints;
                heal
            }
        } else {
            0.
        }
    }

    pub fn heal_with_overheal(&mut self, heal: f32) -> f32 {
        self.current += heal;
        if self.current > self.max {
            self.current - self.max
        } else {
            0.
        }
    }
}
