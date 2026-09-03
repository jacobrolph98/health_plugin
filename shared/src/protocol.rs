use bevy::prelude::*;
use lightyear::prelude::*;
use crate::components::*;

pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        app.component::<Health>().replicate();
        app.component::<MaxHealth>().replicate();
        app.component::<Threshold>().replicate();
        app.component::<Cap>().replicate();
        app.component::<Armour>().replicate();
    }
}