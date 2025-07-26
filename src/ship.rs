use crate::ship_composition::sonar::SonarDetectable;
use crate::{
    AppState,
    data_config::{
        blueprint::{self, BlueprintKey, BlueprintRegistry, BlueprintTable, BlueprintType},
        data::{self, DataKey, DataRegistry, DataTable},
    },
    health::PropagateHealth,
};
use bevy::prelude::*;

pub struct ShipPlugin {}

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameReady), setup);
    }
}

fn setup() {}

/// depricated, if can use spawn_ship_from_blueprint instead
pub fn spawn_ship_from_data(
    parts: &Vec<(DataKey, String)>,
    registry: &Res<DataRegistry>,
    table: &Res<Assets<DataTable>>,
    commands: &mut Commands,
) {
    for part in parts {
        let child_entity = data::insert_from_data(
            &mut commands.spawn_empty(),
            &part.0,
            &part.1,
            registry,
            table,
        );
    }
}

/// get data from assets, then add other needed components
pub fn spawn_ship_from_blueprint(
    value: &str,
    blueprint_type: &BlueprintType,
    blueprint_registry: &Res<BlueprintRegistry>,
    blueprint_table: &Res<Assets<BlueprintTable>>,
    data_registry: &Res<DataRegistry>,
    data_table: &Res<Assets<DataTable>>,
    commands: &mut Commands,
) -> Option<Entity> {
    let ship = blueprint::entity_from_blueprint(
        &BlueprintKey::Ship,
        value,
        blueprint_type,
        blueprint_registry,
        blueprint_table,
        data_registry,
        data_table,
        commands,
    )?;

    commands
        .entity(ship)
        .insert((Ship {}, SonarDetectable::new(), PropagateHealth::new()));

    Some(ship)
}

#[derive(Component, Reflect)]
#[require(SonarDetectable)]
pub struct Ship {}

#[derive(Clone, Reflect)]
pub enum ShipType {
    /// hit and run, fast and agile
    Interceptor,
    /// think flying tank. Less manuverable but stocked full of weapons
    Gunship,
    /// long range missile attacks, long reloads (spawn ships?)
    MissileBoat,
}
