use crate::color_palette::PalColor;
use bevy::{app::App, gizmos::gizmos::Gizmos, prelude::*, reflect::GetTypeRegistration};
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

pub struct DebugPlugin {}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (draw_grid, show_rotation));
    }
}

// APP HELPERS

pub fn insert_inspectable_resource<T: Resource + Default + Reflect + GetTypeRegistration>(
    app: &mut App,
    initial_val: Option<T>,
) {
    app.insert_resource::<T>(initial_val.unwrap_or(T::default()))
        .register_type::<T>()
        .add_plugins(ResourceInspectorPlugin::<T>::default());
}

// GIZMOS STUFF

fn draw_grid(mut gizmos: Gizmos) {
    let grid_color = PalColor::White;
    let grid_size = 100_000.;
    let grid_spacing = 100.;
    let half_grid_size = grid_size / 2.;

    // lines along x axis
    for x in (-half_grid_size as i32..=half_grid_size as i32).step_by(grid_spacing as usize) {
        gizmos.line_2d(
            Vec2::new(x as f32, -half_grid_size),
            Vec2::new(x as f32, half_grid_size),
            grid_color,
        );
    }

    // lines along y axis
    for y in (-half_grid_size as i32..=half_grid_size as i32).step_by(grid_spacing as usize) {
        gizmos.line_2d(
            Vec2::new(-half_grid_size, y as f32),
            Vec2::new(half_grid_size, y as f32),
            grid_color,
        );
    }
}

fn show_rotation(query: Query<&Transform>, mut gizmos: Gizmos) {
    for transform in &query {
        let angle = transform.rotation.z.atan2(transform.rotation.w) * 2.; // quat to x in radians

        let up_vector = Vec2::new(angle.cos(), angle.sin());

        let line_end = transform.translation.xy() + up_vector * 20.0;

        gizmos.line_2d(transform.translation.xy(), line_end, PalColor::Black);
    }
}
