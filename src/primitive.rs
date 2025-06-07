use bevy::prelude::*;
use serde::Deserialize;

// TODO: integrate with Graphic and Colliders
#[derive(Deserialize, Debug, Clone)]
pub enum Primitive {
    /// length, width
    Rectangle(f32, f32),
    Circle(Circle),
}

impl Primitive2d for Primitive {}

/// automatically convert Rectangle to Primitive when using ::from()
impl From<Rectangle> for Primitive {
    fn from(value: Rectangle) -> Self {
        Primitive::Rectangle(value.size().x, value.size().y)
    }
}

/// automatically convert Circle to Primitive when using ::from()
impl From<Circle> for Primitive {
    fn from(value: Circle) -> Self {
        Primitive::Circle(value)
    }
}

impl From<Primitive> for Mesh {
    fn from(value: Primitive) -> Self {
        match value {
            Primitive::Rectangle(x, y) => rectangle(x, y).into(),
            Primitive::Circle(cir) => cir.into(),
        }
    }
}

fn rectangle(x: f32, y: f32) -> Rectangle {
    Rectangle::from_size(Vec2::new(x, y))
}
