use crate::{
    AppState, Health,
    body::{Body, RotationBody},
    collision::Collider,
    color_palette,
    data_tbl::{
        blueprint::{self, BlueprintKey, BlueprintRegistry, BlueprintTable},
        data::{self, DataKey, DataRegistry, DataTable},
    },
    ship_composition::{
        bullet::{BulletData, BulletType},
        engine::{Engine, EngineType},
        gun::{Gun, GunData, GunType},
    },
};
use bevy::prelude::*;

pub struct ShipPlugin {}

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameReady), setup);
    }
}

fn setup() {}

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

pub fn spawn_ship_from_blueprint(
    value: &str,
    body: Body,
    blueprint_registry: &Res<BlueprintRegistry>,
    blueprint_table: &Res<Assets<BlueprintTable>>,
    data_registry: &Res<DataRegistry>,
    data_table: &Res<Assets<DataTable>>,
    commands: &mut Commands,
) -> Option<Entity> {
    if let Some(ship) = blueprint::entity_from_blueprint(
        &BlueprintKey::Ship,
        value,
        blueprint_registry,
        blueprint_table,
        data_registry,
        data_table,
        commands,
    ) {
        commands.entity(ship).insert((
            Ship {
                ship_type: ShipType::Interceptor,
            },
            body,
            RotationBody::new(0., 0.),
        ));
        return Some(ship);
    }
    None
}

pub fn spawn_ship(
    ship_type: &ShipType,
    body: Body,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    commands: &mut Commands,
) -> Entity {
    let name;
    let shape;
    let collider;

    match ship_type {
        ShipType::Interceptor => {
            name = Name::new("Interceptor");
            shape = Rectangle::new(20., 10.);
            collider = Collider::new_rect(20., 10.);
        }
        ShipType::Gunship => {
            name = Name::new("Gunship");
            shape = Rectangle::new(20., 10.);
            collider = Collider::new_rect(20., 10.);
        }
        ShipType::MissileBoat => {
            name = Name::new("Missile Boat");
            shape = Rectangle::new(20., 10.);
            collider = Collider::new_rect(20., 10.);
        }
    }

    let ship = commands
        .spawn((
            name,
            Ship::new(ship_type),
            body,
            RotationBody::new(0., 0.),
            MeshMaterial2d(materials.add(color_palette::random_color())),
            Mesh2d(meshes.add(shape)),
            collider,
            Health::new(50.),
            children![
                (
                    Name::new("Main Engine"),
                    Engine::new_old(ship_type, &EngineType::Main),
                    Health::new(50.),
                    Collider::new_rect(2., 2.),
                    Transform::from_translation(Vec3::ZERO), // consider adding seperate left/right thruster in future
                ),
                (
                    Name::new("Thruster Engine"),
                    Engine::new_old(ship_type, &EngineType::Thruster),
                    Health::new(50.),
                    Collider::new_rect(2., 2.),
                    Transform::from_translation(Vec3::ZERO)
                )
            ],
        ))
        .id();

    commands.entity(ship).with_children(|parent| {
        parent.spawn((
            Name::new("Laser"),
            Gun::new(
                GunData::new(GunType::Laser, 1.),
                BulletData::new(BulletType::Laser, 10., 10.),
            ),
            Body::new(10., Vec2::ZERO, Vec2::ZERO),
            RotationBody::new(0., 0.),
            Health::new(50.),
            Collider::new_rect(2., 2.),
            Transform::from_translation(Vec3::ZERO),
        ));
    });
    ship
}

#[derive(Component, Reflect)]
pub struct Ship {
    ship_type: ShipType,
}

impl Ship {
    pub fn new(ship_type: &ShipType) -> Self {
        let ship_type = ship_type.clone();
        match ship_type {
            ShipType::Interceptor => Ship { ship_type },
            ShipType::Gunship => Ship { ship_type },
            ShipType::MissileBoat => Ship { ship_type },
        }
    }
}

#[derive(Clone, Reflect)]
pub enum ShipType {
    /// hit and run, fast and agile
    Interceptor,
    /// think flying tank. Less manuverable but stocked full of weapons
    Gunship,
    /// long range missile attacks, long reloads (spawn ships?)
    MissileBoat,
}
