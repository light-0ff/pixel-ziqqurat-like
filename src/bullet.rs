use bevy::prelude::*;

pub const BULLET_SPEED: f32 = 310.0;

#[derive(Component)]
pub struct Bullet {
    pub direction: Vec2,
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, bullet_movement);
    }
}

pub fn bullet_movement(mut bullet_query: Query<(&mut Transform, &Bullet)>, time: Res<Time>) {
    for (mut transform, bullet) in bullet_query.iter_mut() {
        let direction = Vec3::new(bullet.direction.x, bullet.direction.y, 0.0);
        transform.translation += direction * BULLET_SPEED * time.delta_seconds();
    }
}
