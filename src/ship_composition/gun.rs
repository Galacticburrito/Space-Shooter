use super::bullet::{Bullet, BulletAssets, BulletData};
use crate::{
    SystemUpdateSet,
    collision::{
        collider::{Collider, CollisionLayer},
        collider_type::ColliderType,
    },
    global::GlobalVelocity,
    lifetime::Lifetime,
    mass::Mass,
    rotation,
    velocity::Velocity,
};
use bevy::prelude::*;
use serde::Deserialize;

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
        g_transform: &GlobalTransform,
        g_velocity: &GlobalVelocity,
        bullet_assets: &Res<BulletAssets>,
    ) {
        if self.can_shoot() {
            self.shoot_bullet(shooter, commands, g_transform, g_velocity, bullet_assets);
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
        g_transform: &GlobalTransform,
        g_velocity: &GlobalVelocity,
        bullet_assets: &Res<BulletAssets>,
    ) {
        match self.gun_data.gun_type {
            GunType::Laser => {
                self.spawn_bullet(shooter, g_transform, g_velocity, commands, bullet_assets);
            }
            GunType::PulseLaser => {
                self.spawn_bullet(shooter, g_transform, g_velocity, commands, bullet_assets);
            }
            GunType::HomingMissile => {
                self.spawn_bullet(shooter, g_transform, g_velocity, commands, bullet_assets);
            }
        }
    }

    fn spawn_bullet(
        &self,
        shooter: &Entity,
        g_transform: &GlobalTransform,
        g_velocity: &GlobalVelocity,
        commands: &mut Commands,
        bullet_assets: &Res<BulletAssets>,
    ) {
        let graphic = bullet_assets.0.get(&self.bullet_data.bullet_type).unwrap();

        let g_position = g_transform.translation().xy();

        // velocity without g_velocity
        let rel_velocity =
            rotation::rad_to_vec2(Vec3::from(g_transform.rotation().to_euler(EulerRot::XYZ)).z)
                * self.bullet_data.speed;
        let velocity = rel_velocity + g_velocity.0;

        commands.spawn((
            Bullet::new(self.bullet_data.clone(), shooter),
            Transform::from_translation(Vec3::new(g_position.x, g_position.y, 0.)),
            Mass(1.),
            Velocity(velocity),
            Collider::new(ColliderType::new_rect(2., 2.), CollisionLayer::Bullet),
            Lifetime::new(5.),
            graphic.clone(),
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
