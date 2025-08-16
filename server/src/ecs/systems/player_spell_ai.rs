use crate::ecs::components::combat::AttackComponent;
use crate::ecs::components::combat::HealthComponent;
use crate::ecs::components::combat::MeleeAttacker;
use crate::ecs::components::effect::ActiveEffects;
use crate::ecs::components::spells::{SpellCaster, SpellTarget, SpellTimer, Spellbook};
use crate::ecs::components::{Enemy, Player, Position};
use crate::ecs::resources::Adventure;
use crate::ecs::resources::RoomCheck;
use common::Spell;
use common::{Health, SpellCasterRestriction, TargetFilter, TargetShape, Targeting};
use rand::seq::IteratorRandom;
use specs::prelude::*;

pub struct PlayerSpellAISystem;

impl<'a> System<'a> for PlayerSpellAISystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Enemy>,
        ReadStorage<'a, HealthComponent>,
        ReadStorage<'a, SpellCaster>,
        ReadStorage<'a, Spellbook>,
        ReadStorage<'a, SpellTimer>,
        ReadStorage<'a, ActiveEffects>,
        ReadStorage<'a, MeleeAttacker>,
        ReadStorage<'a, AttackComponent>,
        WriteStorage<'a, SpellTarget>,
        ReadExpect<'a, Option<Adventure>>,
    );

    fn run(
        &mut self,
        (
            entities,
            positions,
            players,
            enemies,
            healths,
            spell_casters,
            spellbooks,
            spell_timers,
            active_effects,
            melee_attackers,
            attack_components,
            mut spell_targets,
            adv,
        ): Self::SystemData,
    ) {
        let Some(adv) = adv.as_ref() else {
            return;
        };

        // Iterate players
        for (player_entity, pos, _, health) in (&entities, &positions, &players, &healths)
            .join()
            .filter(|(_, _, _, health)| !matches!(health.0, Health::Dead))
        {
            // Skip if not in current room
            let Some(current_room) = adv
                .get_current_floor()
                .rooms
                .iter()
                .find(|r| r.id == adv.current_room_index)
            else {
                return;
            };

            // Check if player can cast
            let (Some(_), Some(spellbook)) = (
                spell_casters.get(player_entity),
                spellbooks.get(player_entity),
            ) else {
                continue;
            };

            // Check cooldown
            let not_on_cooldown = spell_timers
                .get(player_entity)
                .is_none_or(|timer| timer.remaining <= 0.0);

            if !not_on_cooldown {
                continue;
            }

            // Gather enemies in room
            let enemies_in_room: Vec<(Entity, &Position, &Health)> =
                (&entities, &enemies, &positions, &healths)
                    .join()
                    .filter_map(|(e, _, pos, health)| match health.0 {
                        Health::Alive { .. } => Some((e, pos, &health.0)),
                        _ => None,
                    })
                    .filter(|(_, pos, _)| {
                        current_room.contains(&tatami_dungeon::Position::from(*pos))
                    })
                    .collect();

            let chosen = find_buff_spell(
                spellbook,
                player_entity,
                &entities,
                &positions,
                &players,
                &healths,
                &melee_attackers,
                current_room,
            )
            .or_else(|| find_offensive_spell(spellbook, &enemies_in_room))
            .or_else(|| {
                find_healing_spell(
                    spellbook,
                    player_entity,
                    &entities,
                    &positions,
                    &players,
                    &healths,
                    current_room,
                )
            });

            // Cast if something chosen
            if let Some((spell, target_entity)) = chosen {
                spell_targets
                    .insert(
                        player_entity,
                        SpellTarget {
                            caster: player_entity,
                            target: target_entity,
                            spell_id: spell.id,
                        },
                    )
                    .expect("failed to set spell target");
            }
        }
    }
}

fn find_buff_spell<'a>(
    spellbook: &'a Spellbook,
    player_entity: Entity,
    entities: &Entities,
    positions: &ReadStorage<Position>,
    players: &ReadStorage<Player>,
    healths: &ReadStorage<HealthComponent>,
    melee_attackers: &ReadStorage<MeleeAttacker>,
    current_room: &tatami_dungeon::Room,
) -> Option<(&'a Spell, Entity)> {
    // Pick a buff spell
    let spell = spellbook.spells.iter().find(|s| {
        matches!(
            s.targeting.filter,
            TargetFilter::SelfOnly | TargetFilter::AllyOrSelf
        )
    })?;

    // Decide the target entity
    let target_entity = match spell.targeting.filter {
        TargetFilter::SelfOnly => player_entity,
        TargetFilter::AllyOrSelf => {
            // Find allies in the same room
            let mut allies_in_room: Vec<Entity> = (&*entities, players, positions, healths)
                .join()
                .filter(|(entity, _, pos, health)| {
                    *entity != player_entity
                        && !matches!(health.0, Health::Dead)
                        && current_room.contains(&tatami_dungeon::Position::from(*pos))
                })
                .map(|(entity, _, _, _)| entity)
                .collect();

            // Prefer melee attackers
            if let Some(melee_ally) = allies_in_room
                .iter()
                .find(|ally| melee_attackers.contains(**ally))
            {
                *melee_ally
            } else {
                // Fallback to any ally, otherwise self
                allies_in_room.pop().unwrap_or(player_entity)
            }
        }
        _ => player_entity, // Shouldn't happen for buffs
    };

    Some((spell, target_entity))
}

fn find_healing_spell<'a>(
    spellbook: &'a Spellbook,
    player_entity: Entity,
    entities: &Entities,
    positions: &ReadStorage<Position>,
    players: &ReadStorage<Player>,
    healths: &ReadStorage<HealthComponent>,
    current_room: &tatami_dungeon::Room,
) -> Option<(&'a Spell, Entity)> {
    // Pick a healing spell
    let spell = spellbook.spells.iter().find(|s| {
        matches!(
            s.targeting.filter,
            TargetFilter::AllyOrSelf | TargetFilter::Any
        ) && s
            .effects
            .iter()
            .any(|(effect, _)| matches!(effect, common::Effect::Heal(_)))
    })?;

    // Collect all living allies in the room
    let allies: Vec<(Entity, &HealthComponent)> = (entities, players, positions, healths)
        .join()
        .filter(|(entity, _, pos, health)| {
            *entity != player_entity
                && !matches!(health.0, Health::Dead)
                && current_room.contains(&tatami_dungeon::Position::from(*pos))
        })
        .map(|(entity, _, _, health)| (entity, health))
        .collect();

    // Helper: % HP
    let hp_pct = |health: &HealthComponent| -> u32 {
        if let Health::Alive { hp, max_hp } = health.0 {
            (hp * 100 / max_hp) as u32
        } else {
            100
        }
    };

    // Step 1: Most injured ally
    if let Some((target, _)) = allies
        .iter()
        .filter(|(_, health)| hp_pct(health) < 100)
        .min_by_key(|(_, health)| hp_pct(health))
    {
        return Some((spell, *target));
    }

    // Step 2: Fallback to self if injured
    if let Some(health) = healths.get(player_entity) {
        if hp_pct(health) < 100 {
            return Some((spell, player_entity));
        }
    }

    None
}

fn find_offensive_spell<'a>(
    spellbook: &'a Spellbook,
    enemies_in_room: &[(Entity, &Position, &Health)],
) -> Option<(&'a Spell, Entity)> {
    if enemies_in_room.is_empty() {
        return None;
    }

    // Pick the first offensive spell that can be cast on enemies
    let spell = spellbook
        .spells
        .iter()
        .find(|s| matches!(s.targeting.filter, TargetFilter::EnemyOnly))?;

    let mut rng = rand::rngs::ThreadRng::default();
    let target_maybe = enemies_in_room
        .iter()
        .filter(|(_, _, health)| !matches!(health, Health::Dead))
        .choose(&mut rng);

    if let Some((target, _, _)) = target_maybe {
        Some((spell, *target))
    } else {
        None
    }
}
