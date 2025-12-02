use std::ops::Neg;

use bevy::prelude::*;

use crate::{components::attributes, config, resources};

pub fn teleport_at_borders(
    borders: Res<resources::Borders>,
    mut query: Query<(&mut Transform, &attributes::Radius)>,
) {
    for (mut transform, circumradius) in &mut query {
        cycle_coords(
            &mut transform.translation.x,
            borders.left - circumradius.0,
            borders.right + circumradius.0,
            config::BORDER_TELEPORT_CORRECTION,
        );
        cycle_coords(
            &mut transform.translation.y,
            borders.bottom - circumradius.0,
            borders.top + circumradius.0,
            config::BORDER_TELEPORT_CORRECTION,
        );
    }
}

fn cycle_coords(coord: &mut f32, lower_bound: f32, upper_bound: f32, cycle_fix: f32) {
    if *coord < lower_bound {
        *coord = coord.neg() - cycle_fix;
    } else if upper_bound < *coord {
        *coord = coord.neg() + cycle_fix;
    }
}
