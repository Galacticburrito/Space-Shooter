use super::collider_type::ColliderType;
use bevy::prelude::*;
use serde::Deserialize;

#[derive(Component, Clone, Reflect)]
pub struct Collider {
    pub bounding: ColliderType,
    pub collision_layer: CollisionLayer,
    pub collides_with: Vec<CollisionLayer>,
}

impl Collider {
    pub fn new(bounding: ColliderType, collision_layer: CollisionLayer) -> Self {
        Collider {
            bounding,
            collides_with: collision_layer.collides_with(),
            collision_layer,
        }
    }

    /// makes sure, based on layer masks, that both colliders can in fact collide
    pub fn can_collide_with(&self, other_layer: &Collider) -> bool {
        self.collides_with.contains(&other_layer.collision_layer)
            && other_layer.collides_with.contains(&self.collision_layer)
    }
}

#[derive(Reflect, Clone, Deserialize, Debug, PartialEq)]
pub enum CollisionLayer {
    Ship,
    ShipComponent,
    Bullet,
    SonarPulse,
    Planet,
}

impl CollisionLayer {
    fn collides_with(&self) -> Vec<CollisionLayer> {
        match self {
            CollisionLayer::Ship => vec![
                CollisionLayer::Bullet,
                CollisionLayer::SonarPulse,
                CollisionLayer::Planet,
            ],
            CollisionLayer::ShipComponent => vec![CollisionLayer::Bullet],
            CollisionLayer::Bullet => vec![CollisionLayer::Ship, CollisionLayer::ShipComponent],
            CollisionLayer::SonarPulse => vec![CollisionLayer::Ship],
            CollisionLayer::Planet => vec![CollisionLayer::Ship],
        }
    }
}
