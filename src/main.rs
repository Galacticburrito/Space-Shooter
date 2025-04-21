use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
mod color_palette;
use body::BodyPlugin;
use color_palette::PalColor;
mod body;
mod player;
use debug::DebugPlugin;
use planet::PlanetPlugin;
use player::{Player, PlayerPlugin};
mod camera;
use camera::CameraPlugin;
mod space;
use space::SpacePlugin;
mod debug;
mod gun;
use gun::GunPlugin;
mod iterable_enum;
mod planet;

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
            CameraPlugin {},
            DebugPlugin {},
            SpacePlugin {},
            PlayerPlugin {},
            PlanetPlugin {},
            BodyPlugin {},
            GunPlugin {},
        ))
        .add_systems(Startup, setup)
        .configure_sets(
            Update,
            (
                SystemUpdateSet::Main.before(SystemUpdateSet::Body),
                SystemUpdateSet::Body.before(SystemUpdateSet::Camera),
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
