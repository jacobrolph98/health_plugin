use bevy::prelude::*;

/// Intended to be written by external API consumer - Game decides when to deal damage
#[derive(Message)]
pub struct DamageMessage {
    pub target: Entity,
    pub source: Entity,
    pub amount: f32
}

/// Intended to be written by external API consumer - Game decides when to heal
#[derive(Message)]
pub struct HealMessage {
    pub target: Entity,
    pub source: Entity,
    pub amount: f32
}

/// Intended to be read by external API consumer - This library writes these messages when appropriate, game decides what to do
#[derive(Message)]
pub struct DeathMessage {
    pub entity: Entity
}