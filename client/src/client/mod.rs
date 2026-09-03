use bevy::prelude::*;
use shared::health::SharedHealthPlugin;

pub struct ClientHealthPlugin;

impl Plugin for ClientHealthPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(SharedHealthPlugin)
            ;
    }
}