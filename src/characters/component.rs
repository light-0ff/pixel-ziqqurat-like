use bevy::prelude::*;

#[derive(Component)]
pub struct FromPlayer;

#[derive(Component)]
pub struct FromEnemy;

#[derive(Component)]
pub struct Health(pub i32);

impl Health {
    pub fn take_damage(&mut self, damage: i32) {
        self.0 -= damage;
    }
    pub fn heal(&mut self, heal: i32) {
        self.0 += heal;
    }
}