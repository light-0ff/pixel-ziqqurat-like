use bevy::prelude::*;

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
