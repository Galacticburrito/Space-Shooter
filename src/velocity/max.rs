use super::{AngularVelocity, Velocity};

use crate::{SystemUpdateSet, data_config::global_settings::GlobalSettings};
use bevy::prelude::*;

pub struct MaxPlugin {}
impl Plugin for MaxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                max_velocity,
                max_angular_velocity,
                add_max_to_velocity,
                add_max_to_angular_velocity,
            )
                .in_set(SystemUpdateSet::Early),
        )
        .register_type::<MaxVelocity>()
        .register_type::<MaxAngularVelocity>();
    }
}

#[derive(Reflect, Component)]
pub struct MaxVelocity(f32);

#[derive(Reflect, Component)]
pub struct MaxAngularVelocity(f32);

/// make Velocity magnitude not go above max
fn max_velocity(query: Query<(&mut Velocity, &MaxVelocity)>) {
    for (mut velocity, max_velocity) in query {
        velocity.0 = velocity.0.clamp_length_max(max_velocity.0);
    }
}

/// make AngularVelocity magnitude not go above max
fn max_angular_velocity(query: Query<(&mut AngularVelocity, &MaxAngularVelocity)>) {
    for (mut angular_velocity, max_angular_velocity) in query {
        angular_velocity.0 = angular_velocity.0.min(max_angular_velocity.0);
    }
}

/// when velocity created, add MaxVelocity equal to GlobalSettings.velocity_max
fn add_max_to_velocity(
    query: Query<Entity, (With<Velocity>, Without<MaxAngularVelocity>)>,
    g_settings: Res<GlobalSettings>,
    mut commands: Commands,
) {
    for entity in query {
        commands
            .entity(entity)
            .insert(MaxVelocity(g_settings.velocity_max));
    }
}

/// when angular velocity created, add MaxAngularVelocity equal to GlobalSettings.angular_velocity_max
fn add_max_to_angular_velocity(
    query: Query<Entity, (With<AngularVelocity>, Without<MaxAngularVelocity>)>,
    g_settings: Res<GlobalSettings>,
    mut commands: Commands,
) {
    for entity in query {
        commands
            .entity(entity)
            .insert(MaxAngularVelocity(g_settings.angular_velocity_max));
    }
}
