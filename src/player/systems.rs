use super::components::Player;
use crate::{
    bullet::Bullet,
    characters::{AlchemyAmo, FromPlayer, Health, StaffAmo, ThomeAmo, Velocity},
    enemy::Enemy,
    weapon::{Inventory, Triangle, Weapon, WeaponType},
};
use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
    utils::HashSet,
};

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
                color: Color::srgb(0.5, 0.8, 1.0),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.2),
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
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &Velocity), With<Player>>,
    time: Res<Time>,
) {
    if let Ok((mut transform, velocity)) = player_query.get_single_mut() {
        let mut movement = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            movement.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            movement.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            movement.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            movement.y -= 1.0;
        }

        transform.translation += movement.normalize_or_zero() * velocity.0 * time.delta_seconds();
    }
}

pub fn player_shoot(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_query: Query<(&Transform, &mut Inventory), With<Player>>,
    triangle_query: Query<&Transform, (With<Triangle>, Without<Player>)>,
    // asset_server: Res<AssetServer>,  for bullets
) {
    if let Ok((player_transform, inventory)) = player_query.get_single() {
        let mut direction = Vec2::ZERO;
        let triangle_x = triangle_query.single().translation.x;
        let triangle_y = triangle_query.single().translation.y;
        if keyboard_input.pressed(KeyCode::Space) {
            let player_x = player_transform.translation.x;
            let player_y = player_transform.translation.y;
            // get cursor/ joystick/ etc. position and change direction
            direction.x = triangle_x - player_x;
            direction.y = triangle_y - player_y;
            direction = direction.normalize();

            // delete
            println!(
                "{:#?}",
                inventory.slots[inventory.active_slot].as_ref().unwrap()
            );
            // calculate exact damage(crit etc) and pass it to bullet?
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(player_x, player_y, 0.0),
                    // texture: asset_server.load("sprites/ball_red.png"),
                    sprite: Sprite {
                        color: Color::srgb(1.0, 1.0, 0.0),
                        custom_size: Some(Vec2::new(25.0, 25.0)),
                        ..default()
                    },
                    ..default()
                },
                Bullet {
                    direction: direction,
                },
                FromPlayer,
            ));
        }
    }
}

pub fn player_laser_hit_enemy_system(
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
                let collision = Aabb2d::new(
                    laser_tf.translation.truncate(),
                    laser_size * laser_scale / 2.,
                )
                .intersects(&Aabb2d::new(
                    enemy_tf.translation.truncate(),
                    enemy_size * enemy_scale / 2.,
                ));

                // perform collision
                if collision {
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

pub fn test_spawn_weapon(
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
                color: Color::srgb(0.5, 0.8, 1.0),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform::from_xyz(100.0, 0.0, 0.1),
            // texture: asset_server.load("ducky.png"),
            ..default()
        },
        wand,
        Name::new("Abra kadabra"),
    ));
}
