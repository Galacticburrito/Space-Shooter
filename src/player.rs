use crate::{
    AppState, SystemUpdateSet,
    body::{Body, RotationBody},
    ship::{self, ShipType},
    ship_composition::{
        bullet::BulletAssets,
        engine::{Engine, EngineType},
        gun::Gun,
    },
};
use bevy::prelude::*;

pub struct PlayerPlugin {}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameReady), setup);
        app.add_systems(
            Update,
            (player_accelerate, player_rotate, player_shoot).in_set(SystemUpdateSet::Main),
        );
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let player_ship = ship::spawn_ship(
        &ShipType::Interceptor,
        Body {
            mass: 100.,
            ..Default::default()
        },
        &mut materials,
        &mut meshes,
        &mut commands,
    );

    commands
        .entity(player_ship)
        .insert((Name::new("Player"), Player {}));
}

#[derive(Component)]
pub struct Player {}

fn player_accelerate(
    mut engine_query: Query<&mut Engine>,
    player_children: Query<&Children, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
) -> Result<(), BevyError> {
    let p_children = player_children.single()?;

    let w_pressed = keys.pressed(KeyCode::KeyW);
    let s_pressed = keys.pressed(KeyCode::KeyS);

    let throttle_action = match (w_pressed, s_pressed) {
        (true, true) | (false, false) => |engine: &mut Engine| engine.hold_throttle(),
        (true, false) => |engine: &mut Engine| engine.full_throttle(),
        (false, true) => |engine: &mut Engine| engine.no_throttle(),
    };

    for &child in p_children {
        if let Ok(mut engine) = engine_query.get_mut(child) {
            if engine.engine_type == EngineType::Main {
                throttle_action(&mut engine);
            }
        }
    }
    Ok(())
}

fn player_rotate(
    mut engine_query: Query<&mut Engine>,
    player_children: Query<&Children, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
) -> Result<(), BevyError> {
    let p_children = player_children.single()?;

    let a_pressed = keys.pressed(KeyCode::KeyA);
    let d_pressed = keys.pressed(KeyCode::KeyD);

    let throttle_action = match (a_pressed, d_pressed) {
        (true, true) | (false, false) => |engine: &mut Engine| engine.hold_throttle(),
        (true, false) => |engine: &mut Engine| engine.full_throttle(),
        (false, true) => |engine: &mut Engine| engine.min_throttle(),
    };

    for &child in p_children {
        if let Ok(mut engine) = engine_query.get_mut(child) {
            if engine.engine_type == EngineType::Thruster {
                throttle_action(&mut engine);
            }
        }
    }
    Ok(())
}

/// if space bar pressed, have player main gun shoot
/// TODO: move most of this logic to gun and bullet.rs
fn player_shoot(
    player: Query<(Entity, &Children), With<Player>>,
    guns: Query<(&Gun, &Body, &GlobalTransform, &RotationBody)>,
    bullet_assets: Res<BulletAssets>,
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) -> Result<(), BevyError> {
    if keys.just_pressed(KeyCode::Space) {
        let (player, p_children) = player.single()?;

        for &child in p_children {
            if let Ok((gun, g_body, g_transform, g_rot_body)) = guns.get(child) {
                gun.try_shoot(
                    &player,
                    &mut commands,
                    g_body,
                    g_transform,
                    g_rot_body,
                    &bullet_assets,
                );
            }
        }
    }
    Ok(())
}
