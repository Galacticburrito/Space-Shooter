use crate::{
    AppState, SystemUpdateSet,
    data_tbl::{
        blueprint::{BlueprintRegistry, BlueprintTable, BlueprintType},
        data::{DataRegistry, DataTable},
    },
    player::Player,
    ship::{self, Ship, ShipType},
    velocity::{AngularVelocity, Velocity},
};
use bevy::prelude::*;

pub struct AiPlugin {}

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameReady), setup)
            .add_systems(Update, move_ai.in_set(SystemUpdateSet::Main));
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
        "ship_1",
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

    ship::spawn_ship_from_blueprint(
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
    );
}

#[derive(Component)]
pub struct Ai {}

/// move ai ships to enemy (just Player for now...)
fn move_ai(
    ai: Query<(&Ship, &Transform), With<Ai>>,
    enemy: Query<&Transform, With<Player>>,
) -> Result<(), BevyError> {
    let enemy = enemy.single()?;
    for (ship, body) in &ai {
        // move ship to player, another system to shoot?
    }
    Ok(())
}
