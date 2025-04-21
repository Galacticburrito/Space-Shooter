use crate::SystemUpdateSet;
use bevy::prelude::*;

// NOTE: Body doesn't work with parent/child relationships

pub struct BodyPlugin {}

impl Plugin for BodyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (transform_from_body, transform_from_rotational_body).in_set(SystemUpdateSet::Body),
        );
        app.register_type::<Body>();
        app.register_type::<RotationBody>();
    }
}

#[derive(Clone, Component, Default, Reflect)]
#[require(Transform)]
pub struct Body {
    pub mass: f32,
    pub position: Vec2,
    pub velocity: Vec2,
}

#[derive(Clone, Component, Default, Reflect)]
pub struct RotationBody {
    pub rotation: f32,         // radians
    pub angular_velocity: f32, // radians
}

/// update transform posiiton equal to computed body transform
fn transform_from_body(mut query: Query<(&mut Transform, &mut Body)>, time: Res<Time>) {
    for (mut transform, mut body) in &mut query {
        transform.translation.x = body.position.x + (body.velocity.x * time.delta_secs());
        transform.translation.y = body.position.y + (body.velocity.y * time.delta_secs());
        body.position.x = transform.translation.x;
        body.position.y = transform.translation.y;
    }
}

/// update transform rotation based on RotationalBody rotation
fn transform_from_rotational_body(
    mut query: Query<(&mut Transform, &mut RotationBody)>,
    time: Res<Time>,
) {
    for (mut transform, mut rot_body) in &mut query {
        rot_body.rotation += rot_body.angular_velocity * time.delta_secs();
        transform.rotation = Quat::from_rotation_z(rot_body.rotation);
    }
}

#[derive(Component)]
#[require(Body)]
pub struct HistoryBody {
    past_bodies: Vec<Body>, // most recent to least
}

/// records current body to HistoryBody
fn record_body(mut query: Query<(&mut HistoryBody, &Body)>) {
    let history_len = 20;
    for (mut hist_body, body) in &mut query {
        hist_body.past_bodies.insert(0, body.clone());
        if hist_body.past_bodies.len() > history_len {
            hist_body.past_bodies.pop();
        }
    }
}
