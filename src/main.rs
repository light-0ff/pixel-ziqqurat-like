use bevy::prelude::*;

pub const SCALE_MULTIPLIER: f32 = 2.0;
pub const WINDOW_WIDTH: f32 = 640.0 * SCALE_MULTIPLIER;
pub const WINDOW_HEIGHT: f32 = 360.0 * SCALE_MULTIPLIER;
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Some game window".into(),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_player)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), Name::new("MainCamera")));
}

#[derive(Component)]
struct Player;
pub const PLAYER_WIDTH: f32 = 32.0 * SCALE_MULTIPLIER;
pub const PLAYER_HEIGHT: f32 = 32.0 * SCALE_MULTIPLIER;

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("knight.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 8, 8, None, None);
    let texture_atlas_layout = texture_atlases.add(layout);

    commands.spawn((
        SpriteBundle {
            texture,
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: 0,
        },
        Player,
        Name::new("Player"),
    ));
}
