use bevy::prelude::*;
use serde::Deserialize;

// TODO: integrate with Graphic and Colliders
#[derive(Deserialize, Debug, Clone)]
pub enum Primitive {
    /// length, width
    Rectangle(f32, f32),
    Circle(Circle),
    Ring(Annulus),
}

impl Primitive2d for Primitive {}

impl From<Rectangle> for Primitive {
    fn from(value: Rectangle) -> Self {
        Primitive::Rectangle(value.size().x, value.size().y)
    }
}

impl From<Circle> for Primitive {
    fn from(value: Circle) -> Self {
        Primitive::Circle(value)
    }
}

impl From<Annulus> for Primitive {
    fn from(value: Annulus) -> Self {
        Primitive::Ring(value)
    }
}

impl From<Primitive> for Mesh {
    fn from(value: Primitive) -> Self {
        match value {
            Primitive::Rectangle(x, y) => rectangle(x, y).into(),
            Primitive::Circle(cir) => cir.into(),
            Primitive::Ring(ring) => ring.into(),
        }
    }
}

fn rectangle(x: f32, y: f32) -> Rectangle {
    Rectangle::from_size(Vec2::new(x, y))
}
