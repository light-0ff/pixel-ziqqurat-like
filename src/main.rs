use bevy::{prelude::*, time::Stopwatch};

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
        .add_systems(
            Update,
            (handle_player_input, update_gun_position, handle_gun_input),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), Name::new("MainCamera")));
}

#[derive(Component)]
struct Player;
#[derive(Component)]
struct Gun;
#[derive(Component)]
struct GunTimer(Stopwatch);
// TODO: replace to gun specific firerate
const GUN_FIRERATE: f32 = 0.5;
#[derive(Component)]
struct Bullet;

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
    let texture2: Handle<Image> = asset_server.load("ducky.png");
    commands.spawn((
        Name::new("first gun"),
        SpriteBundle {
            texture: texture2,
            transform: Transform::from_scale(Vec3::splat(SCALE_MULTIPLIER / 20.0)),
            ..default()
        },
        Gun,
        GunTimer(Stopwatch::new()),
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

fn update_gun_position(
    player_query: Query<&Transform, With<Player>>,
    mut gun_query: Query<&mut Transform, (With<Gun>, Without<Player>)>,
) {
    if player_query.is_empty() || gun_query.is_empty() {
        return;
    }

    let player_position = player_query.single().translation;
    let mut gun_transform = gun_query.single_mut();

    gun_transform.translation = player_position;
}

// TODO: rewrite to handle active weapon
fn handle_gun_input(
    mut commands: Commands,
    time: Res<Time>,
    mut gun_query: Query<(&Transform, &mut GunTimer), With<Gun>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
) {
    if gun_query.is_empty() {
        return;
    }

    let (gun_translation, mut gun_timer) = gun_query.single_mut();
    let gun_position = gun_translation.translation.truncate();
    gun_timer.0.tick(time.delta());
    if !mouse_button_input.pressed(MouseButton::Left) {
        return;
    }

    if !(gun_timer.0.elapsed_secs() >= GUN_FIRERATE) {
        return;
    }

    gun_timer.0.reset();
    commands.spawn((
        SpriteBundle {
            // texture,
            transform: Transform::from_xyz(gun_position.x, gun_position.y, 1.0)
                .with_scale(Vec3::splat(5.0 * SCALE_MULTIPLIER)),
            ..default()
        },
        TextureAtlas {
            // layout: texture_atlas_layout,
            index: 8,
            ..Default::default()
        },
        Bullet,
    ));
}

/*
// ONE SPRITESHEET TO RULE THEM ALL
#[derive(Resource)]
struct MyAssets {
    texture_handle: Handle<Image>,
    texture_atlas_layout: Handle<TextureAtlasLayout>,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle = asset_server.load("my_texture.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(25.0, 25.0), 5, 5, None, None);
    let texture_atlas_layout = texture_atlases.add(layout);

    commands.insert_resource(MyAssets {
        texture_handle: texture_handle.clone(),
        texture_atlas_layout: texture_atlas_layout.clone(),
    });

    commands.spawn(SpriteSheetBundle {
        sprite: Sprite::default(),
        texture: texture_handle,
        atlas: TextureAtlas {
            layout: texture_atlas_layout,
            index: 0,
        },
        ..default()
    });
}

    fn spawn_duck(
    mut commands: Commands,
    my_assets: Res<MyAssets>,
) {
    commands.spawn(SpriteSheetBundle {
        sprite: Sprite::default(),
        texture: my_assets.texture_handle.clone(),
        atlas: TextureAtlas {
            layout: my_assets.texture_atlas_layout.clone(),
            index: 1, // Assuming the duck is at index 1
        },
        ..default()
    });
}
*/
