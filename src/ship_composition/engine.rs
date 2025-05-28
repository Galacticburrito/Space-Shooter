use crate::{
    SystemUpdateSet,
    body::{Body, RotationBody},
    health::Health,
    ship::{Ship, ShipType},
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
    pub fn new_old(ship_type: &ShipType, engine_type: &EngineType) -> Self {
        let max_thrust;
        let max_acceleration;
        let mut reverse_percent: f32 = 0.;

        match engine_type {
            EngineType::Main => match ship_type {
                ShipType::Interceptor => {
                    max_thrust = 5.;
                    max_acceleration = 3.;
                }
                ShipType::Gunship => {
                    max_thrust = 3.;
                    max_acceleration = 2.;
                }
                ShipType::MissileBoat => {
                    max_thrust = 2.;
                    max_acceleration = 1.;
                }
            },

            EngineType::Thruster => {
                reverse_percent = 1.;
                match ship_type {
                    ShipType::Interceptor => {
                        max_thrust = 0.01;
                        max_acceleration = 0.1;
                    }
                    ShipType::Gunship => {
                        max_thrust = 0.01;
                        max_acceleration = 0.1;
                    }
                    ShipType::MissileBoat => {
                        max_thrust = 0.01;
                        max_acceleration = 0.1;
                    }
                }
            }
        }
        Engine {
            engine_type: engine_type.clone(),
            max_thrust,
            max_acceleration,
            reverse_percent,
            ..Default::default()
        }
    }

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

    /// want to go at this thrust
    pub fn set_desired_thrust(&mut self, desired_thrust: f32) {
        let min_thrust = -self.max_thrust * self.reverse_percent;
        self.desired_thrust = desired_thrust.clamp(min_thrust, self.max_thrust);
    }

    /// want to go faster by this much
    pub fn add_desired_thrust(&mut self, desired_thrust: f32) {
        let min_thrust = -self.max_thrust * self.reverse_percent;
        self.desired_thrust += desired_thrust;
        self.desired_thrust = self.desired_thrust.clamp(min_thrust, self.max_thrust);
    }

    /// want no acceleration
    pub fn no_throttle(&mut self) {
        self.desired_thrust = 0.;
    }

    /// want to go as fast as possible
    pub fn full_throttle(&mut self) {
        self.desired_thrust = self.max_thrust;
    }

    pub fn min_throttle(&mut self) {
        self.desired_thrust = -self.max_thrust * self.reverse_percent;
    }

    /// want to keep current engine acceleration
    pub fn hold_throttle(&mut self) {
        self.desired_thrust = self.current_thrust;
    }

    /// each engine type implements their own way of working
    fn thrust(&self, ship_body: &mut Body, ship_rot_body: &mut RotationBody, deltatime: f32) {
        match self.engine_type {
            // thrusts ship forward
            EngineType::Main => {
                ship_body.velocity +=
                    ship_rot_body.rotation_vector() * self.current_thrust * deltatime;
            }
            // makes ship rotate
            EngineType::Thruster => {
                ship_rot_body.angular_velocity += self.current_thrust * deltatime;
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
    mut ship_query: Query<(&mut Body, &mut RotationBody, &Children), With<Ship>>,
    mut engine_query: Query<(&mut Engine, &Health)>,
    time: Res<Time>,
) {
    for (mut s_body, mut s_rot_body, children) in &mut ship_query {
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

                engine.thrust(&mut s_body, &mut s_rot_body, time.delta_secs());
            }
        }
    }
}
