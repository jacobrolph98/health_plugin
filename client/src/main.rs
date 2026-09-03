use bevy::prelude::*;
use client::client::ClientHealthPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            ClientHealthPlugin
        ))
        .run();
}
