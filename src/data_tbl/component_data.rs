use super::raw;
use crate::{
    Health,
    ship_composition::{engine::Engine, gun::Gun},
};
use bevy::prelude::*;
use serde::Deserialize;

/// if fn new() of concrete is needed, use Raw version.
/// Otherwise, just use regular version
#[derive(Deserialize, Debug, Clone)]
pub enum ComponentData {
    Engine(raw::EngineRaw),
    Health(raw::HealthRaw),
    Gun(raw::GunRaw),
}

impl ComponentData {
    pub fn concrete(&self) -> ComponentConcrete {
        match self {
            Self::Engine(engine) => ComponentConcrete::Engine(engine.concrete()),
            Self::Health(health) => ComponentConcrete::Health(health.concrete()),
            Self::Gun(gun) => ComponentConcrete::Gun(gun.concrete()),
        }
    }
}

pub enum ComponentConcrete {
    Engine(Engine),
    Health(Health),
    Gun(Gun),
}
