use crate::{PalColor, SystemUpdateSet, body::Body, collision::CollisionEvent, lifetime::Lifetime};
use bevy::prelude::*;
use std::collections::HashMap;

pub struct BulletPlugin {}

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, bullet_collide.in_set(SystemUpdateSet::Main));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut bullet_assets = BulletAssets {
        meshes: HashMap::new(),
        materials: HashMap::new(),
    };

    // laser bullet
    bullet_assets
        .meshes
        .insert(BulletType::Laser, meshes.add(Circle::new(2.0)));
    bullet_assets.materials.insert(
        BulletType::Laser,
        materials.add(Into::<ColorMaterial>::into(PalColor::Red)),
    );

    // missile bullet
    bullet_assets.meshes.insert(
        BulletType::Missile,
        meshes.add(Circle::new(3.0)), // missiles are bigger
    );
    bullet_assets.materials.insert(
        BulletType::Missile,
        materials.add(Into::<ColorMaterial>::into(PalColor::White)),
    );
    commands.insert_resource(bullet_assets);
}

#[derive(Resource)]
pub struct BulletAssets {
    pub meshes: HashMap<BulletType, Handle<Mesh>>,
    pub materials: HashMap<BulletType, Handle<ColorMaterial>>,
}

#[derive(Component, Clone, Reflect)]
#[require(Body, Lifetime)]
pub struct Bullet {
    pub shooter: Entity, // who shot the bullet
    pub bullet_type: BulletType,
    pub speed: f32,
    pub damage: f32,
}

impl Bullet {
    pub fn new(bullet_type: BulletType, shooter: Entity) -> Self {
        match bullet_type {
            BulletType::Laser => Self {
                shooter,
                bullet_type,
                speed: 100.,
                damage: 5.,
            },
            BulletType::Missile => Self {
                shooter,
                bullet_type,
                speed: 50.,
                damage: 10.,
            },
        }
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Reflect)]
pub enum BulletType {
    Laser,
    Missile,
}

/// if bullet hit anything not own ship, gets destroyed and adds damage if applicable
fn bullet_collide(
    mut collision_events: EventReader<CollisionEvent>,
    mut commands: Commands,
    world: &World,
) {
    for collision in collision_events.read() {
        match collision.get_component::<Bullet>(world) {
            (Some(bullet_1), Some(bullet_2)) => {
                commands.entity(collision.0).despawn();
                commands.entity(collision.1).despawn();
                info!("bullet collided w/ bullet!");
            }
            (Some(bullet_1), None) => {
                apply_bullet_hit(collision.0, bullet_1, collision.1, &mut commands);
            }
            (None, Some(bullet_2)) => {
                apply_bullet_hit(collision.1, bullet_2, collision.0, &mut commands);
            }
            (None, None) => {}
        }
    }
}

fn apply_bullet_hit(
    bullet_entity: Entity,
    bullet: &Bullet,
    other: Entity,
    commands: &mut Commands,
) {
    if bullet.shooter == other {
        return;
    }
    commands.entity(bullet_entity).despawn();
    info!("bullet collided w/ something!");
}
