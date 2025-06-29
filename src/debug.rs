use crate::{
    collision::{collider::Collider, collider_type::ColliderType},
    color_palette::PalColor,
};
use bevy::{
    app::App, gizmos::gizmos::Gizmos, math::bounding::BoundingVolume, prelude::*,
    reflect::GetTypeRegistration,
};
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

pub struct DebugPlugin {}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        // don't care when these run
        app.add_systems(Startup, draw_grid)
            .add_systems(Update, (show_rotation, show_colliders, pause_game));
    }
}

// APP HELPERS

pub fn insert_inspectable_resource<T: Resource + Default + Reflect + GetTypeRegistration>(
    app: &mut App,
    initial_val: Option<T>,
    window_display: bool,
) {
    app.insert_resource::<T>(initial_val.unwrap_or_default())
        .register_type::<T>();
    if window_display {
        app.add_plugins(ResourceInspectorPlugin::<T>::default());
    }
}

// GIZMOS STUFF

fn draw_grid(mut gizmo_assets: ResMut<Assets<GizmoAsset>>, mut commands: Commands) {
    let mut gizmo = GizmoAsset::default();

    let grid_color = PalColor::White;
    let grid_size = 100_000.;
    let grid_spacing = 100.;
    let half_grid_size = grid_size / 2.;

    // lines along x axis
    for x in (-half_grid_size as i32..=half_grid_size as i32).step_by(grid_spacing as usize) {
        gizmo.line_2d(
            Vec2::new(x as f32, -half_grid_size),
            Vec2::new(x as f32, half_grid_size),
            grid_color,
        );
    }

    // lines along y axis
    for y in (-half_grid_size as i32..=half_grid_size as i32).step_by(grid_spacing as usize) {
        gizmo.line_2d(
            Vec2::new(-half_grid_size, y as f32),
            Vec2::new(half_grid_size, y as f32),
            grid_color,
        );
    }

    // so is persistant
    commands.spawn(Gizmo {
        handle: gizmo_assets.add(gizmo),
        ..default()
    });
}

fn show_rotation(query: Query<&Transform>, mut gizmo: Gizmos) {
    for transform in &query {
        // quat to x in radians
        let angle = transform.rotation.z.atan2(transform.rotation.w) * 2.;

        let up_vector = Vec2::new(angle.cos(), angle.sin());

        let line_end = transform.translation.xy() + up_vector * 20.0;

        gizmo.line_2d(transform.translation.xy(), line_end, PalColor::Black);
    }
}

fn show_colliders(query: Query<(&GlobalTransform, &Collider, &Visibility)>, mut gizmo: Gizmos) {
    for (transform, collider, visibility) in &query {
        let center = transform.translation().xy();
        let angle_rad = transform.rotation().to_euler(EulerRot::XYZ).2;
        match collider.bounding {
            ColliderType::Rectangle(aabb) => {
                gizmo.rect_2d(
                    Isometry2d::new(center, Rot2::radians(angle_rad)),
                    aabb.half_size() * 2.,
                    PalColor::Green,
                );
            }
            ColliderType::Circle(bounding_circle) => {
                gizmo.circle_2d(
                    Isometry2d::from_translation(center),
                    bounding_circle.radius(),
                    PalColor::Green,
                );
            }
            ColliderType::Ring(inner, outer) => {
                gizmo.circle_2d(
                    Isometry2d::from_translation(center),
                    inner.radius(),
                    PalColor::Green,
                );
                gizmo.circle_2d(
                    Isometry2d::from_translation(center),
                    outer.radius(),
                    PalColor::Green,
                );
            }
        };
    }
}

fn pause_game(mut time: ResMut<Time<Virtual>>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::KeyP) {
        if time.is_paused() {
            time.unpause();
        } else {
            time.pause();
        }
    }
}
