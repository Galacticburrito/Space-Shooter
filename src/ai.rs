use crate::{
    SystemUpdateSet,
    body::Body,
    player::Player,
    ship::{self, Ship, ShipType},
};
use bevy::prelude::*;

pub struct AiPlugin {}

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, move_ai.in_set(SystemUpdateSet::Main));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    ship::spawn_ship(
        &ShipType::Interceptor,
        Body::new(1000., Vec2::new(-200., 0.), Vec2::new(10., 0.)),
        &mut materials,
        &mut meshes,
        &mut commands,
    );

    ship::spawn_ship(
        &ShipType::Interceptor,
        Body::new(1000., Vec2::new(-200., 0.), Vec2::new(10., 0.)),
        &mut materials,
        &mut meshes,
        &mut commands,
    );
}

#[derive(Component)]
pub struct Ai {}

/// move ai ships to enemy (just Player for now...)
fn move_ai(
    ai: Query<(&Ship, &Body), With<Ai>>,
    enemy: Query<(&Body), With<Player>>,
) -> Result<(), BevyError> {
    let enemy = enemy.single()?;
    for (ship, body) in &ai {
        // move ship to player, another system to shoot?
    }
    Ok(())
}
