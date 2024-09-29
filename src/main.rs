use bevy::prelude::*;

pub const WINDOW_WIDTH: f32 = 640.0 * 2.0;
pub const WINDOW_HEIGHT: f32 = 360.0 * 2.0;
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
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), Name::new("MainCamera")));
}
