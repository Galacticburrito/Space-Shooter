use bevy::prelude::*;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, ecs::schedule::InternedSystemSet};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
mod color_palette;
use color_palette::PalColor;
mod player;
mod velocity;
use player::Player;
mod camera;
mod collision;
mod debug;
mod health;
use health::{Damage, Health};
mod ai;
mod collider;
mod data_tbl;
mod durability;
mod global;
mod graphic;
mod iterable_enum;
mod lifetime;
mod mass;
mod planet;
mod primitive;
mod record;
mod rotation;
mod schedule;
mod ship;
mod ship_composition;
mod space;

#[derive(Reflect, Resource, Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    LoadingAssets,
    GameReady,
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
        filter: "info,wgpu_core=warn,wgpu_hal=warn,pong=debug".to_string(),
        level: bevy::log::Level::TRACE,
        ..Default::default()
    }))
    // external
    .add_plugins((
        EguiPlugin {
            enable_multipass_for_primary_context: true,
        },
        FrameTimeDiagnosticsPlugin::default(),
        WorldInspectorPlugin::new(),
    ))
    // internal
    .add_plugins((
        debug::DebugPlugin {},
        camera::CameraPlugin {},
        space::SpacePlugin {},
        velocity::VelocityPlugin {},
        global::GlobalPlugin {},
        planet::PlanetPlugin {},
        player::PlayerPlugin {},
        lifetime::LifetimePlugin {},
        collision::CollisionPlugin {},
        health::HealthPlugin {},
        ai::AiPlugin {},
        ship::ShipPlugin {},
        data_tbl::TablePlugin {},
        ship_composition::ShipCompositionPlugin {},
        graphic::GraphicPlugin {},
    ))
    .configure_sets(Update, SystemUpdateSet::configuration())
    .configure_sets(FixedUpdate, SystemUpdateSet::configuration())
    .init_state::<AppState>();

    app.run();
}

/// order that systems run in Update or FixedUpdate
/// runs in AssetState::GameReady
#[derive(SystemSet, Debug, Clone, PartialEq, Hash, Eq)]
pub enum SystemUpdateSet {
    Main,
    Body,
    Camera,
}

impl SystemUpdateSet {
    fn configuration() -> (
        bevy::ecs::schedule::ScheduleConfigs<bevy::ecs::intern::Interned<dyn SystemSet>>,
        bevy::ecs::schedule::ScheduleConfigs<bevy::ecs::intern::Interned<dyn SystemSet>>,
        bevy::ecs::schedule::ScheduleConfigs<bevy::ecs::intern::Interned<dyn SystemSet>>,
    ) {
        (
            SystemUpdateSet::Main
                .before(SystemUpdateSet::Body)
                .run_if(in_state(AppState::GameReady)),
            SystemUpdateSet::Body
                .before(SystemUpdateSet::Camera)
                .run_if(in_state(AppState::GameReady)),
            SystemUpdateSet::Camera.run_if(in_state(AppState::GameReady)),
        )
    }
}
