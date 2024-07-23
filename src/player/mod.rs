pub(crate) mod components;
mod systems;

use bevy::prelude::*;
use systems::{
    player_laser_hit_enemy_system, player_movement, player_shoot, spawn_player, test_spawn_weapon,
};

use crate::weapon::{move_aim_triangle, spawn_aim_triangle};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Startup, (test_spawn_weapon, spawn_aim_triangle))
            .add_systems(FixedUpdate, player_movement)
            .add_systems(
                Update,
                (
                    player_shoot,
                    player_laser_hit_enemy_system,
                    move_aim_triangle,
                ),
            );
    }
}
