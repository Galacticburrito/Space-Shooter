use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use serde::Deserialize;

use crate::iterable_enum::IterableEnum;

pub struct DataPlugin {}

impl Plugin for DataPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<DataTable>::new(&["data.ron"])) // add RON asset plugin, specifying file extension
            .init_resource::<DataRegistry>()
            .add_systems(Startup, load_data_registries)
            .add_systems(Update, access_data_entry);
    }
}

#[derive(Deserialize, TypePath, Asset, Debug)]
struct DataEntry {
    #[serde(alias = "id")]
    name: String,
    #[serde(flatten)]
    data: HashMap<String, ron::Value>,
}

#[derive(Resource, Debug, Default)]
struct DataRegistry {
    registries: HashMap<String, Handle<DataTable>>,
}

#[derive(Deserialize, TypePath, Asset, Debug)]
struct DataTable(Vec<DataEntry>);

enum DataComponentTypes {
    Engine,
}

impl DataComponentTypes {
    pub fn string(&self) -> String {
        match self {
            Self::Engine => "engine",
        }
        .to_owned()
    }
}

impl IterableEnum for DataComponentTypes {
    type Iter = std::array::IntoIter<DataComponentTypes, 1>;
    fn iter() -> Self::Iter {
        [DataComponentTypes::Engine].into_iter()
    }
}
fn load_data_registries(asset_server: Res<AssetServer>, mut data_registry: ResMut<DataRegistry>) {
    let registry_paths: HashMap<String, String> = DataComponentTypes::iter()
        .map(|d| (d.string(), format!("data/{}.data.ron", d.string())))
        .collect();

    //("engine", "data/engine.data.ron")

    for (registry_name, path) in registry_paths {
        let handle: Handle<DataTable> = asset_server.load(path);
        data_registry
            .registries
            .insert(registry_name.to_string(), handle);
    }
}

// System to access a specific entry from a loaded data registry
fn access_data_entry(data_registry: Res<DataRegistry>, assets: Res<Assets<DataTable>>) {
    // Example: Accessing data for an engine named "engine_mk1"
    if let Some(engine_table_handle) = data_registry.registries.get("engine") {
        if let Some(engine_table) = assets.get(engine_table_handle) {
            if let Some(entry) = engine_table.0.iter().find(|e| e.name == "engine_mk1") {
                if let Some(value) = entry.data.get("power") {
                    info!("Engine Power: {:?}", value);
                }
                if let Some(value) = entry.data.get("thrust") {
                    info!("Engine Thrust: {:?}", value);
                }
            } else {
                info!("Engine data for 'engine_mk1' not found.");
            }
        } else {
            info!("Engine data registry not yet loaded.");
        }
    }
}
