use bevy::ecs::intern::Interned;
use bevy::prelude::*;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, ecs::schedule::ScheduleConfigs};
use bevy_behave::prelude::BehavePlugin;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
mod color_palette;
mod player;
mod velocity;
use player::Player;
mod camera;
mod debug;
mod health;
use health::{Damage, Health};
mod ai;
mod collision;
mod data_config;
mod durability;
mod graphic;
mod iterable_enum;
mod lifetime;
mod particle_system;
mod primitive;
mod record;
mod rotation;
mod schedule;
mod serialization;
mod ship;
mod ship_composition;
mod space;
mod ui;

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
        BehavePlugin::new(Update),
    ))
    // internal
    .add_plugins((
        camera::CameraPlugin {},
        space::SpacePlugin {},
        velocity::VelocityPlugin {},
        player::PlayerPlugin {},
        lifetime::LifetimePlugin {},
        collision::CollisionPlugin {},
        health::HealthPlugin {},
        ai::AiPlugin {},
        ship::ShipPlugin {},
        data_config::TablePlugin {},
        ship_composition::ShipCompositionPlugin {},
        graphic::GraphicPlugin {},
        ui::UiPlugin {},
        particle_system::ParticleSystemPlugin {},
    ))
    // debug
    .add_plugins((debug::DebugPlugin {},))
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
    Early,
    Camera,
}

impl SystemUpdateSet {
    fn configuration() -> (
        ScheduleConfigs<Interned<dyn SystemSet>>,
        ScheduleConfigs<Interned<dyn SystemSet>>,
        ScheduleConfigs<Interned<dyn SystemSet>>,
    ) {
        (
            SystemUpdateSet::Main
                .before(SystemUpdateSet::Early)
                .run_if(in_state(AppState::GameReady)),
            SystemUpdateSet::Early
                .before(SystemUpdateSet::Camera)
                .run_if(in_state(AppState::GameReady)),
            SystemUpdateSet::Camera.run_if(in_state(AppState::GameReady)),
        )
    }
}
