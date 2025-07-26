use crate::{
    AppState, Health,
    collision::{
        collider::{Collider, CollisionLayer},
        collider_type::ColliderType,
    },
    color_palette::PalColor,
    graphic::Graphic,
    space::{
        gravity::{Gravitated, GravitySource},
        mass::Mass,
    },
    velocity::Velocity,
};
use bevy::prelude::*;

pub struct PlanetPlugin {}

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameReady), setup);
    }
}

fn make_planet(transform: Transform, mass: Mass, velocity: Velocity) -> impl Bundle {
    let radius = determine_radius(&mass);
    let graphic = Graphic::new(Circle::new(radius).into(), PalColor::Random.into());
    (
        Name::new("Planet"),
        transform,
        velocity.clone(),
        GravitySource {},
        Gravitated {},
        graphic,
        Collider::new(ColliderType::new_circle(radius), CollisionLayer::Planet),
        Health::new(1000.),
    )
}

fn setup(mut commands: Commands) {
    commands.spawn(make_planet(
        Transform::from_translation(Vec3::new(-50., 0., 0.)),
        Mass(10.),
        Velocity(Vec2::new(0., -10.)),
    ));
    commands.spawn(make_planet(
        Transform::from_translation(Vec3::new(100., 0., 0.)),
        Mass(100.),
        Velocity(Vec2::new(-10., 0.)),
    ));
}

/// radius determined by mass
fn determine_radius(mass: &Mass) -> f32 {
    let modifier = 100.;
    let radius = (mass.0 / modifier).trunc() + 1.;
    info!("radius is {}", radius);
    radius
}
