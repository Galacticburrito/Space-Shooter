use crate::{
    AppState, SystemUpdateSet,
    data_config::{
        blueprint::{BlueprintRegistry, BlueprintTable, BlueprintType},
        data::{DataRegistry, DataTable},
    },
    ship,
    ship_composition::{
        bullet::BulletAssets,
        engine::{Engine, EngineType},
        gun::Gun,
        sonar::Sonar,
    },
    velocity::global::GlobalVelocity,
    velocity::{AngularVelocity, Velocity},
};
use bevy::prelude::*;

pub struct PlayerPlugin {}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameReady), setup);
        app.add_systems(
            Update,
            (player_accelerate, player_rotate, player_shoot, player_sonar)
                .in_set(SystemUpdateSet::Main),
        );
    }
}

fn setup(
    mut commands: Commands,
    //mut meshes: ResMut<Assets<Mesh>>,
    //mut materials: ResMut<Assets<ColorMaterial>>,
    b_registry: Res<BlueprintRegistry>,
    b_table: Res<Assets<BlueprintTable>>,
    d_registry: Res<DataRegistry>,
    d_table: Res<Assets<DataTable>>,
) {
    let Some(player_ship) = ship::spawn_ship_from_blueprint(
        "ship_1",
        &BlueprintType::TransformVelocity(
            Transform::default(),
            Velocity::ZERO,
            AngularVelocity(0.),
        ),
        &b_registry,
        &b_table,
        &d_registry,
        &d_table,
        &mut commands,
    ) else {
        warn!("player ship unable to spawn!");
        return;
    };

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
        (true, true) | (false, false) => |engine: &mut Engine| {
            engine.hold_throttle();
        },
        (true, false) => |engine: &mut Engine| {
            engine.full_throttle();
        },
        (false, true) => |engine: &mut Engine| {
            engine.no_throttle();
        },
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
        (true, true) | (false, false) => |engine: &mut Engine| {
            // if ship is rotating less than the rotate_threshold, clamp it to 0 so no
            // annoying drift
            let rotate_threshold = 0.01;
            if engine.current_thrust() < rotate_threshold {
                engine.no_throttle();
            }
            engine.hold_throttle();
        },
        (true, false) => |engine: &mut Engine| {
            engine.full_throttle();
        },
        (false, true) => |engine: &mut Engine| {
            engine.min_throttle();
        },
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
fn player_shoot(
    player: Query<(Entity, &Children), With<Player>>,
    guns: Query<(&Gun, &GlobalTransform, &GlobalVelocity)>,
    bullet_assets: Res<BulletAssets>,
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) -> Result<(), BevyError> {
    if keys.just_pressed(KeyCode::Space) {
        let (player, p_children) = player.single()?;

        for &child in p_children {
            if let Ok((gun, g_transform, g_velocity)) = guns.get(child) {
                gun.try_shoot(
                    &player,
                    &mut commands,
                    g_transform,
                    g_velocity,
                    &bullet_assets,
                );
            }
        }
    }
    Ok(())
}

/// if q key pressed, sonar pulse erupts
fn player_sonar(
    player: Query<&Children, With<Player>>,
    sonars: Query<(Entity, &Sonar)>,
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) -> Result<(), BevyError> {
    if keys.just_pressed(KeyCode::KeyQ) {
        let p_children = player.single()?;

        for &child in p_children {
            if let Ok((entity, sonar)) = sonars.get(child) {
                sonar.pulse(entity, &mut commands);
            }
        }
    }
    Ok(())
}
