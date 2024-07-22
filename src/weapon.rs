use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{characters::player::Player, MyWorldCoords};

// use crate::bullet::Bullet;

#[derive(Debug)]
pub enum WeaponType {
    Wand,
    Staff,
    Thome,
    Alchemy,
}

#[derive(Component, Debug)]
pub struct Weapon {
    pub weapon_type: WeaponType,
    pub damage: i32,
    pub firerate: f32,
    // pub sprite: Handle<Image>,
    // pub bullet_sprite: Handle<Image>,
    // pub name: Name,
    // pub description: String,
}
#[derive(Component)]
pub struct Inventory {
    pub slots: [Option<Weapon>; 5],
    pub active_slot: usize,
}

// weapon aim traker
// uses weapon sprite to orbit around player
#[derive(Component)]
pub struct Triangle;
const ORBIT_DISTANCE: f32 = 50.;
pub fn spawn_weapon_aim(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let triangle = Mesh2dHandle(meshes.add(Triangle2d::new(
        Vec2::Y * 15.0,
        Vec2::new(-15.0, -15.0),
        Vec2::new(15.0, -15.0),
    )));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: triangle,
            material: materials.add(ColorMaterial::from(Color::hsl(360., 0.95, 0.7))),
            transform: Transform::from_xyz(0., ORBIT_DISTANCE, 0.1),
            ..default()
        },
        Triangle,
    ));
}

// follows mouse
// rotates sprite
pub fn move_weapon_aim(
    mycoords: ResMut<MyWorldCoords>,
    player_query: Query<&Transform, With<Player>>,
    mut triangle_query: Query<&mut Transform, (With<Triangle>, Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let player_pos = player_transform.translation.xy();
        let mouse_pos = mycoords.0; // Mouse position in world coordinates

        // Calculate the angle between player and mouse.
        let angle = (mouse_pos.y - player_pos.y).atan2(mouse_pos.x - player_pos.x);

        // Calculate new triangle position.
        let triangle_x = player_pos.x + ORBIT_DISTANCE * angle.cos();
        let triangle_y = player_pos.y + ORBIT_DISTANCE * angle.sin();

        // Update triangle's position and rotation.
        for mut transform in triangle_query.iter_mut() {
            transform.translation.x = triangle_x;
            transform.translation.y = triangle_y;
            transform.rotation = Quat::from_rotation_z(angle - 90f32.to_radians());
        }
    }
}
