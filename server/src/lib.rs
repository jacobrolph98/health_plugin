use bevy::prelude::*;
use health_shared::SharedHealthPlugin;

use crate::systems::{deal_damage, deal_healing};

mod systems;

pub struct ServerHealthPlugin;

impl Plugin for ServerHealthPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(SharedHealthPlugin)
            .add_systems(Update, (deal_healing, deal_damage).chain())
            ;
    }
}