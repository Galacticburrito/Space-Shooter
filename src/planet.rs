use crate::{
    PalColor,
    body::Body,
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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // planet 1
    let color = materials.add(color_palette::random_color());
    let body = Body {
        mass: 1000.,
        position: Vec2::new(-50., 0.),
        velocity: Vec2::new(0., -10.),
    };
    let shape = meshes.add(Circle::new(determine_radius(&body)));
    commands.spawn((
        Name::new("Planet"),
        body.clone(),
        GravitySource {},
        Gravitated {},
        MeshMaterial2d(color),
        Mesh2d(shape),
        Transform::from_translation(Vec3::ZERO),
    ));

    // planet 2
    let color = materials.add(PalColor::Blue);
    let body = Body {
        mass: 100.,
        position: Vec2::new(100., 0.),
        velocity: Vec2::new(-10., 0.),
    };
    let shape = meshes.add(Circle::new(determine_radius(&body)));
    commands.spawn((
        Name::new("Planet"),
        body,
        GravitySource {},
        Gravitated {},
        MeshMaterial2d(color),
        Mesh2d(shape),
        Transform::from_translation(Vec3::ZERO),
    ));
}

/// radius determined by mass
fn determine_radius(body: &Body) -> f32 {
    let modifier = 100.;
    let radius = (body.mass / modifier).trunc() + 1.;
    info!("radius is {}", radius);
    radius
}
