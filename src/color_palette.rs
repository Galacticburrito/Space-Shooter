use crate::iterable_enum::IterableEnum;
use bevy::prelude::*;
use serde::Deserialize;

#[derive(Debug, Reflect, Deserialize, Clone, Copy, PartialEq)]
pub enum PalColor {
    Random,
    Custom(Color),
    Black,
    White,
    Red,
    Green,
    Blue,
}

impl PalColor {
    pub fn mix(&self, other: &PalColor, percent: f32) -> PalColor {
        PalColor::Custom(Color::from(self.clone()).mix(&Color::from(other.clone()), percent))
    }
}

impl From<PalColor> for Color {
    fn from(pal_color: PalColor) -> Self {
        match pal_color {
            PalColor::Random => random_color(),
            PalColor::Custom(color) => color,
            PalColor::Black => Color::BLACK,
            PalColor::White => Color::WHITE,
            PalColor::Red => Color::srgb(1., 0., 0.),
            PalColor::Green => Color::srgb(0., 1., 0.),
            PalColor::Blue => Color::srgb(0., 0., 1.),
        }
    }
}

impl From<PalColor> for ColorMaterial {
    fn from(pal_color: PalColor) -> Self {
        ColorMaterial::from(Color::from(pal_color))
    }
}

impl IterableEnum for PalColor {
    type Iter = std::array::IntoIter<PalColor, 5>;

    fn iter() -> Self::Iter {
        [
            PalColor::Black,
            PalColor::White,
            PalColor::Red,
            PalColor::Green,
            PalColor::Blue,
        ]
        .into_iter()
    }
}

pub fn random_color() -> Color {
    Color::srgb(rand::random(), rand::random(), rand::random())
}
