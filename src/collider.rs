use crate::primitive::Primitive;
use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume},
    prelude::*,
};

#[derive(Component, Clone)]
pub enum Collider {
    Rectangle(Aabb2d),
    Circle(BoundingCircle),
}

impl Collider {
    pub fn new_rect(width: f32, height: f32) -> Self {
        let half_width = width / 2.0;
        let half_height = height / 2.0;
        Collider::Rectangle(Aabb2d::new(Vec2::ZERO, Vec2::new(half_width, half_height)))
    }

    pub fn new_circle(radius: f32) -> Self {
        Collider::Circle(BoundingCircle::new(Vec2::ZERO, radius))
    }

    pub fn convert_to_global(&self, g_translation: Vec3) -> Collider {
        match self {
            Collider::Rectangle(aabb) => Collider::Rectangle(Aabb2d::new(
                aabb.center() + g_translation.xy(),
                aabb.half_size(),
            )),
            Collider::Circle(circle) => Collider::Circle(BoundingCircle::new(
                circle.center + g_translation.xy(),
                circle.radius(),
            )),
        }
    }
}

impl From<Primitive> for Collider {
    fn from(value: Primitive) -> Self {
        match value {
            Primitive::Rectangle(x, y) => Collider::new_rect(x, y),
            Primitive::Circle(cir) => Collider::new_circle(cir.radius),
        }
    }
}

impl From<Rectangle> for Collider {
    fn from(value: Rectangle) -> Self {
        Collider::from(Primitive::from(value))
    }
}

impl From<Circle> for Collider {
    fn from(value: Circle) -> Self {
        Collider::from(Primitive::from(value))
    }
}
