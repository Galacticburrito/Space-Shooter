use crate::{
    AppState,
    data_config::{
        blueprint::{BlueprintRegistry, BlueprintTable, BlueprintType},
        data::{DataRegistry, DataTable},
    },
    ship,
    velocity::{AngularVelocity, Velocity},
};
use bevy::prelude::*;
use bevy_behave::prelude::BehaveTree;

pub struct SetupPlugin {}

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameReady), setup);
    }
}

fn setup(
    mut commands: Commands,
    blueprint_registry: Res<BlueprintRegistry>,
    blueprint_table: Res<Assets<BlueprintTable>>,
    data_registry: Res<DataRegistry>,
    data_table: Res<Assets<DataTable>>,
) {
    ship::spawn_ship_from_blueprint(
        "ship_2",
        &BlueprintType::TransformVelocity(
            Transform::from_translation(Vec3::new(-200., 0., 0.)),
            Velocity(Vec2::new(10., 0.)),
            AngularVelocity(0.),
        ),
        &blueprint_registry,
        &blueprint_table,
        &data_registry,
        &data_table,
        &mut commands,
    );

    ship::spawn_ship_from_blueprint(
        "ship_2",
        &BlueprintType::TransformVelocity(
            Transform::from_translation(Vec3::new(200., 0., 0.)),
            Velocity(Vec2::new(10., 0.)),
            AngularVelocity(0.),
        ),
        &blueprint_registry,
        &blueprint_table,
        &data_registry,
        &data_table,
        &mut commands,
    );

    if let Some(ai_ship) = ship::spawn_ship_from_blueprint(
        "ship_1",
        &BlueprintType::TransformVelocity(
            Transform::from_translation(Vec3::new(0., 10., 0.)),
            Velocity::ZERO,
            AngularVelocity(0.),
        ),
        &blueprint_registry,
        &blueprint_table,
        &data_registry,
        &data_table,
        &mut commands,
    ) {
        super::ai_ship_tree::add_ai_ship_tree(ai_ship, &mut commands, false);
    }
}
