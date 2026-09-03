use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize)]
pub struct Health(pub f32);

#[derive(Component, Serialize, Deserialize)]
pub struct MaxHealth(pub f32);

/// Ignore damage below this value
#[derive(Component, Serialize, Deserialize)]
pub struct Threshold(pub f32);

/// Reduce damage to this value
#[derive(Component, Serialize, Deserialize)]
pub struct Cap(pub f32);

/// Subtract armour value from damage
#[derive(Component, Serialize, Deserialize)]
pub struct Armour(pub f32);