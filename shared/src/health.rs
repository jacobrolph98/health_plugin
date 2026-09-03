use bevy::prelude::*;

use crate::{messages::{DamageMessage, DeathMessage}, protocol::ProtocolPlugin};

pub struct SharedHealthPlugin;

impl Plugin for SharedHealthPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(ProtocolPlugin)
            .add_message::<DamageMessage>()
            .add_message::<DeathMessage>()
            ;
    }
}