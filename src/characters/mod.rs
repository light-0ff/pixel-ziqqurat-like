pub mod component;
pub mod enemy;
pub mod player;

use bevy::prelude::*;
use enemy::EnemyPlugin;
use player::PlayerPlugin;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerPlugin).add_plugins(EnemyPlugin);
    }
}
