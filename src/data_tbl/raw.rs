use crate::{
    Health,
    collider::Collider,
    primitive::Primitive,
    ship_composition::{
        bullet::{self, BulletData},
        engine::{Engine, EngineType},
        gun::{self, Gun, GunData},
    },
};
use serde::Deserialize;

// raw is used to convert from RON into concrete component,
// if that component has special logic in their fn new()

#[derive(Debug, Deserialize, Clone)]
pub struct EngineRaw {
    engine_type: EngineType,
    reverse_percent: f32,
    max_thrust: f32,
    max_acceleration: f32,
}

impl EngineRaw {
    pub fn concrete(&self) -> Engine {
        Engine::new(
            &self.engine_type,
            self.max_thrust,
            self.max_acceleration,
            self.reverse_percent,
        )
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct HealthRaw {
    max: f32,
}

impl HealthRaw {
    pub fn concrete(&self) -> Health {
        Health::new(self.max)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct GunRaw {
    gun_data: GunData,
    bullet_data: BulletData,
}

impl GunRaw {
    pub fn concrete(&self) -> Gun {
        Gun::new(self.gun_data.clone(), self.bullet_data.clone())
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct ColliderRaw(Primitive);

impl ColliderRaw {
    pub fn concrete(&self) -> Collider {
        Collider::from(self.0.clone())
    }
}
