use super::bullet::{Bullet, BulletAssets, BulletData};
use crate::{
    SystemUpdateSet,
    body::{Body, RotationBody},
    collision::Collider,
    lifetime::Lifetime,
};
use bevy::prelude::*;
use serde::Deserialize;

// NOTE: consider spawning each guntype only once, then using that as base to shoot bullets

pub struct GunPlugin {}

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, gun_cooldown.in_set(SystemUpdateSet::Main))
            .register_type::<Gun>();
    }
}

/// static info about the gun
#[derive(Reflect, Deserialize, Clone, Debug)]
pub struct GunData {
    gun_type: GunType,
    /// rounds per second
    fire_rate: f32,
}

impl GunData {
    pub fn new(gun_type: GunType, fire_rate: f32) -> Self {
        GunData {
            gun_type,
            fire_rate,
        }
    }
}

#[derive(Component, Clone, Reflect)]
pub struct Gun {
    gun_data: GunData,
    bullet_data: BulletData,
    cooldown: Timer,
}

impl Gun {
    pub fn new(gun_data: GunData, bullet_data: BulletData) -> Self {
        Gun {
            gun_data: gun_data.clone(),
            bullet_data,
            cooldown: Timer::from_seconds(gun_data.fire_rate, TimerMode::Once),
        }
    }

    /// if can shoot, shoot the gun
    pub fn try_shoot(
        &self,
        shooter: &Entity,
        commands: &mut Commands,
        body: &Body,
        g_transform: &GlobalTransform,
        rotation: &RotationBody,
        bullet_assets: &Res<BulletAssets>,
    ) {
        if self.can_shoot() {
            self.shoot_bullet(
                shooter,
                commands,
                body,
                g_transform,
                rotation,
                bullet_assets,
            );
        }
    }

    fn can_shoot(&self) -> bool {
        self.cooldown.finished()
    }

    /// spawn bullet depending on gun type (i.e., pulselaser will spawn multiple at diff angles)
    fn shoot_bullet(
        &self,
        shooter: &Entity,
        commands: &mut Commands,
        body: &Body,
        g_transform: &GlobalTransform,
        rotation: &RotationBody,
        bullet_assets: &Res<BulletAssets>,
    ) {
        match self.gun_data.gun_type {
            GunType::Laser => {
                self.spawn_bullet(
                    shooter,
                    body,
                    g_transform,
                    rotation,
                    commands,
                    bullet_assets,
                );
            }
            GunType::PulseLaser => {
                self.spawn_bullet(
                    shooter,
                    body,
                    g_transform,
                    rotation,
                    commands,
                    bullet_assets,
                );
            }
            GunType::HomingMissile => {
                self.spawn_bullet(
                    shooter,
                    body,
                    g_transform,
                    rotation,
                    commands,
                    bullet_assets,
                );
            }
        }
    }

    fn spawn_bullet(
        &self,
        shooter: &Entity,
        body: &Body,
        g_transform: &GlobalTransform,
        rotation: &RotationBody,
        commands: &mut Commands,
        bullet_assets: &Res<BulletAssets>,
    ) {
        let mesh_handle = bullet_assets
            .meshes
            .get(&self.bullet_data.bullet_type)
            .unwrap();
        let material_handle = bullet_assets
            .materials
            .get(&self.bullet_data.bullet_type)
            .unwrap();

        info!(
            "bullet speed: {}, body velocity: {}, rotation_vector: {}",
            self.bullet_data.speed,
            body.velocity,
            rotation.rotation_vector()
        );
        commands.spawn((
            Bullet::new(self.bullet_data.clone(), shooter),
            Body {
                mass: 1.,
                position: body.global_position(g_transform),
                velocity: rotation.rotation_vector() * (self.bullet_data.speed + body.velocity),
            },
            Collider::new_rect(2., 2.),
            Lifetime::new(5.),
            MeshMaterial2d(material_handle.clone()),
            Mesh2d(mesh_handle.clone()),
        ));
    }
}

#[derive(Debug, Clone, Deserialize, Reflect)]
pub enum GunType {
    /// rapid fire, moderate damage, good accuracy
    Laser,
    /// short bursts, less accuracy (think shotgun)
    PulseLaser,
    /// lock onto enemy ships (implement countermeasures?)
    HomingMissile,
}

fn gun_cooldown(query: Query<&mut Gun>, time: Res<Time>) {
    for mut gun in query {
        gun.cooldown.tick(time.delta());
    }
}
