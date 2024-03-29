use super::{
    component::{FromPlayer, Health},
    enemy::Enemy,
};
use crate::{bullet::Bullet, weapon::Pistol, WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::{
    math::Vec3Swizzles, prelude::*, sprite::collide_aabb::collide, utils::HashSet,
    window::PrimaryWindow,
};

pub const PLAYER_SPEED: f32 = 500.0;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player).add_systems(
            Update,
            (
                player_movement,
                // player_shoot,
                test_shoot,
                player_laser_hit_enemy_system,
            ),
        );
    }
}

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.8, 1.0),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.1),
            texture: asset_server.load("ducky.png"),
            ..default()
        },
        Player,
        Health(6),
        Name::new("Player"),
        Pistol {
            fire_rate: 1.0,
            damage: 5.0,
        },
    ));
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
    q_windows: Query<&Window, With<PrimaryWindow>>,
    // asset_server: Res<AssetServer>,  for bullets
) {
    if let Ok(transform) = player_query.get_single() {
        let mut direction = Vec2::ZERO;

        if keyboard_input.pressed(KeyCode::Space) {
            let x = transform.translation.x;
            let y = transform.translation.y;
            // get cursor/ joystick/ etc. position and change direction
            if let Some(position) = q_windows.single().cursor_position() {
                direction += Vec2::new(
                    -WINDOW_WIDTH / 2.0 + position.x,
                    WINDOW_HEIGHT / 2.0 - position.y,
                );
            }

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
                FromPlayer,
            ));
        }
    }
}

pub fn test_shoot(
    commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    player_query: Query<(&Transform, &Pistol), With<Player>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok((transform, shotgun)) = player_query.get_single() {
        if keyboard_input.pressed(KeyCode::Space) {
            shotgun.shoot(
                commands,
                transform.translation,
                q_windows.single().cursor_position(),
            );
        }
    }
}

fn player_laser_hit_enemy_system(
    mut commands: Commands,
    laser_query: Query<
        (
            Entity,
            &Transform,
            // &SpriteSize
        ),
        (With<Bullet>, With<FromPlayer>),
    >,
    mut enemy_query: Query<
        (
            Entity,
            &Transform,
            // &SpriteSize
            &mut Health,
        ),
        With<Enemy>,
    >,
) {
    let mut despawned_entities: HashSet<Entity> = HashSet::new();

    // iterate trough lasers
    for (
        laser_entity,
        laser_tf,
        // laser_size
    ) in laser_query.iter()
    {
        if despawned_entities.contains(&laser_entity) {
            continue;
        }
        let laser_scale = Vec2::from(laser_tf.scale.xy());

        // iterate trough enemies
        enemy_query.iter_mut().for_each(
            |(
                enemy_entity,
                enemy_tf,
                // enemy_size
                mut enemy_health,
            )| {
                if despawned_entities.contains(&enemy_entity)
                    || despawned_entities.contains(&laser_entity)
                {
                    return;
                }
                let enemy_scale = Vec2::from(enemy_tf.scale.xy());
                let laser_size = Vec2::new(25.0, 25.0);
                let enemy_size = Vec2::new(50.0, 50.0);
                // determine if collision
                let collision = collide(
                    laser_tf.translation,
                    laser_size * laser_scale,
                    enemy_tf.translation,
                    enemy_size * enemy_scale,
                );

                // perform collision
                if let Some(_) = collision {
                    commands.entity(laser_entity).despawn();
                    despawned_entities.insert(laser_entity);

                    if enemy_health.0 > 1 {
                        enemy_health.take_damage(1);
                        return;
                    }
                    commands.entity(enemy_entity).despawn();
                    despawned_entities.insert(enemy_entity);

                    // spawn explosionToSpawn
                    // commands.spawn(ExplosionToSpawn(enemy_tf.translation.clone()));
                }
            },
        );
    }
}
