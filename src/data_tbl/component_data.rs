use super::raw;
use crate::{
    Health,
    collider::Collider,
    graphic::Graphic,
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
    Graphic(Graphic),
    Collider(raw::ColliderRaw),
}

impl ComponentData {
    pub fn concrete(&self) -> ComponentConcrete {
        match self {
            Self::Engine(engine) => ComponentConcrete::Engine(engine.concrete()),
            Self::Health(health) => ComponentConcrete::Health(health.concrete()),
            Self::Gun(gun) => ComponentConcrete::Gun(gun.concrete()),
            Self::Graphic(graphic) => ComponentConcrete::Graphic(graphic.clone()),
            Self::Collider(collider) => ComponentConcrete::Collider(collider.concrete()),
        }
    }
}

pub enum ComponentConcrete {
    Engine(Engine),
    Health(Health),
    Gun(Gun),
    Graphic(Graphic),
    Collider(Collider),
}

pub fn add_components_to_entity(entity: &mut EntityCommands, components: &[ComponentData]) {
    for component in components {
        let concrete_component = component.concrete();
        match concrete_component {
            ComponentConcrete::Engine(engine) => {
                entity.insert(engine.clone());
            }
            ComponentConcrete::Health(health) => {
                entity.insert(health.clone());
            }
            ComponentConcrete::Gun(gun) => {
                entity.insert(gun.clone());
            }
            ComponentConcrete::Graphic(graphic) => {
                entity.insert(graphic.clone());
            }
            ComponentConcrete::Collider(collider) => {
                entity.insert(collider.clone());
            }
        }
    }
}
