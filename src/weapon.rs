use std::sync::Arc;

use bevy::prelude::*;

use crate::{bullet::Bullet, characters::component::FromPlayer, WINDOW_HEIGHT, WINDOW_WIDTH};


#[derive(Component)]
pub struct Pistol {
    pub fire_rate: f32,
    pub damage: f32,
}

impl Pistol {
    pub fn shoot(
        &self,
        mut commands: Commands,
        player_position: Vec3,
        cursor_position: Option<Vec2>,
    ) {
        let mut direction = Vec2::ZERO;
        if let Some(position) = cursor_position {
            direction += Vec2::new(
                -WINDOW_WIDTH / 2.0 + position.x,
                WINDOW_HEIGHT / 2.0 - position.y,
            );
            println!("Start \t{} --------------------------\n\n\n", direction);
        }
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(player_position.x, player_position.y, 0.0),
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
