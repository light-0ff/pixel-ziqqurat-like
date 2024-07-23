use crate::{characters::Velocity, player::components::Player};

use super::characters::Health;
use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct StalkerBehavior;

enum EnemyState {
    Idle,
    Chase,
    Search,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_enemy, spawn_stalker_enemy))
            .add_systems(Update, stalker_follow_player);
    }
}

pub fn spawn_enemy(mut commands: Commands) {
    let sprite_width = 50.0; //remove later
    let sprite_height = 50.0; //remove later
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(1.0, 0.5, 0.5),
                custom_size: Some(Vec2::new(sprite_width, sprite_height)),
                ..Default::default()
            },
            transform: Transform::from_xyz(
                -640.0 + (sprite_width / 2.0) + 10.0,
                400.0 - (sprite_height / 2.0) - 10.0,
                0.1,
            ),
            ..default()
        },
        Enemy,
        Health { current: 5, max: 5 },
        Name::new("Stationary Enemy"),
    ));
}

pub fn spawn_stalker_enemy(mut commands: Commands) {
    let sprite_width = 50.;
    let sprite_height = 50.;
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(1.0, 0.5, 0.5),
                custom_size: Some(Vec2::new(sprite_width, sprite_height)),
                ..default()
            },
            transform: Transform::from_xyz(
                640.0 - (sprite_width / 2.0) - 10.0,
                400.0 - (sprite_height / 2.0) - 10.0,
                0.1,
            ),
            ..default()
        },
        Enemy,
        Velocity(Vec3::new(150.0, 150.0, 1.0)),
        StalkerBehavior,
        Health { current: 5, max: 5 },
        Name::new("Stalker Enemy"),
    ));
}

pub fn stalker_follow_player(
    mut enemy_query: Query<
        (&mut Transform, &Velocity),
        (With<Enemy>, With<StalkerBehavior>, Without<Player>),
    >,
    player_query: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(player_tf) = player_query.get_single() {
        for (
            mut enemy_tf, // , mut stalker_behavior
            velocity,
        ) in enemy_query.iter_mut()
        {
            let distance = enemy_tf.translation.distance(player_tf.translation);

            let scanning_radius = 300.0;

            if distance <= scanning_radius {
                // Calculate direction vector towards the player
                let direction = player_tf.translation - enemy_tf.translation;

                // Update enemy position based on velocity
                enemy_tf.translation += direction.normalize() * velocity.0 * time.delta_seconds();
            }
        }
    }
}
