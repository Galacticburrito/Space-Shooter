use crate::{
    AppState, Damage, SystemUpdateSet, collision::CollisionEvent, color_palette::PalColor,
    graphic::Graphic, health::Health, lifetime::Lifetime, velocity::Velocity,
};
use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

pub struct BulletPlugin {}

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameReady), setup)
            .add_systems(Update, bullet_collide.in_set(SystemUpdateSet::Main));
    }
}

fn setup(mut commands: Commands) {
    let mut bullet_assets = BulletAssets(HashMap::new());

    // laser bullet
    bullet_assets.0.insert(
        BulletType::Laser,
        Graphic::new(Circle::new(2.0).into(), PalColor::Red),
    );

    // missile bullet
    // missiles are bigger
    bullet_assets.0.insert(
        BulletType::Missile,
        Graphic::new(Circle::new(3.0).into(), PalColor::White),
    );
    commands.insert_resource(bullet_assets);
}

#[derive(Resource)]
pub struct BulletAssets(pub HashMap<BulletType, Graphic>);

/// data that is carried by gun, then copied over to fired bullet
#[derive(Clone, Debug, Deserialize, Reflect)]
pub struct BulletData {
    pub bullet_type: BulletType,
    pub speed: f32,
    pub damage: f32,
}

impl BulletData {
    pub fn new(bullet_type: BulletType, speed: f32, damage: f32) -> Self {
        BulletData {
            bullet_type,
            speed,
            damage,
        }
    }
}
#[derive(Component, Clone, Reflect)]
#[require(Velocity, Lifetime)]
pub struct Bullet {
    pub bullet_data: BulletData,
    /// what entity shot the bullet
    /// must be at top of hierarchy, since determines ignored collisions
    /// TODO: see if want Gun itself instead, then can do searching on that, so more type safety
    pub shooter: Entity,
}

impl Bullet {
    pub fn new(bullet_data: BulletData, shooter: &Entity) -> Self {
        Bullet {
            bullet_data,
            shooter: *shooter,
        }
    }
}

#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Clone, Reflect)]
pub enum BulletType {
    Laser,
    Missile,
}

/// if bullet hit anything not own ship, gets destroyed and adds damage if applicable
fn bullet_collide(
    children_query: Query<&Children>,
    mut collision_events: EventReader<CollisionEvent>,
    mut commands: Commands,
    world: &World,
) {
    for collision in collision_events.read() {
        match collision.get_component::<Bullet>(world) {
            (Some(_), Some(_)) => {
                commands.entity(collision.0).despawn();
                commands.entity(collision.1).despawn();
                info!("bullet collided w/ bullet!");
            }
            (Some(bullet_1), None) => {
                apply_bullet_hit(
                    collision.0,
                    bullet_1,
                    collision.1,
                    children_query,
                    &mut commands,
                    world,
                );
            }
            (None, Some(bullet_2)) => {
                apply_bullet_hit(
                    collision.1,
                    bullet_2,
                    collision.0,
                    children_query,
                    &mut commands,
                    world,
                );
            }
            (None, None) => {}
        }
    }
}

/// delete bullet, apply damage to other object
fn apply_bullet_hit(
    bullet_entity: Entity,
    bullet: &Bullet,
    other: Entity,
    children_query: Query<&Children>,
    commands: &mut Commands,
    world: &World,
) {
    if bullet.shooter == other {
        return;
    }

    // check if other is descendant of shooter
    if let Ok(shooter_children) = children_query.get(bullet.shooter) {
        for child in shooter_children {
            if *child == other {
                return;
            }
        }
    }

    commands.entity(bullet_entity).despawn();
    info!("bullet collided w/ entity {}!", other);

    if world.get::<Health>(other).is_some() {
        commands
            .entity(other)
            .insert(Damage(bullet.bullet_data.damage));
    }
}
