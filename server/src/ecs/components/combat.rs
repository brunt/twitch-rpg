use crate::ecs::components::DenseVecStorage;
use crate::ecs::components::NullStorage;
use crate::ecs::components::inventory::Equipment;
use crate::ecs::components::{Component, Stats};
use common::{AttackModifiers, DamageType, DefenseModifiers, Health};
use specs::Entity;

/// the entity that this entity is attacking
#[derive(Component)]
pub struct AttackTarget {
    pub(crate) entity: Entity,
}

#[derive(Debug, Clone)]
pub enum PendingAction {
    Attack {
        target: Entity,
        attacker_position: crate::ecs::components::Position,
        target_position: crate::ecs::components::Position,
        attack_data: AttackComponent,
        is_ranged: bool,
    },
    Spell {
        target: Entity,
        spell_id: u32,
        caster_position: crate::ecs::components::Position,
    },
}

#[derive(Component, Debug)]
pub struct ActionTimer {
    pub remaining: f64,
    pub action: PendingAction,
}

#[derive(Component)]
pub struct DefenseComponent {
    /// the entity's rating of flat damage reduction
    pub defense: u32,
    /// the entity's rating of how often hits against them do no damage
    pub evasion: u32,
}

impl DefenseComponent {
    pub fn from_enemy_difficulty(difficulty: u32) -> Self {
        Self {
            defense: 1 * difficulty,
            evasion: 5 * difficulty,
        }
    }
    // TODO: from buffs as well
    /// Defense component derived from ALL equipped items
    pub fn from_stats_and_items(stats: &Stats, equipment: &Equipment) -> Self {
        // Aggregate item modifiers
        let defense_mods = equipment.slots.values().fold(
            DefenseModifiers {
                damage_reduction: None,
                evasion_rating: None,
            },
            |mut m, item| {
                if let Some(stats) = &item.stats {
                    if let Some(modifiers) = &stats.defense_modifiers {
                        //TODO: multiplicative?
                        m.damage_reduction = Some(
                            m.damage_reduction.unwrap_or(0)
                                + modifiers.damage_reduction.unwrap_or(0),
                        );
                        m.evasion_rating = Some(
                            m.evasion_rating.unwrap_or(0) + modifiers.evasion_rating.unwrap_or(0),
                        );
                    }
                }
                m
            },
        );

        Self {
            defense: defense_mods.damage_reduction.unwrap_or(0).max(0) as u32,
            evasion: (stats.agility / 4) + defense_mods.evasion_rating.unwrap_or(0).max(0) as u32,
        }
    }
}

#[derive(Component, Clone, Debug)]
pub struct AttackComponent {
    /// base damage that this entity can do
    pub damage: u32,
    /// calculates chance of dealing damage (or critical strike) with each hit
    pub hit_rating: u32,
    /// the range in tiles that is how far this entity can attack another entity
    pub range: u32,
    /// the time after which this entity may attack again in milliseconds
    /// base attack cooldown is 2000ms and cooldown subtracts from that value
    pub cooldown: u32,
    /// additional multiplier added to critical damage (base 1.5x)
    pub crit_damage_multiplier: f64,
}

impl AttackComponent {
    pub fn from_enemy_difficulty(difficulty: u32) -> Self {
        Self {
            damage: 2 * difficulty,
            hit_rating: 15 * difficulty,
            range: 1,
            cooldown: 3000 / difficulty,
            crit_damage_multiplier: 0.0,
        }
    }

    /// Attack component derived from ALL equipped items
    pub fn from_stats_and_items(stats: &Stats, equipment: &Equipment) -> Self {
        // Aggregate item modifiers (keep Options here for tooltip flexibility)
        let attack_mods = equipment.slots.values().fold(
            AttackModifiers {
                damage_bonus: None,
                hit_rating_bonus: None,
                range_bonus: None,
                cooldown_reduction_ms: None,
                crit_damage_multiplier: None,
            },
            |mut m, item| {
                if let Some(stats) = &item.stats
                    && let Some(modifiers) = &stats.attack_modifiers
                {
                    m.damage_bonus =
                        Some(m.damage_bonus.unwrap_or(0) + modifiers.damage_bonus.unwrap_or(0));
                    m.hit_rating_bonus = Some(
                        m.hit_rating_bonus.unwrap_or(0) + modifiers.hit_rating_bonus.unwrap_or(0),
                    );
                    m.range_bonus =
                        Some(m.range_bonus.unwrap_or(0) + modifiers.range_bonus.unwrap_or(0));
                    m.cooldown_reduction_ms = Some(
                        m.cooldown_reduction_ms.unwrap_or(0)
                            + modifiers.cooldown_reduction_ms.unwrap_or(0),
                    );
                    m.crit_damage_multiplier = Some(
                        m.crit_damage_multiplier.unwrap_or(0.0)
                            + modifiers.crit_damage_multiplier.unwrap_or(0.0),
                    );
                }
                m
            },
        );

        let base_cooldown = 2000;

        Self {
            damage: (stats.strength as i32 + attack_mods.damage_bonus.unwrap_or(0)).max(1) as u32,
            hit_rating: ((stats.agility * 2 + stats.intelligence / 2) as i32
                + attack_mods.hit_rating_bonus.unwrap_or(0))
            .max(1) as u32,
            range: attack_mods.range_bonus.unwrap_or(1).max(1) as u32,
            cooldown: (base_cooldown - attack_mods.cooldown_reduction_ms.unwrap_or(0))
                .max(200)
                .min(base_cooldown) as u32,
            crit_damage_multiplier: 1.5 + attack_mods.crit_damage_multiplier.unwrap_or(0.0),
        }
    }
}

#[derive(Component)]
pub struct FiredProjectile {
    pub position: tatami_dungeon::Position,
    pub target_position: tatami_dungeon::Position,
    pub damage: u32,
    pub damage_type: DamageType,
}

#[derive(Component)]
#[storage(NullStorage)]
pub struct MeleeAttacker;

#[derive(Component)]
#[storage(NullStorage)]
pub struct RangedAttacker;

#[derive(Debug, Component)]
pub struct HealthComponent(pub Health);

impl Default for HealthComponent {
    fn default() -> Self {
        Self(Health::Alive { hp: 1, max_hp: 1 })
    }
}

impl HealthComponent {
    pub fn is_alive(&self) -> bool {
        matches!(self.0, Health::Alive { .. })
    }

    pub fn new_from_difficulty(difficulty: u32) -> Self {
        Self(Health::Alive {
            hp: 10 * difficulty,
            max_hp: 10 * difficulty,
        })
    }
}
