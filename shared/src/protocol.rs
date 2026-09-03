use bevy::prelude::*;
use lightyear::prelude::*;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use crate::components::{Health, MaxHealth, Threshold, Cap, Armour};
pub struct ProtocolPlugin;

pub const TICK_DURATION: f32 = 1./64.;
pub const LOCAL_SERVER_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 5000);

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        app.component::<Health>().replicate();
        app.component::<MaxHealth>().replicate();
        app.component::<Threshold>().replicate();
        app.component::<Cap>().replicate();
        app.component::<Armour>().replicate();
    }
}