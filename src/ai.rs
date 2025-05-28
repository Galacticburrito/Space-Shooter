use crate::{
    AppState, SystemUpdateSet,
    body::Body,
    data_tbl::{
        blueprint::{BlueprintRegistry, BlueprintTable},
        data::{DataRegistry, DataTable},
    },
    player::Player,
    ship::{self, Ship, ShipType},
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    blueprint_registry: Res<BlueprintRegistry>,
    blueprint_table: Res<Assets<BlueprintTable>>,
    data_registry: Res<DataRegistry>,
    data_table: Res<Assets<DataTable>>,
) {
    ship::spawn_ship(
        &ShipType::Interceptor,
        Body::new(1000., Vec2::new(-200., 0.), Vec2::new(10., 0.)),
        &mut materials,
        &mut meshes,
        &mut commands,
    );

    ship::spawn_ship(
        &ShipType::Interceptor,
        Body::new(1000., Vec2::new(-200., 0.), Vec2::new(10., 0.)),
        &mut materials,
        &mut meshes,
        &mut commands,
    );

    if let Some(ship) = ship::spawn_ship_from_blueprint(
        "ship_1",
        Body::new(1., Vec2::new(0., 10.), Vec2::ZERO),
        &blueprint_registry,
        &blueprint_table,
        &data_registry,
        &data_table,
        &mut commands,
    ) {
        commands.entity(ship).log_components();
        commands.entity(ship).insert(Name::new("SHIIP"));
        info!("added shiip! :)");
    }
}

#[derive(Component)]
pub struct Ai {}

/// move ai ships to enemy (just Player for now...)
fn move_ai(
    ai: Query<(&Ship, &Body), With<Ai>>,
    enemy: Query<(&Body), With<Player>>,
) -> Result<(), BevyError> {
    let enemy = enemy.single()?;
    for (ship, body) in &ai {
        // move ship to player, another system to shoot?
    }
    Ok(())
}
