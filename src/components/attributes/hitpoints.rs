use bevy::ecs::component::Component;

#[derive(Component, Default)]
pub struct Hitpoints {
    current: f32,
    max: f32,
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_max_hp() {
        let hitpoints = Hitpoints::from_max(100.);
        assert_eq!(hitpoints.current, 100.);
        assert_eq!(hitpoints.max, 100.);
    }

    #[test]
    fn heal() {
        let mut hitpoints = Hitpoints {
            current: 0.,
            max: 100.,
        };

        hitpoints.heal(10.);
        assert_eq!(hitpoints.current, 10.);
        assert_eq!(hitpoints.max, 100.);

        assert_eq!(hitpoints.heal(42.), 42.);
        assert_eq!(hitpoints.current, 52.);
        assert_eq!(hitpoints.max, 100.);

        assert_eq!(hitpoints.heal(55.), 48.);
        assert_eq!(hitpoints.current, 100.);
        assert_eq!(hitpoints.max, 100.);
    }

    #[test]
    fn overheal() {
        let mut hitpoints = Hitpoints {
            current: 0.,
            max: 100.,
        };

        assert_eq!(hitpoints.heal_with_overheal(42.), 0.);
        assert_eq!(hitpoints.current, 42.);
        assert_eq!(hitpoints.max, 100.);

        assert_eq!(hitpoints.heal_with_overheal(42.), 0.);
        assert_eq!(hitpoints.current, 84.);
        assert_eq!(hitpoints.max, 100.);

        assert_eq!(hitpoints.heal_with_overheal(42.), 26.);
        assert_eq!(hitpoints.current, 126.);
        assert_eq!(hitpoints.max, 100.);
    }

    #[test]
    fn damage() {
        let mut hitpoints = Hitpoints::from_max(100.);

        assert_eq!(hitpoints.damage(42.), 42.);
        assert_eq!(hitpoints.current, 100. - 42.);
        assert_eq!(hitpoints.max, 100.);

        assert_eq!(hitpoints.damage(42.), 42.);
        assert_eq!(hitpoints.current, 100. - 84.);
        assert_eq!(hitpoints.max, 100.);

        assert_eq!(hitpoints.damage(42.), 16.);
        assert_eq!(hitpoints.current, 0.);
        assert_eq!(hitpoints.max, 100.);
    }
}
