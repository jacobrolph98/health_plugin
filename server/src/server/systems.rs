use bevy::prelude::*;
use shared::{components::{Armour, Cap, Health, MaxHealth, Threshold}, messages::{DamageMessage, DeathMessage, HealMessage}};

pub fn deal_damage(
    mut damage_msgs: MessageReader<DamageMessage>,
    mut health_q: Query<(&mut Health, Option<&Threshold>, Option<&Cap>, Option<&Armour>)>,
    mut death_msgs: MessageWriter<DeathMessage>,
) {
    for damage in damage_msgs.read() {
        let (mut health, has_threshold, has_cap, has_armour) = health_q.get_mut(damage.target).expect("Target entity has no health");
        let minimum = has_threshold.unwrap_or(&Threshold(0.)).0;
        let maximum = has_cap.unwrap_or(&Cap(f32::MAX)).0;
        let armour = has_armour.unwrap_or(&Armour(0.)).0;
        let dealt_damage = damage.amount;
        let dealt_damage = if dealt_damage >= minimum { dealt_damage } else { 0. };
        let dealt_damage = if dealt_damage > maximum { maximum } else { dealt_damage };
        let dealt_damage = dealt_damage - armour;
        let dealt_damage = if dealt_damage >= health.0 { 
            death_msgs.write(DeathMessage { entity: damage.target });
            health.0
         } else { 
            dealt_damage 
        };
        health.0 -= dealt_damage;
    }
}

pub fn deal_healing(
    mut heal_msgs: MessageReader<HealMessage>,
    mut health_q: Query<(&mut Health, Option<&MaxHealth>)>,
) {
    for healing in heal_msgs.read() {
        let (mut health, has_max) = health_q.get_mut(healing.target).expect("Target entity has no health");
        let max = has_max.unwrap_or(&MaxHealth(f32::MAX)).0;
        let dealt_healing = if healing.amount + health.0 > max { max - health.0 } else { healing.amount };
        health.0 += dealt_healing;
    }
}