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
                .in_set(SystemUpdateSet::Early),
        )
        .add_systems(
            Update,
            (record_update::<GlobalTransform>, record_update::<Transform>)
                .in_set(SystemUpdateSet::Early),
        )
        .register_type::<GlobalVelocity>();
    }
}

/// read only
#[derive(Component, serde::Deserialize, Clone, Debug, Default, Reflect)]
pub struct GlobalVelocity(pub Vec2);

/// get difference between last 2 positions to get velocity
fn update_g_velocity(
    query: Query<(&mut GlobalVelocity, &Record<GlobalTransform>)>,
    time: Res<Time>,
) {
    for (mut g_velocity, transform_record) in query {
        let records = transform_record.newest_number(2, &UpdateSchedule::Update);

        let transform_1 = records[0];
        let transform_2 = records.get(1).map_or(transform_1, |t| *t);

        let delta_pos = transform_1.val.translation().xy() - transform_2.val.translation().xy();
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
        let records = rotation_record.newest_number(2, &UpdateSchedule::Update);

        let delta_velocity = rotation::quat_to_vec2(records[0].val.rotation())
            - rotation::quat_to_vec2(records.get(1).unwrap_or(&records[0]).val.rotation());

        g_velocity.0 = (delta_velocity / time.delta_secs()).to_angle();
    }
}
