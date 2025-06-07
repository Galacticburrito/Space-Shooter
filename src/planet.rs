use crate::{
    AppState, Health,
    collider::Collider,
    color_palette,
    mass::Mass,
    space::{Gravitated, GravitySource},
    velocity::Velocity,
};
use bevy::prelude::*;

pub struct PlanetPlugin {}

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameReady), setup);
    }
}

fn make_planet(
    transform: Transform,
    mass: Mass,
    velocity: Velocity,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
) -> impl Bundle {
    let radius = determine_radius(&mass);
    let color = materials.add(color_palette::random_color());
    let shape = meshes.add(Circle::new(radius));
    (
        Name::new("Planet"),
        transform,
        velocity.clone(),
        GravitySource {},
        Gravitated {},
        MeshMaterial2d(color),
        Mesh2d(shape),
        Collider::new_circle(radius),
        Health::new(1000.),
    )
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // can't use spawn bundle for some reason (think b/c passing in mat and mesh)
    commands.spawn(make_planet(
        Transform::from_translation(Vec3::new(-50., 0., 0.)),
        Mass(10.),
        Velocity(Vec2::new(0., -10.)),
        &mut materials,
        &mut meshes,
    ));
    commands.spawn(make_planet(
        Transform::from_translation(Vec3::new(100., 0., 0.)),
        Mass(100.),
        Velocity(Vec2::new(-10., 0.)),
        &mut materials,
        &mut meshes,
    ));
}

/// radius determined by mass
fn determine_radius(mass: &Mass) -> f32 {
    let modifier = 100.;
    let radius = (mass.0 / modifier).trunc() + 1.;
    info!("radius is {}", radius);
    radius
}
