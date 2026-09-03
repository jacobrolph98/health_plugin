use bevy::prelude::*;

#[derive(Message)]
pub struct DamageMessage {
    pub target: Entity,
    pub source: Entity,
    pub amount: f32
}

#[derive(Message)]
pub struct DeathMessage {
    pub entity: Entity
}