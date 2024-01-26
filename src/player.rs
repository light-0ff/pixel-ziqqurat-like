use bevy::prelude::*;
use crate::bullet::Bullet;

pub const PLAYER_SPEED: f32 = 500.0;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (player_movement, player_shoot));
    }
}

pub fn spawn_player(
    mut commands: Commands,
    // asset_server: Res<AssetServer>, will need for sprites
) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.8, 1.0),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.1),
                ..default()
            },
            ..default()
        })
        .insert(Player)
        .insert(Name::new("Player"));
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }
        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn player_shoot(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    player_query: Query<&mut Transform, With<Player>>,
    // asset_server: Res<AssetServer>,  for bullets
) {
    if let Ok(transform) = player_query.get_single() {
        let mut direction = Vec2::ZERO;

        if keyboard_input.pressed(KeyCode::Space) {
            let x = transform.translation.x;
            let y = transform.translation.y;
            // get cursor/ joystick/ etc. position and change direction
            direction += Vec2::new(x + 1.0, y);

            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(x, y, 0.0),
                    // texture: asset_server.load("sprites/ball_red.png"),
                    sprite: Sprite {
                        color: Color::rgb(1.0, 1.0, 0.0),
                        custom_size: Some(Vec2::new(25.0, 25.0)),
                        ..default()
                    },
                    ..default()
                },
                Bullet {
                    direction: direction.normalize(),
                },
            ));
        }
    }
}
