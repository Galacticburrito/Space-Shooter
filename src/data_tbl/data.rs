use super::component_data::{self, ComponentConcrete, ComponentData};
use crate::{AppState, iterable_enum::IterableEnum};
use bevy::{platform::collections::HashMap, prelude::*};
use bevy_common_assets::ron::RonAssetPlugin;
use serde::Deserialize;

use super::assets_loaded::AssetsLoading;

pub struct DataPlugin {}

impl Plugin for DataPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<DataTable>::new(&[".ron"]))
            .init_resource::<DataRegistry>()
            .add_systems(OnEnter(AppState::LoadingAssets), load_data_registry);
    }
}

#[derive(Deserialize, TypePath, Asset, Clone, Debug)]
struct DataEntry {
    #[serde(alias = "id")]
    name: String,
    components: Vec<ComponentData>,
}

#[derive(Resource, Debug, Default)]
pub struct DataRegistry {
    tables: HashMap<String, Handle<DataTable>>,
}

#[derive(Deserialize, TypePath, Asset, Debug)]
pub struct DataTable(Vec<DataEntry>);

#[derive(Clone, Debug, Deserialize)]
pub enum DataKey {
    Engine,
    Gun,
}

impl DataKey {
    pub fn string(&self) -> String {
        match self {
            Self::Engine => "engine",
            Self::Gun => "gun",
        }
        .to_owned()
    }
}

impl IterableEnum for DataKey {
    type Iter = std::array::IntoIter<DataKey, 2>;
    fn iter() -> Self::Iter {
        [DataKey::Engine, DataKey::Gun].into_iter()
    }
}

fn load_data_registry(
    asset_server: Res<AssetServer>,
    mut data_registry: ResMut<DataRegistry>,
    mut assets_loading: ResMut<AssetsLoading>,
) {
    let registry_paths: HashMap<String, String> = DataKey::iter()
        .map(|d| (d.string(), format!("data/{}.ron", d.string())))
        .collect();

    for (registry_name, path) in registry_paths {
        let handle: Handle<DataTable> = asset_server.load(&path);
        assets_loading.0.push(handle.id().untyped());
        data_registry.tables.insert(registry_name, handle);
    }
    info!("data registry loading")
}

/// returns a copied entry from loaded data registry
fn access_data_entry(
    key: &str,
    value: &str,
    data_registry: &Res<DataRegistry>,
    assets: &Res<Assets<DataTable>>,
) -> Option<DataEntry> {
    let Some(handle) = data_registry.tables.get(key) else {
        warn!(
            "data table \"{}\"not found! \nCurrent entries: {:?}",
            key, data_registry.tables
        );
        return None;
    };
    let Some(table) = assets.get(handle) else {
        warn!("data table not exist in assets!");
        return None;
    };
    let Some(entry) = table.0.iter().find(|e| e.name == value) else {
        warn!(
            "data entry \"{}\"not found! \nCurrent entries: {:?}",
            value, table.0
        );
        return None;
    };
    Some(entry.clone())
}

/// inserts needed components to given entity, as well as name
pub fn insert_from_data(
    entity: &mut EntityCommands,
    key: &DataKey,
    value: &str,
    data_registry: &Res<DataRegistry>,
    assets: &Res<Assets<DataTable>>,
) {
    let entry = access_data_entry(&key.string(), value, data_registry, assets);
    if let Some(entry) = entry {
        component_data::add_components_to_entity(entity, &entry.components);
        entity.insert(Name::new(value.to_owned()));
    }
}
