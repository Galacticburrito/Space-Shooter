use crate::{
    Health, PalColor, SystemUpdateSet,
    body::{Body, RotationBody},
    collision::Collider,
    ship::{self, ShipType},
    ship_composition::{
        engine::{Engine, EngineType},
        gun::{Gun, GunType},
    },
};
use bevy::prelude::*;

pub struct PlayerPlugin {}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            (player_accelerate, player_rotate).in_set(SystemUpdateSet::Main),
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

fn pplayer_rotate(
    mut player: Query<&mut RotationBody, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) -> Result<(), BevyError> {
    let rotate_speed = 1.;

    let mut rot_body = player.single_mut()?;
    if keys.pressed(KeyCode::KeyA) {
        rot_body.angular_velocity += rotate_speed * time.delta_secs();
    }
    if keys.pressed(KeyCode::KeyD) {
        rot_body.angular_velocity -= rotate_speed * time.delta_secs();
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
