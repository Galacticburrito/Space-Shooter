use crate::{
    SystemUpdateSet,
    record::{Record, record_fixed_update, record_update},
    rotation,
    schedule::UpdateSchedule,
};
use bevy::prelude::*;

pub struct GlobalPlugin {}

impl Plugin for GlobalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                record_fixed_update::<GlobalTransform>,
                update_g_velocity,
                update_g_angular_velocity,
            )
                .in_set(SystemUpdateSet::Body),
        )
        .add_systems(
            Update,
            (record_update::<GlobalTransform>).in_set(SystemUpdateSet::Body),
        )
        .register_type::<GlobalVelocity>();
    }
}

/// read only
#[derive(Component, Default, Reflect)]
pub struct GlobalVelocity(pub Vec2);

/// get difference between last 2 positions to get velocity
fn update_g_velocity(
    query: Query<(&mut GlobalVelocity, &Record<GlobalTransform>)>,
    time: Res<Time>,
) {
    for (mut g_velocity, transform_record) in query {
        let fixed_update = UpdateSchedule::FixedUpdate;

        let transform_1 = transform_record.newest(&fixed_update);
        let transform_2 = transform_record
            .deq(&fixed_update)
            .get(1)
            .unwrap_or(transform_1);

        let delta_pos = transform_1.translation().xy() - transform_2.translation().xy();
        g_velocity.0 = delta_pos / time.delta_secs();
    }
}

/// wrapper of GlobalTransform
#[derive(Component, Default, Reflect)]
pub struct GlobalAngularVelocity(pub f32);

/// get difference between last 2 positions to get velocity
fn update_g_angular_velocity(
    query: Query<(&mut GlobalAngularVelocity, &Record<GlobalTransform>)>,
    time: Res<Time>,
) {
    for (mut g_velocity, rotation_record) in query {
        let fixed_update = UpdateSchedule::FixedUpdate;
        let rotation_1 = rotation_record.newest(&fixed_update);
        let rotation_2 = rotation_record
            .deq(&fixed_update)
            .get(1)
            .unwrap_or(rotation_1);

        let delta_velocity = rotation::quat_to_vec2(rotation_1.rotation())
            - rotation::quat_to_vec2(rotation_2.rotation());
        g_velocity.0 = (delta_velocity / time.delta_secs()).to_angle();
    }
}
