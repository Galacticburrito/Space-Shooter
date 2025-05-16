use crate::{SystemUpdateSet, body::Body, debug};
use bevy::prelude::*;

pub struct SpacePlugin {}

impl Plugin for SpacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (apply_gravity).in_set(SystemUpdateSet::Main));
        debug::insert_inspectable_resource(app, Some(GravityConst(100.)));
    }
}

/// affected by gravity
#[derive(Component)]
#[require(Body)]
pub struct Gravitated {}

/// affect gravity
#[derive(Component)]
#[require(Body)]
pub struct GravitySource {}

#[derive(Reflect, Resource, Default)]
#[reflect(Resource)]
pub struct GravityConst(f32);

fn apply_gravity(
    mut param_set: ParamSet<(
        Query<&Body, With<GravitySource>>,  // sources
        Query<&mut Body, With<Gravitated>>, // affected
    )>,
    gravity_const: Res<GravityConst>,
    time: Res<Time>,
) {
    let sources: Vec<(Vec2, f32)> = param_set
        .p0()
        .iter()
        .map(|body| (body.position, body.mass))
        .collect();

    for source in &sources {
        for mut affected in &mut param_set.p1() {
            if source.0 == affected.position {
                // same object
                continue;
            }

            // compute acceleration of affected (mass cancels out)
            let acceleration =
                gravity_const.0 * (source.1) / source.0.distance_squared(affected.position);

            // point acceleration vector to source, then add to affected velocity
            let dir = (source.0 - affected.position).normalize_or(Vec2::new(0., 0.));
            affected.velocity += dir * acceleration * time.delta_secs();
        }
    }
}
