use crate::{
    SystemUpdateSet,
    health::Health,
    rotation,
    ship::{Ship, ShipType},
    velocity::{AngularVelocity, Velocity},
};
use bevy::prelude::*;
use serde::Deserialize;

pub struct EnginePlugin {}

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, engine_thrust.in_set(SystemUpdateSet::Main));
        app.register_type::<Engine>();
    }
}

#[derive(Component, Clone, Debug, Reflect, Deserialize)]
pub struct Engine {
    /// for now, only important for implementers
    pub engine_type: EngineType,

    /// % of forward movement that engine can do in reverse direction
    /// defaults to 0% (engine cant move backwards)
    reverse_percent: f32,

    /// max thrust healthy engine can perform
    max_thrust: f32,
    current_thrust: f32,

    /// max change in thrust healthy engine can perform
    max_acceleration: f32,

    desired_thrust: f32,
}

#[derive(Reflect, Clone, Debug, Deserialize, PartialEq)]
pub enum EngineType {
    /// ship go forward
    Main,
    /// ship rotation
    Thruster,
}

impl Engine {
    pub fn new(
        engine_type: &EngineType,
        max_thrust: f32,
        max_acceleration: f32,
        reverse_percent: f32,
    ) -> Self {
        Engine {
            engine_type: engine_type.clone(),
            max_thrust,
            max_acceleration,
            reverse_percent,
            ..Default::default()
        }
    }

    /// want to go at given acceleration
    pub fn set_throttle(&mut self, desired_thrust: f32) {
        let min_thrust = -self.max_thrust * self.reverse_percent;
        self.desired_thrust = desired_thrust.clamp(min_thrust, self.max_thrust);
    }

    /// want to go faster by this much
    pub fn add_throttle(&mut self, desired_thrust: f32) {
        let min_thrust = -self.max_thrust * self.reverse_percent;
        self.desired_thrust += desired_thrust;
        self.desired_thrust = self.desired_thrust.clamp(min_thrust, self.max_thrust);
    }

    /// want no acceleration, set thrust to 0
    pub fn no_throttle(&mut self) {
        self.desired_thrust = 0.;
    }

    /// want to go as fast as possible
    pub fn full_throttle(&mut self) {
        self.desired_thrust = self.max_thrust;
    }

    /// want to go backwards at max (or 0 thrust if can't go backwards)
    pub fn min_throttle(&mut self) {
        self.desired_thrust = -self.max_thrust * self.reverse_percent;
    }

    /// want to keep current engine acceleration
    pub fn hold_throttle(&mut self) {
        self.desired_thrust = self.current_thrust;
    }

    /// return current thrust
    pub fn current_thrust(&self) -> f32 {
        self.current_thrust
    }

    /// each engine type implements their own way of working
    fn thrust(
        &self,
        transform: &Transform,
        velocity: &mut Velocity,
        angular_velocity: &mut AngularVelocity,
        deltatime: f32,
    ) {
        match self.engine_type {
            // thrusts ship forward
            EngineType::Main => {
                velocity.0 +=
                    rotation::quat_to_vec2(transform.rotation) * self.current_thrust * deltatime;
            }
            // makes ship rotate
            EngineType::Thruster => {
                angular_velocity.0 += self.current_thrust * deltatime;
            }
        }
    }
}

impl Default for Engine {
    fn default() -> Self {
        Engine {
            engine_type: EngineType::Main,
            reverse_percent: 0.,
            max_thrust: 10.,
            current_thrust: 0.,
            max_acceleration: 10.,
            desired_thrust: 0.,
        }
    }
}

/// calculate and apply actual thrust based on desired thrust, engine capabilities, and health
fn engine_thrust(
    mut ship_query: Query<(&Transform, &mut Velocity, &mut AngularVelocity, &Children), With<Ship>>,
    mut engine_query: Query<(&mut Engine, &Health)>,
    time: Res<Time>,
) {
    for (s_transform, mut s_velocity, mut s_angular_velocity, children) in &mut ship_query {
        for child in children {
            if let Ok((mut engine, health)) = engine_query.get_mut(*child) {
                // calculate achievable delta thrust based on Health
                let available_acceleration = engine.max_acceleration * health.percent();
                let thrust_difference = engine.desired_thrust - engine.current_thrust;

                let actual_acceleration = if thrust_difference.abs() > 0. {
                    thrust_difference.signum() * available_acceleration * time.delta_secs()
                } else {
                    0.
                };

                engine.current_thrust += actual_acceleration.clamp(
                    -engine.max_thrust * health.percent(),
                    engine.max_thrust * health.percent(),
                );

                engine.thrust(
                    s_transform,
                    &mut s_velocity,
                    &mut s_angular_velocity,
                    time.delta_secs(),
                );
            }
        }
    }
}
