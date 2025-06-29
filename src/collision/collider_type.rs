use crate::primitive::Primitive;
use bevy::math::bounding::{Aabb2d, BoundingCircle, BoundingVolume};
use bevy::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub enum ColliderType {
    Rectangle(Aabb2d),
    Circle(BoundingCircle),
    /// inner, outer
    Ring(BoundingCircle, BoundingCircle),
}

impl ColliderType {
    pub fn new_rect(width: f32, height: f32) -> Self {
        let half_width = width / 2.0;
        let half_height = height / 2.0;
        ColliderType::Rectangle(Aabb2d::new(Vec2::ZERO, Vec2::new(half_width, half_height)))
    }

    pub fn new_circle(radius: f32) -> Self {
        ColliderType::Circle(BoundingCircle::new(Vec2::ZERO, radius))
    }

    pub fn new_annulus(inner: f32, outer: f32) -> Self {
        ColliderType::Ring(
            BoundingCircle::new(Vec2::ZERO, inner),
            BoundingCircle::new(Vec2::ZERO, outer),
        )
    }

    pub fn convert_to_global(&self, g_translation: Vec3) -> ColliderType {
        match self {
            ColliderType::Rectangle(aabb) => ColliderType::Rectangle(Aabb2d::new(
                aabb.center() + g_translation.xy(),
                aabb.half_size(),
            )),
            ColliderType::Circle(circle) => ColliderType::Circle(BoundingCircle::new(
                circle.center + g_translation.xy(),
                circle.radius(),
            )),
            ColliderType::Ring(inner, outer) => {
                let center = inner.center + g_translation.xy();
                ColliderType::Ring(
                    BoundingCircle::new(center, inner.radius()),
                    BoundingCircle::new(center, outer.radius()),
                )
            }
        }
    }
}

impl From<Primitive> for ColliderType {
    fn from(value: Primitive) -> Self {
        match value {
            Primitive::Rectangle(x, y) => ColliderType::new_rect(x, y),
            Primitive::Circle(cir) => ColliderType::new_circle(cir.radius),
            Primitive::Ring(ring) => {
                ColliderType::new_annulus(ring.inner_circle.radius, ring.outer_circle.radius)
            }
        }
    }
}

impl From<Rectangle> for ColliderType {
    fn from(value: Rectangle) -> Self {
        ColliderType::from(Primitive::from(value))
    }
}

impl From<Circle> for ColliderType {
    fn from(value: Circle) -> Self {
        ColliderType::from(Primitive::from(value))
    }
}

impl From<Annulus> for ColliderType {
    fn from(value: Annulus) -> Self {
        ColliderType::from(Primitive::from(value))
    }
}
