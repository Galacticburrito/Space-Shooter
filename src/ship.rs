use crate::{
    Health,
    body::{Body, RotationBody},
    collision::Collider,
    color_palette,
    ship_composition::{
        engine::{Engine, EngineType},
        gun::{Gun, GunType},
    },
};
use bevy::prelude::*;

pub struct ShipPlugin {}

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup() {}

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
                    Engine::new(ship_type, &EngineType::Main),
                    Health::new(50.),
                    Collider::new_rect(2., 2.),
                    Transform::from_translation(Vec3::ZERO),
                ),
                // consider adding seperate left/right thruster in future
                (
                    Name::new("Thruster Engine"),
                    Engine::new(ship_type, &EngineType::Thruster),
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
            Gun::new(GunType::Laser, ship),
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
