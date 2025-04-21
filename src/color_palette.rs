use crate::iterable_enum::IterableEnum;
use bevy::prelude::*;

#[derive(Clone, Copy)]
pub enum PalColor {
    Black,
    White,
    Red,
    Green,
    Blue,
}

impl From<PalColor> for Color {
    fn from(pal_color: PalColor) -> Self {
        match pal_color {
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
