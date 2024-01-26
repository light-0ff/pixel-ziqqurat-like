use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Component)]
pub struct Player;

pub const PLAYER_SPEED: f32 = 500.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
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
