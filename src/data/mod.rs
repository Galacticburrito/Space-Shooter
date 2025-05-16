use crate::{
    iterable_enum::IterableEnum,
    ship_composition::engine::{Engine, EngineType},
};
use bevy::{
    platform::collections::HashMap,
    prelude::*,
    reflect::{TypeRegistry, serde::ReflectDeserializer},
};
use bevy_common_assets::ron::RonAssetPlugin;
use ron::{Value, de::Deserializer};
use serde::{Deserialize, de::DeserializeOwned};
use std::any::TypeId;

pub struct DataPlugin {}

impl Plugin for DataPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<DataTable>::new(&["data.ron"])) // add RON asset plugin, specifying file extension
            .init_resource::<DataRegistry>()
            .add_systems(Startup, load_data_registries)
            .add_systems(Update, test_data_access);
    }
}

#[derive(Deserialize, Debug, Clone)]
struct ComponentData {
    #[serde(rename = "type")]
    name: String,
    #[serde(flatten)]
    fields: HashMap<String, ron::Value>,
}

impl ComponentData {
    /// access multiple fields at once
    fn access_fields(&self, fields: &[&str]) -> Vec<Option<&Value>> {
        fields.iter().map(|f| self.fields.get(*f)).collect()
    }

    /// get concrete type of field
    fn get_converted_field<T: DeserializeOwned>(
        &self,
        field_name: &str,
    ) -> Result<T, FieldConversionError> {
        if let Some(field) = self.fields.get(field_name) {
            if let Ok(val) = field.clone().into_rust::<T>() {
                return Ok(val);
            }
            // since into_rust not support enums, see if enum here
            if let Some(val) = self.enum_conversion::<T>(field) {
                return Ok(val);
            }
            return Err(FieldConversionError::ConversionFailed(
                field_name.to_owned(),
            ));
        }
        Err(FieldConversionError::FieldNotFound(field_name.to_owned()))
    }

    /// if field not included, add default value instead
    fn get_converted_field_default<T: DeserializeOwned + Default>(&self, field_name: &str) -> T {
        self.get_converted_field(field_name).unwrap_or(T::default())
    }

    /// custom deserializing of enums
    fn enum_conversion<T: 'static + DeserializeOwned>(&self, value: &Value) -> Option<T> {
        if let Ok(val) = value.into_rust::<String>() {
            match TypeId::of::<T>() {
                id if id == TypeId::of::<EngineType>() => match val.as_ref() {
                    "Main" => Some(EngineType::Main) as T,
                    "Thruster" => Some(EngineType::Thruster) as T,
                },
                _ => {
                    warn!("not able to convert to enum!");
                    None
                }
            }
        }
        None
    }
}

enum FieldConversionError {
    FieldNotFound(String),
    ConversionFailed(String),
}

#[derive(Deserialize, TypePath, Asset, Clone, Debug)]
struct DataEntry {
    #[serde(alias = "id")]
    name: String,
    components: Vec<ComponentData>,
}

impl DataEntry {
    /// returns a copied component from data entry
    fn access_component(&self, component_name: &str) -> Option<&ComponentData> {
        self.components.iter().find(|c| c.name == component_name)
    }
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

    for (registry_name, path) in registry_paths {
        let handle: Handle<DataTable> = asset_server.load(path);
        data_registry
            .registries
            .insert(registry_name.to_string(), handle);
    }
}

/// returns a copied entry from loaded data registry
fn access_data_entry(
    key: &str,
    value: &str,
    data_registry: Res<DataRegistry>,
    assets: Res<Assets<DataTable>>,
) -> Option<DataEntry> {
    if let Some(handle) = data_registry.registries.get(key) {
        if let Some(table) = assets.get(handle) {
            if let Some(entry) = table.0.iter().find(|e| e.name == value) {
                return Some(entry.clone());
            }
            info!("Engine data registry not yet loaded.");
        }
    }
    None
}

fn component_from_data(componentdata: &ComponentData) -> Result<(), FieldConversionError> {
    match componentdata.name.as_str() {
        "engine" => {
            let engine_type = componentdata.get_converted_field("engine_type")?;
            let max_thrust = componentdata.get_converted_field("max_thrust")?;
            let max_acceleration = componentdata.get_converted_field("max_acceleration")?;
            // default: 10%
            let reverse_percent = componentdata.get_converted_field_default("reverse_percent");
            Engine::new(&engine_type, max_thrust, max_acceleration, reverse_percent);
        }
        _ => warn!(
            "Unknown component: {}. Unable to parse!",
            componentdata.name
        ),
    }
    Ok(())
}

fn test_data_access(
    data_registry: Res<DataRegistry>,
    type_registry: Res<AppTypeRegistry>,
    assets: Res<Assets<DataTable>>,
) {
    let entry = access_data_entry("engine", "engine_mk1", data_registry, assets);
    if let Some(entry) = entry {
        if let Some(engine) = entry.access_component("engine") {
            match component_from_data(engine) {
                Ok(()) => info!("success :)"),
                Err(FieldConversionError::FieldNotFound(field)) => {
                    warn!("field not found: {}", field)
                }
                Err(FieldConversionError::ConversionFailed(field)) => {
                    warn!("failed to convert field: {}", field)
                }
            }
            /*
            if let Some(value) = engine.fields.get("power") {
                info!("Engine Power: {:?}", value);
            }
            if let Some(value) = engine.fields.get("thrust") {
                info!("Engine Thrust: {:?}", value);
            }*/
        }
    }
}
