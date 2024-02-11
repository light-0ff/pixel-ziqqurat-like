use super::component::Health;
use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemy);
    }
}

pub fn spawn_enemy(mut commands: Commands) {
    let sprite_width = 50.0; //remove later
    let sprite_height = 50.0; //remove later
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 0.5, 0.5),
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
        Health(5),
        Name::new("Stationary Enemy"),
    ));
}
