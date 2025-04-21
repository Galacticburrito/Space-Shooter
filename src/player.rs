use crate::{
    PalColor, SystemUpdateSet,
    body::{Body, RotationBody},
    gun::{Gun, GunType},
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
    // player
    let shape = meshes.add(Circle::new(5.));
    let color = materials.add(PalColor::Green);
    commands
        .spawn((
            Name::new("Player"),
            Player {},
            Body {
                mass: 100.,
                ..Default::default()
            },
            RotationBody {
                ..Default::default()
            },
            MeshMaterial2d(color),
            Mesh2d(shape),
            Transform::from_translation(Vec3::ZERO),
        ))
        .with_children(|parent| {
            parent.spawn(Gun::new(GunType::Laser));
        });
}

#[derive(Component)]
pub struct Player {}

fn player_accelerate(
    mut query: Query<(&mut Body, &RotationBody), With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let acceleration_speed = 2.;
    for (mut body, rot_body) in &mut query {
        if keys.pressed(KeyCode::KeyW) {
            let x = rot_body.rotation.cos();
            let y = rot_body.rotation.sin();
            body.velocity +=
                Vec2::new(x, y).normalize_or(Vec2::X) * acceleration_speed * time.delta_secs();
        }
    }
}

fn player_rotate(
    mut query: Query<&mut RotationBody, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let rotate_speed = 1.;
    for mut rot_body in &mut query {
        if keys.pressed(KeyCode::KeyA) {
            rot_body.angular_velocity += rotate_speed * time.delta_secs();
        }
        if keys.pressed(KeyCode::KeyD) {
            rot_body.angular_velocity -= rotate_speed * time.delta_secs();
        }
    }
}
