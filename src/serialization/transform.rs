use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Reflect, Clone, Serialize, Deserialize, PartialEq)]
pub struct SerializeableTransform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl From<Transform> for SerializeableTransform {
    fn from(value: Transform) -> Self {
        Self {
            translation: value.translation,
            rotation: value.rotation,
            scale: value.scale,
        }
    }
}

impl From<SerializeableTransform> for Transform {
    fn from(value: SerializeableTransform) -> Self {
        Transform {
            translation: value.translation,
            rotation: value.rotation,
            scale: value.scale,
        }
    }
}
