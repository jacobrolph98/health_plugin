use bevy::prelude::*;
use health_shared::health::SharedHealthPlugin;

pub struct ClientHealthPlugin;

impl Plugin for ClientHealthPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(SharedHealthPlugin)
            ;
    }
}