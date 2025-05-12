use crate::{
    SystemUpdateSet,
    body::{Body, RotationBody},
    bullet::{Bullet, BulletAssets, BulletType},
    collision::Collider,
    lifetime::Lifetime,
    player::Player,
};
use bevy::prelude::*;

// NOTE: consider spawning each guntype only once, then using that as base to shoot bullets

pub struct GunPlugin {}

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (shoot_manual_gun).in_set(SystemUpdateSet::Main));

        app.register_type::<Gun>();
    }
}

#[derive(Component, Reflect)]
pub struct Gun {
    gun_type: GunType,
    bullet: Bullet,
    fire_rate: f32,
}

impl Gun {
    pub fn new(gun_type: GunType, owner: Entity) -> Self {
        match gun_type {
            GunType::Laser => Self {
                gun_type,
                bullet: Bullet::new(BulletType::Laser, owner),
                fire_rate: 10.,
            },
            GunType::PulseLaser => Self {
                gun_type,
                bullet: Bullet::new(BulletType::Laser, owner),
                fire_rate: 5.,
            },
            GunType::HomingMissile => Self {
                gun_type,
                bullet: Bullet::new(BulletType::Missile, owner),
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
            Collider::new_rect(2., 2.),
            Lifetime::new(5.),
            MeshMaterial2d(material_handle.clone().into()),
            Mesh2d(mesh_handle.clone().into()),
        ));
    }
}

#[derive(Reflect)]
pub enum GunType {
    /// rapid fire, moderate damage, good accuracy
    Laser,
    /// short bursts, less accuracy (think shotgun)
    PulseLaser,
    /// lock onto enemy ships (implement countermeasures?)
    HomingMissile,
}

// systems

/// if space bar pressed, have player main gun shoot
fn shoot_manual_gun(
    player: Query<(&RotationBody, &Children), With<Player>>,
    guns: Query<(&Gun, &GlobalTransform)>,
    bullet_assets: Res<BulletAssets>,
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) -> Result<(), BevyError> {
    if keys.just_pressed(KeyCode::Space) {
        let (player_rotation, children) = player.single()?;

        for &child in children {
            if let Ok((gun, transform)) = guns.get(child) {
                gun.shoot_bullet(
                    &mut commands,
                    transform.translation().xy(),
                    player_rotation.rotation,
                    &bullet_assets,
                );
            }
        }
    }
    Ok(())
}
