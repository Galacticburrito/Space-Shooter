use crate::{
    SystemUpdateSet,
    global::{GlobalAngularVelocity, GlobalVelocity},
    rotation,
};
use bevy::prelude::*;

// NOTE: Body doesn't work with parent/child relationships

pub struct VelocityPlugin {}

impl Plugin for VelocityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_velocity, update_angular_velocity).in_set(SystemUpdateSet::Body),
        )
        .add_observer(add_velocity_to_transform)
        .add_observer(add_angular_velocity_to_transform)
        .register_type::<Velocity>()
        .register_type::<AngularVelocity>();
    }
}

/// NOTE: may need GlobalTransform and GlobalVelocity if child
#[derive(Clone, Component, Default, Reflect)]
#[require(Transform, GlobalVelocity)]
pub struct Velocity(pub Vec2);

impl Velocity {
    pub const ZERO: Velocity = Velocity(Vec2::ZERO);
}

#[derive(Clone, Component, Default, Reflect)]
#[require(Transform, GlobalAngularVelocity)]
pub struct AngularVelocity(pub f32);

/// update transform posiiton equal to computed body transform
fn update_velocity(query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query {
        transform.translation.x += velocity.0.x * time.delta_secs();
        transform.translation.y += velocity.0.y * time.delta_secs();
    }
}

/// update transform rotation based on RotationalBody rotation
fn update_angular_velocity(mut query: Query<(&AngularVelocity, &mut Transform)>, time: Res<Time>) {
    for (angular_velocity, mut transform) in &mut query {
        let rot_change = angular_velocity.0 * time.delta_secs();
        transform.rotation *= rotation::rad_to_quat(rot_change);
    }
}

/// same thing as adding a #[require(Velocity)] to Transform component
fn add_velocity_to_transform(
    trigger: Trigger<OnAdd, Transform>,
    query: Query<Entity, With<Velocity>>,
    mut commands: Commands,
) {
    let entity = trigger.target();

    if query.get(entity).is_err() {
        commands.entity(entity).insert(Velocity::ZERO);
    }
}

/// same thing as adding a #[require(AngularVelocity)] to Transform component
fn add_angular_velocity_to_transform(
    trigger: Trigger<OnAdd, Transform>,
    query: Query<Entity, With<AngularVelocity>>,
    mut commands: Commands,
) {
    let entity = trigger.target();

    if query.get(entity).is_err() {
        commands.entity(entity).insert(AngularVelocity(0.));
    }
}
