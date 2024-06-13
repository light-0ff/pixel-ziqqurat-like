use super::{
    component::{AlchemyAmo, FromPlayer, Health, StaffAmo, ThomeAmo, Velocity},
    enemy::Enemy,
};
use crate::weapon::*;
use crate::{bullet::Bullet, WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::{
    math::Vec3Swizzles, prelude::*, sprite::collide_aabb::collide, utils::HashSet,
    window::PrimaryWindow,
};

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Startup, test_spawn_weapon)
            .add_systems(FixedUpdate, player_movement)
            .add_systems(Update, (player_shoot, player_laser_hit_enemy_system));
    }
}

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    const DEFAULT_WAND: Weapon = Weapon {
        weapon_type: WeaponType::Wand,
        damage: 1,
        firerate: 1.0,
    };
    let mut inventory = Inventory {
        slots: [Some(DEFAULT_WAND), None, None, None, None],
        active_slot: 0,
    };
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
        Name::new("Player"),
        Velocity(Vec3::new(300.0, 300.0, 1.0)),
        Health {
            current: 10,
            max: 10,
        },
        StaffAmo {
            current: 0,
            max: 10,
        },
        ThomeAmo {
            current: 0,
            max: 10,
        },
        AlchemyAmo {
            current: 0,
            max: 10,
        },
        inventory,
    ));
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &Velocity), With<Player>>,
    time: Res<Time>,
) {
    if let Ok((mut transform, velocity)) = player_query.get_single_mut() {
        let mut movement = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            movement.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            movement.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            movement.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            movement.y -= 1.0;
        }

        transform.translation +=
            movement.normalize_or_zero() * velocity.0 * time.delta_seconds();
    }
}

pub fn player_shoot(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    player_query: Query<(&mut Transform, &mut Inventory), With<Player>>,
    windows_q: Query<&Window, With<PrimaryWindow>>,
    // asset_server: Res<AssetServer>,  for bullets
) {
    if let Ok((transform, inventory)) = player_query.get_single() {
        let mut direction = Vec2::ZERO;
        if keyboard_input.pressed(KeyCode::Space) {
            let x = transform.translation.x;
            let y = transform.translation.y;
            // get cursor/ joystick/ etc. position and change direction
            if let Some(position) = windows_q.single().cursor_position() {
                direction += Vec2::new(
                    -WINDOW_WIDTH / 2.0 + position.x,
                    WINDOW_HEIGHT / 2.0 - position.y,
                );
            }

            // delete
            println!(
                "{:#?}",
                inventory.slots[inventory.active_slot].as_ref().unwrap()
            );
            // calculate exact damage(crit etc) and pass it to bullet?
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

                    if enemy_health.current > 1 {
                        // replace damage with weapon damage
                        let damage: i32 = 1;
                        enemy_health.current -= damage;
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

fn test_spawn_weapon(
    mut commands: Commands,
    // asset_server: Res<AssetServer>
) {
    let wand = Weapon {
        weapon_type: WeaponType::Staff,
        damage: 1,
        firerate: 1.0,
    };
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.8, 1.0),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform::from_xyz(100.0, 0.0, 0.5),
            // texture: asset_server.load("ducky.png"),
            ..default()
        },
        wand,
        Name::new("Abra kadabra"),
    ));
}
