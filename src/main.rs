use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
mod color_palette;
use color_palette::PalColor;
mod body;
mod player;
use player::Player;
mod bullet;
mod camera;
mod collision;
mod debug;
mod gun;
mod iterable_enum;
mod lifetime;
mod planet;
mod space;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
            filter: "info,wgpu_core=warn,wgpu_hal=warn,pong=debug".to_string(),
            level: bevy::log::Level::TRACE,
            ..Default::default()
        }))
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins((
            debug::DebugPlugin {},
            camera::CameraPlugin {},
            space::SpacePlugin {},
            body::BodyPlugin {},
            planet::PlanetPlugin {},
            player::PlayerPlugin {},
            bullet::BulletPlugin {},
            gun::GunPlugin {},
            lifetime::LifetimePlugin {},
            collision::CollisionPlugin {},
        ))
        .add_systems(Startup, setup)
        .configure_sets(
            Update,
            (
                SystemUpdateSet::Main.before(SystemUpdateSet::Body),
                SystemUpdateSet::Body.before(SystemUpdateSet::Camera),
                SystemUpdateSet::Camera,
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {}

/// order that systems run in Update
#[derive(SystemSet, Debug, Clone, PartialEq, Hash, Eq)]
pub enum SystemUpdateSet {
    Main,
    Body,
    Camera,
}
