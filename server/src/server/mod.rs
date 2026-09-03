use bevy::prelude::*;
use health_shared::health::SharedHealthPlugin;

use crate::server::systems::{deal_damage, deal_healing};

mod systems;

pub struct ServerHealthPlugin;

impl Plugin for ServerHealthPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(SharedHealthPlugin)
            .add_systems(Update, (deal_damage, deal_healing))
            ;
    }
}