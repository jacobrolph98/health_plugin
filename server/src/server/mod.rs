use bevy::prelude::*;
use shared::health::SharedHealthPlugin;

use crate::server::systems::deal_damage;

mod systems;

pub struct ServerHealthPlugin;

impl Plugin for ServerHealthPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(SharedHealthPlugin)
            .add_systems(Update, deal_damage)
            ;
    }
}