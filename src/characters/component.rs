use bevy::prelude::*;

#[derive(Component)]
pub struct FromPlayer;

#[derive(Component)]
pub struct FromEnemy;

#[derive(Component)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Component)]
pub struct StaffAmo{
    pub current: i32,
    pub max: i32,
}

#[derive(Component)]
pub struct ThomeAmo{
    pub current: i32,
    pub max: i32,
}

#[derive(Component)]
pub struct AlchemyAmo{
    pub current: i32,
    pub max: i32,
}

