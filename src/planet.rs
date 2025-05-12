use crate::{
    Health,
    body::Body,
    collision::Collider,
    color_palette,
    space::{Gravitated, GravitySource},
};
use bevy::prelude::*;

pub struct PlanetPlugin {}

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn make_planet(
    body: Body,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
) -> impl Bundle {
    let radius = determine_radius(&body);
    let color = materials.add(color_palette::random_color());
    let shape = meshes.add(Circle::new(radius));
    (
        Name::new("Planet"),
        body.clone(),
        GravitySource {},
        Gravitated {},
        MeshMaterial2d(color),
        Mesh2d(shape),
        Collider::new_circle(radius),
        Health::new(1000.),
        Transform::from_translation(Vec3::ZERO),
    )
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // can't use spawn bundle for some reason (think b/c passing in mat and mesh)
    commands.spawn(make_planet(
        Body::new(1000., Vec2::new(-50., 0.), Vec2::new(0., -10.)),
        &mut materials,
        &mut meshes,
    ));
    commands.spawn(make_planet(
        Body::new(100., Vec2::new(100., 0.), Vec2::new(-10., 0.)),
        &mut materials,
        &mut meshes,
    ));
}

/// radius determined by mass
fn determine_radius(body: &Body) -> f32 {
    let modifier = 100.;
    let radius = (body.mass / modifier).trunc() + 1.;
    info!("radius is {}", radius);
    radius
}
