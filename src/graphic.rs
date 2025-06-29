use bevy::prelude::*;
use serde::Deserialize;

use crate::{SystemUpdateSet, color_palette::PalColor, primitive::Primitive};

pub struct GraphicPlugin {}

impl Plugin for GraphicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, graphic_changed.in_set(SystemUpdateSet::Main));
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
    pub fn new(shape: Primitive, color: PalColor) -> Self {
        Graphic { shape, color }
    }

    /// replaces current graphic color with new one
    pub fn replace_color(&mut self, color: PalColor) {
        self.color = color;
    }

    /// replaces current shape with new one
    pub fn replace_shape(&mut self, shape: Primitive) {
        self.shape = shape;
    }

    /// adds the shape and color to assets
    /// can run in command.spawn() to add necessary components
    fn add_mesh_and_material(
        &self,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> (Mesh2d, MeshMaterial2d<ColorMaterial>) {
        let mesh = meshes.add(self.shape.clone());
        let material = materials.add(self.color);

        (Mesh2d(mesh), MeshMaterial2d(material))
    }
}

/// adds required components for rendering when a Graphic is changed
/// allows changes to meshes and materials during runtime by changing Graphic itself
fn graphic_changed(
    query: Query<(Entity, &Graphic), Changed<Graphic>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    for (entity, graphic) in query {
        let (mesh, material) = graphic.add_mesh_and_material(&mut meshes, &mut materials);

        commands.entity(entity).try_insert((mesh, material));
    }
}
