use crate::ecs::components::Stats;
use crate::ecs::components::combat::{
    AttackComponent, DefenseComponent, HealthComponent, MeleeAttacker, RangedAttacker,
};
use common::Health;
use pathfinding::num_traits::real::Real;
use specs::{Entities, Join, ReadStorage, System, WriteStorage};

pub struct StatSyncSystem;

impl<'a> System<'a> for StatSyncSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Stats>,
        ReadStorage<'a, MeleeAttacker>,
        ReadStorage<'a, RangedAttacker>,
        WriteStorage<'a, AttackComponent>,
        WriteStorage<'a, DefenseComponent>,
        WriteStorage<'a, HealthComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            stats_storage,
            melees,
            ranges,
            mut attack_storage,
            mut defense_storage,
            mut health_storage,
        ) = data;

        // strength increases damage for melee attacks
        for (entity, stats, _) in (&entities, &stats_storage, &melees).join() {
            if let Some(attack) = attack_storage.get_mut(entity) {
                attack.damage += stats.strength / 10;
                attack.hit_rating += stats.agility * 2;
            }
        }
        // agility increases damage for ranged attacks
        for (entity, stats, _) in (&entities, &stats_storage, &ranges).join() {
            if let Some(attack) = attack_storage.get_mut(entity) {
                attack.damage += stats.agility / 10;
                attack.hit_rating += stats.agility * 2;
            }
        }
        // agility increases hit rating for all attacks
        for (entity, stats) in (&entities, &stats_storage).join() {
            if let Some(health_component) = health_storage.get_mut(entity) {
                let new_max_hp = 2 + stats.strength * 2;
                if let Health::Alive { hp, max_hp } = &mut health_component.0 {
                    let hp_ratio = *hp as f32 / (*max_hp).max(1) as f32;
                    *max_hp = new_max_hp;
                    *hp = (new_max_hp as f32 * hp_ratio).round() as u32;
                }
            }

            if let Some(defense) = defense_storage.get_mut(entity) {
                //TODO: this math
                // defense.defense = 2 + stats.strength;
                defense.evasion = 5 + stats.agility / 2;
            }
        }
    }
}
