#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod bullet;
mod characters;
mod weapon;

use bevy::asset::AssetMetaCheck;
use bevy::{prelude::*, window::PrimaryWindow};
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
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
        )
        .add_plugins(BulletPlugin)
        .add_plugins(CharacterPlugin)
        .init_resource::<MyWorldCoords>()
        .add_systems(Startup, (setup, spawn_center))
        .add_systems(Update, (camera_track_player, my_cursor_system))
        .run();
}

/// We will store the world position of the mouse cursor here.
#[derive(Resource, Default)]
struct MyWorldCoords(Vec2);

/// Used to help identify our main camera
#[derive(Component)]
struct MainCamera;

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        MainCamera,
        Name::new("MainCamera"),
    ));
}

    pub fn spawn_center(mut commands: Commands) {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(1.2, 1.2, 1.2),
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

    fn my_cursor_system(
        mut mycoords: ResMut<MyWorldCoords>,
        // query to get the window (so we can read the current cursor position)
        window_q: Query<&Window, With<PrimaryWindow>>,
        // query to get camera transform
        camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    ) {
        // get the camera info and transform
        // assuming there is exactly one main camera entity, so Query::single() is OK
        let (camera, camera_transform) = camera_q.single();

        // There is only one primary window, so we can similarly get it from the query:
        let window = window_q.single();

        // check if the cursor is inside the window and get its position
        // then, ask bevy to convert into world coordinates, and truncate to discard Z
        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            mycoords.0 = world_position;
            eprintln!("World coords: {}/{}", world_position.x, world_position.y);
        }
    }