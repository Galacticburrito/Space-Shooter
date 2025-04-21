use crate::{
    PalColor, SystemUpdateSet,
    body::{Body, RotationBody},
    player::Player,
};
use bevy::prelude::*;
use std::collections::HashMap;

// NOTE: consider spawning each guntype only once, then using that as base to shoot bullets

pub struct GunPlugin {}

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (shoot_manual_gun).in_set(SystemUpdateSet::Main));

        app.register_type::<Gun>();
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
        meshes.add(Circle::new(3.0)), // missles are bigger
    );
    bullet_assets.materials.insert(
        BulletType::Missile,
        materials.add(Into::<ColorMaterial>::into(PalColor::White)),
    );

    commands.insert_resource(bullet_assets);
}

#[derive(Resource)]
pub struct BulletAssets {
    meshes: HashMap<BulletType, Handle<Mesh>>,
    materials: HashMap<BulletType, Handle<ColorMaterial>>,
}

#[derive(Component, Reflect)]
pub struct Gun {
    gun_type: GunType,
    bullet: Bullet,
    fire_rate: f32,
}

impl Gun {
    pub fn new(gun_type: GunType) -> Self {
        match gun_type {
            GunType::Laser => Self {
                gun_type,
                bullet: Bullet::new(BulletType::Laser),
                fire_rate: 10.,
            },
            GunType::PulseLaser => Self {
                gun_type,
                bullet: Bullet::new(BulletType::Laser),
                fire_rate: 5.,
            },
            GunType::HomingMissile => Self {
                gun_type,
                bullet: Bullet::new(BulletType::Missile),
                fire_rate: 1.,
            },
        }
    }

    // spawn bullet depending on gun type (i.e., pulselaser will spawn multiple at diff angles)
    pub fn shoot_bullet(
        &self,
        commands: &mut Commands,
        position: Vec2,
        rotation: f32,
        bullet_assets: &Res<BulletAssets>,
    ) {
        info!("shooting!");
        match self.gun_type {
            GunType::Laser => {
                self.spawn_bullet(position, rotation, commands, bullet_assets);
            }
            GunType::PulseLaser => {
                self.spawn_bullet(position, rotation, commands, bullet_assets);
            }
            GunType::HomingMissile => {
                self.spawn_bullet(position, rotation, commands, bullet_assets);
            }
        }
    }

    fn spawn_bullet(
        &self,
        position: Vec2,
        rotation: f32,
        commands: &mut Commands,
        bullet_assets: &Res<BulletAssets>,
    ) {
        info!("spawning bullet!");
        let mesh_handle = bullet_assets.meshes.get(&self.bullet.bullet_type).unwrap();
        let material_handle = bullet_assets
            .materials
            .get(&self.bullet.bullet_type)
            .unwrap();

        commands.spawn((
            self.bullet.clone(),
            Body {
                mass: 1.,
                position,
                velocity: Vec2::new(f32::cos(rotation), f32::sin(rotation)) * self.bullet.speed, // make
            },
            MeshMaterial2d(material_handle.clone().into()),
        ));
    }
}

#[derive(Reflect)]
pub enum GunType {
    Laser,         // rapid fire, moderate damage, good accuracy
    PulseLaser,    // short bursts, less accuracy (think shotgun)
    HomingMissile, // lock onto enemy ships (implement countermeasures?)
}

#[derive(Component, Clone, Reflect)]
struct Bullet {
    bullet_type: BulletType,
    speed: f32,
    damage: f32,
}

impl Bullet {
    pub fn new(bullet_type: BulletType) -> Self {
        match bullet_type {
            BulletType::Laser => Self {
                bullet_type,
                speed: 10.,
                damage: 5.,
            },
            BulletType::Missile => Self {
                bullet_type,
                speed: 5.,
                damage: 10.,
            },
        }
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Reflect)]
enum BulletType {
    Laser,
    Missile,
}

// systems

// TODO: system to detect if hit ship

// TODO: consider going back to transform, global transform, or just get that from parent?
/// if space bar pressed, have player main gun shoot
fn shoot_manual_gun(
    parent: Query<(&Body, &RotationBody, &Children), With<Player>>,
    children: Query<(&Gun, &GlobalTransform)>,
    bullet_assets: Res<BulletAssets>,
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    for (p_body, p_rot_body, p_children) in &parent {
        for (gun, g_transform) in &children {
            if keys.just_pressed(KeyCode::Space) {
                gun.shoot_bullet(
                    &mut commands,
                    g_transform.translation().xy(),
                    p_rot_body.rotation,
                    &bullet_assets,
                );
            }
        }
    }
}
