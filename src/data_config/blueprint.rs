use super::{
    assets_loaded::AssetsLoading,
    component_data::{self, ComponentData},
    data::{self, DataKey, DataRegistry, DataTable},
};
use crate::{
    AppState, debug,
    iterable_enum::IterableEnum,
    velocity::{AngularVelocity, Velocity},
};
use bevy::{platform::collections::HashMap, prelude::*};
use bevy_common_assets::ron::RonAssetPlugin;
use serde::Deserialize;

pub struct BlueprintPlugin {}

impl Plugin for BlueprintPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<BlueprintTable>::new(&[".ron"]))
            .init_resource::<BlueprintRegistry>()
            .add_systems(OnEnter(AppState::LoadingAssets), load_blueprint_registry);

        debug::insert_inspectable_resource(
            app,
            Some(BlueprintRegistry {
                tables: HashMap::new(),
            }),
            false,
        );
    }
}

#[derive(Deserialize, TypePath, Asset, Clone, Debug)]
struct BlueprintEntry {
    #[serde(alias = "id")]
    name: String,
    /// components this parent entity will directly have
    components: Vec<ComponentData>,
    /// data that is added to parent entity
    modules: Vec<(DataKey, String)>,
    /// each data element is made into a child of the parent
    children: Vec<(DataKey, String)>,
}

#[derive(Reflect, Resource, Debug, Default)]
pub struct BlueprintRegistry {
    tables: HashMap<String, Handle<BlueprintTable>>,
}

#[derive(Deserialize, TypePath, Asset, Debug)]
pub struct BlueprintTable(Vec<BlueprintEntry>);

pub enum BlueprintKey {
    Ship,
}

impl BlueprintKey {
    pub fn string(&self) -> String {
        match self {
            Self::Ship => "ship",
        }
        .to_owned()
    }
}

impl IterableEnum for BlueprintKey {
    type Iter = std::array::IntoIter<BlueprintKey, 1>;
    fn iter() -> Self::Iter {
        [BlueprintKey::Ship].into_iter()
    }
}

fn load_blueprint_registry(
    asset_server: Res<AssetServer>,
    mut blueprint_registry: ResMut<BlueprintRegistry>,
    mut assets_loading: ResMut<AssetsLoading>,
) {
    let registry_paths: HashMap<String, String> = BlueprintKey::iter()
        .map(|d| (d.string(), format!("blueprint/{}.ron", d.string())))
        .collect();

    for (registry_name, path) in registry_paths {
        let handle: Handle<BlueprintTable> = asset_server.load(path);
        assets_loading.0.push(handle.id().untyped());
        blueprint_registry
            .tables
            .insert(registry_name.to_string(), handle);
    }
    info!("blueprint registry loading");
}

/// returns a copied entry from loaded blueprint registry
fn access_blueprint_entry(
    key: &str,
    value: &str,
    blueprint_registry: &Res<BlueprintRegistry>,
    assets: &Res<Assets<BlueprintTable>>,
) -> Option<BlueprintEntry> {
    let Some(handle) = blueprint_registry.tables.get(key) else {
        warn!(
            "blueprint table \"{}\"not found! \nCurrent entries: {:?}",
            key, blueprint_registry.tables
        );
        return None;
    };

    let Some(table) = assets.get(handle) else {
        warn!("blueprint table not exist in assets!");
        return None;
    };
    let Some(entry) = table.0.iter().find(|e| e.name == value) else {
        warn!(
            "blueprint entry \"{}\"not found! \nCurrent entries: {:?}",
            value, table.0
        );
        return None;
    };
    Some(entry.clone())
}

/// what data do you want added to the blueprint?
pub enum BlueprintType {
    /// no extra data added
    Normal,
    /// add Transform
    Transform(Transform),
    /// add Transform, Velocity, and AngularVelocity
    TransformVelocity(Transform, Velocity, AngularVelocity),
}

impl BlueprintType {
    fn add_components(&self, entity: Entity, commands: &mut Commands) {
        match self {
            BlueprintType::Normal => {}
            BlueprintType::Transform(transform) => {
                commands.entity(entity).insert(*transform);
            }
            BlueprintType::TransformVelocity(transform, velocity, anguler_velocity) => {
                commands.entity(entity).insert((
                    *transform,
                    velocity.clone(),
                    anguler_velocity.clone(),
                ));
            }
        }
    }
}

/// creates entity from a given blueprint, if exists.
/// Also adds name component and other components depending on blueprint_type
pub fn entity_from_blueprint(
    key: &BlueprintKey,
    value: &str,
    blueprint_type: &BlueprintType,
    blueprint_registry: &Res<BlueprintRegistry>,
    blueprint_assets: &Res<Assets<BlueprintTable>>,
    data_registry: &Res<DataRegistry>,
    data_assets: &Res<Assets<DataTable>>,
    commands: &mut Commands,
) -> Option<Entity> {
    let mut entry =
        access_blueprint_entry(&key.string(), value, blueprint_registry, blueprint_assets)?;

    let entity = main_entity(&mut entry, data_registry, data_assets, commands);
    let child_entities = child_entities(&mut entry, data_registry, data_assets, commands);

    for child_entity in child_entities {
        // add needed components to children
        commands.entity(entity).add_child(child_entity);
        blueprint_type.add_components(child_entity, commands);
    }

    // add needed components to parent
    commands.entity(entity).insert(Name::new(value.to_owned()));
    blueprint_type.add_components(entity, commands);

    Some(entity)
}

fn main_entity(
    entry: &mut BlueprintEntry,
    data_registry: &Res<DataRegistry>,
    data_assets: &Res<Assets<DataTable>>,
    commands: &mut Commands,
) -> Entity {
    let mut entity = commands.spawn_empty();

    component_data::add_components_to_entity(&mut entity, &entry.components);

    for module in entry.modules.iter_mut() {
        data::insert_from_data(
            &mut entity,
            &module.0,
            &module.1,
            data_registry,
            data_assets,
        );
    }
    entity.id()
}

fn child_entities(
    entry: &mut BlueprintEntry,
    data_registry: &Res<DataRegistry>,
    data_assets: &Res<Assets<DataTable>>,
    commands: &mut Commands,
) -> Vec<Entity> {
    let mut child_entities = Vec::new();

    for child_module in entry.children.iter_mut() {
        let mut child_entity = commands.spawn_empty();
        {
            data::insert_from_data(
                &mut child_entity,
                &child_module.0,
                &child_module.1,
                data_registry,
                data_assets,
            );
        }
        child_entities.push(child_entity.id());
    }
    child_entities
}
