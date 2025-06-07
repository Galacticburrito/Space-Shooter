use bevy::prelude::*;
use serde::Deserialize;

use crate::{collider::Collider, color_palette::PalColor, primitive::Primitive};

pub struct GraphicPlugin {}

impl Plugin for GraphicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, add_mesh_and_material);
    }
}

/// container for mesh, color, and associated data
/// when added to entity, automatically adds required components
#[derive(Component, Clone, Deserialize, Debug)]
pub struct Graphic {
    shape: Primitive,
    color: PalColor,
}

impl Graphic {
    pub fn new(shape: Primitive, color: PalColor) -> Graphic {
        Graphic { shape, color }
    }

    /// adds the shape and color to assets
    /// can run in command.spawn() to add necessary components
    pub fn add_mesh_and_material(
        &self,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> (Mesh2d, MeshMaterial2d<ColorMaterial>) {
        let mesh = meshes.add(self.shape.clone());
        let material = materials.add(self.color);

        (Mesh2d(mesh), MeshMaterial2d(material))
    }
}

/// adds required components for rendering when a Graphic is added
fn add_mesh_and_material(
    query: Query<(Entity, &Graphic), Added<Graphic>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    for (entity, graphic) in query {
        let mesh = meshes.add(graphic.shape.clone());
        let material = materials.add(graphic.color);

        commands
            .entity(entity)
            .insert((Mesh2d(mesh), MeshMaterial2d(material)));
    }
}
