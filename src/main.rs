use bevy::prelude::*;

pub const SCALE_MULTIPLIER: f32 = 2.0;
const WINDOW_WIDTH: f32 = 640.0 * SCALE_MULTIPLIER;
const WINDOW_HEIGHT: f32 = 360.0 * SCALE_MULTIPLIER;
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
        .add_systems(Update, handle_player_input)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), Name::new("MainCamera")));
}

#[derive(Component)]
struct Player;

const PLAYER_SHEET_PATH: &str = "knight.png";
const PLAYER_SPRITE_WIDTH: u32 = 32;
const PLAYER_SPRITE_HEIGHT: u32 = 32;

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load(PLAYER_SHEET_PATH);
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(PLAYER_SPRITE_WIDTH, PLAYER_SPRITE_HEIGHT),
        8,
        8,
        None,
        None,
    );
    let texture_atlas_layout = texture_atlases.add(layout);

    commands.spawn((
        SpriteBundle {
            texture,
            transform: Transform::from_scale(Vec3::splat(SCALE_MULTIPLIER)),
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

const PLAYER_SPEED: f32 = 7.0; // replase to velocity{min, max}

fn handle_player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    let mut transform = match player_query.get_single_mut() {
        Ok(result) => result,
        Err(error) => panic!("handle_player_input Cant find player: {error:?}"),
    };
    let mut movement = Vec2::ZERO;
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
    movement = movement.normalize_or_zero() * PLAYER_SPEED;
    transform.translation += Vec3::new(movement.x, movement.y, 0.0);
}
