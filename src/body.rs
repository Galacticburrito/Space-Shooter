use crate::SystemUpdateSet;
use bevy::prelude::*;

// NOTE: Body doesn't work with parent/child relationships

pub struct BodyPlugin {}

impl Plugin for BodyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_body, update_rotational_body)
                .chain()
                .in_set(SystemUpdateSet::Body),
        )
        .register_type::<Body>()
        .register_type::<RotationBody>();
    }
}

#[derive(Clone, Component, Default, Reflect)]
#[require(Transform)]
pub struct Body {
    pub mass: f32,
    pub position: Vec2,
    pub velocity: Vec2,
}

impl Body {
    pub fn new(mass: f32, position: Vec2, velocity: Vec2) -> Self {
        Body {
            mass,
            velocity,
            position,
        }
    }

    pub fn global_position(&self, global_transform: &GlobalTransform) -> Vec2 {
        global_transform.translation().xy()
    }
}

#[derive(Clone, Component, Default, Reflect)]
pub struct RotationBody {
    pub rotation: f32,         // radians
    pub angular_velocity: f32, // radians
}

impl RotationBody {
    pub fn new(rotation: f32, angular_velocity: f32) -> Self {
        RotationBody {
            rotation,
            angular_velocity,
        }
    }

    /// convert rad angle to unit vector
    pub fn rotation_vector(&self) -> Vec2 {
        Vec2::new(self.rotation.cos(), self.rotation.sin())
    }
}

/// update transform posiiton equal to computed body transform
fn update_body(query: Query<(&mut Body, &mut Transform)>, time: Res<Time>) {
    for (mut body, mut transform) in query {
        body.position.x += body.velocity.x * time.delta_secs();
        body.position.y += body.velocity.y * time.delta_secs();
        transform.translation.x = body.position.x;
        transform.translation.y = body.position.y;
    }
}

/// update transform rotation based on RotationalBody rotation
fn update_rotational_body(mut query: Query<(&mut RotationBody, &mut Transform)>, time: Res<Time>) {
    for (mut rot_body, mut transform) in &mut query {
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
