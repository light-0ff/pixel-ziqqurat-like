#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod bullet;
mod characters;

use bevy::prelude::*;
use bullet::BulletPlugin;
use characters::{player::Player, CharacterPlugin};

pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 800.0;
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Some game window".into(),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(BulletPlugin)
        .add_plugins(CharacterPlugin)
        .add_systems(Startup, (setup, spawn_center))
        .add_systems(Update, camera_track_player)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), Name::new("MainCamera")));
}

pub fn spawn_center(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.2, 1.2, 1.2),
                custom_size: Some(Vec2::new(60.0, 60.0)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            },
            ..default()
        },
        Name::new("Center"),
    ));
}

fn camera_track_player(
    mut camera_transform: Query<&mut Transform, With<Camera>>,
    player_transform: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    let mut camera_trans = camera_transform.single_mut();
    let playertrans = player_transform.single().translation.truncate();
    let camtrans = camera_trans.translation.truncate();
    camera_trans.translation = camtrans.lerp(playertrans, 0.1).extend(999.0);
}
