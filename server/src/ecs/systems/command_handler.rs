use crate::commands::PlayerCommand::Use;
use crate::commands::{PlayerCommand, RpgCommand};
use crate::ecs::components::class::{CharacterClass, ShowCharacter};
use crate::ecs::components::combat::HealthComponent;
use crate::ecs::components::effect::{ActiveEffects, TimedEffect};
use crate::ecs::components::form::FormComponent;
use crate::ecs::components::inventory::Equipment;
use crate::ecs::components::spells::{SpellCaster, Spellbook};
use crate::ecs::components::{Experience, Level, Money, Name, Player, Stats};
use crate::ecs::resources::{Difficulty, GameState, ShopInventory};
use crate::ecs::spells::AllSpells;
use common::{Effect, EquipmentSlot, Form, Health};
use specs::{Entities, Entity, Join, ReadExpect, System, Write, WriteExpect, WriteStorage};
use std::collections::VecDeque;
use std::ops::Add;
use tatami_dungeon::Dungeon;

pub type CommandQueue = VecDeque<(String, RpgCommand, bool)>;

pub struct CommandHandlerSystem;

impl<'a> System<'a> for CommandHandlerSystem {
    type SystemData = (
        WriteExpect<'a, CommandQueue>,
        ReadExpect<'a, GameState>,
        WriteExpect<'a, Difficulty>,
        WriteStorage<'a, Name>,
        WriteStorage<'a, Level>,
        WriteStorage<'a, Experience>,
        WriteStorage<'a, HealthComponent>,
        WriteStorage<'a, CharacterClass>,
        WriteStorage<'a, Money>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Equipment>,
        WriteStorage<'a, Stats>,
        WriteStorage<'a, ShowCharacter>,
        WriteStorage<'a, FormComponent>,
        WriteStorage<'a, ActiveEffects>,
        WriteStorage<'a, Spellbook>,
        WriteStorage<'a, SpellCaster>,
        ReadExpect<'a, ShopInventory>,
        ReadExpect<'a, AllSpells>,
        Entities<'a>,
    );

    fn run(
        &mut self,
        (
            mut command_queue,
            game_state,
            mut difficulty,
            mut names,
            mut levels,
            mut experiences,
            mut healths,
            mut classes,
            mut money,
            mut players,
            mut equipment,
            mut stats,
            mut show_characters,
            mut forms,
            mut active_effects,
            mut spellbooks,
            mut spell_casters,
            ref shop_inventory,
            ref all_spells,
            ref entities,
        ): Self::SystemData,
    ) {
        while let Some((player_name, command, is_privileged)) = command_queue.pop_front() {
            match *game_state {
                GameState::InTown => {
                    match command {
                        RpgCommand::Join(class) => {
                            // a player has queued up to join a dungeon.
                            // they begin in "town" and are able to immediately purchase items
                            // when the dungeon starts, they are added as entities (with purchased items) to dungeon
                            // before that, they are stored in the Town hashmap

                            // do not let player duplicates join
                            if (&names).join().any(|name| name.0 == player_name) {
                                continue;
                            }

                            let player_entity = entities.create();
                            levels
                                .insert(player_entity, Level::default())
                                .expect("failed to set default level");
                            experiences
                                .insert(player_entity, Experience::default())
                                .expect("failed to set default experience");
                            names
                                .insert(player_entity, Name(player_name))
                                .expect("failed to set name");
                            healths
                                .insert(player_entity, HealthComponent::default())
                                .expect("failed to set default health");
                            stats
                                .insert(player_entity, Stats::new(&class, 1)) //TODO: load from save with this command
                                .expect("failed to add stats");
                            classes
                                .insert(player_entity, CharacterClass(class))
                                .expect("failed to set class");

                            money
                                .insert(
                                    player_entity,
                                    if is_privileged {
                                        Money::new(100)
                                    } else {
                                        Money::default()
                                    },
                                )
                                .expect("failed to set default money");
                            players
                                .insert(player_entity, Player)
                                .expect("failed to set player");
                            equipment
                                .insert(player_entity, Equipment::default())
                                .expect("failed to create inventory");
                            spellbooks
                                .insert(
                                    player_entity,
                                    Spellbook::from_class_and_level(Some(class), 1, all_spells),
                                )
                                .expect("failed to create spellbook");
                            spell_casters
                                .insert(player_entity, SpellCaster)
                                .expect("failed to create spell caster");
                        }
                        RpgCommand::Rejoin => {
                            // Load player character
                            // do not let player duplicates join
                            if (&names).join().any(|name| name.0 == player_name) {
                                continue;
                            }
                            //TODO: load data from database
                        }
                        RpgCommand::PlayerCommand(PlayerCommand::Buy(item)) => {
                            if let Some((e, _name)) = (entities, &names)
                                .join()
                                .find(|(_, name)| name.0 == player_name)
                            {
                                if let Some(shop_item) =
                                    shop_inventory.items.iter().find(|(id, _)| *id == item)
                                    && let Some(gold) = money.get_mut(e)
                                    && gold.0 >= shop_item.1.price
                                {
                                    gold.0 -= shop_item.1.price;

                                    if let Some(equip_slots) = equipment.get_mut(e) {
                                        equip_slots.slots.insert(
                                            shop_item.1.equip_slot.clone(),
                                            shop_item.1.to_item(),
                                        );
                                    }
                                }
                            }
                        }
                        RpgCommand::PlayerCommand(PlayerCommand::Show) => {
                            if let Some((e, _name)) = (entities, &names)
                                .join()
                                .find(|(_, name)| name.0 == player_name)
                            {
                                show_characters
                                    .insert(e, ShowCharacter)
                                    .expect("failed to show character");
                            }
                        }
                        RpgCommand::Difficulty(diff) => {
                            difficulty.value = diff.min(4);
                        }
                        _ => {}
                    }
                }
                GameState::OnAdventure => match command {
                    RpgCommand::PlayerCommand(Use(_item)) => {
                        if let Some((e, _, _)) = (entities, &names, &healths)
                            .join()
                            .filter(|(_, _, health)| !matches!(health.0, Health::Dead))
                            .find(|(_, name, _)| name.0 == player_name)
                        {
                            let Some(equipment) = equipment.get_mut(e) else {
                                return;
                            };
                            let Some(item) = equipment.slots.get(&EquipmentSlot::UtilitySlot)
                            else {
                                return;
                            };
                            let Some(effects) = &item.effects else { return };
                            for (effect, duration) in effects {
                                match effect {
                                    Effect::Heal(amount) => {
                                        let Some(current_health) = healths.get_mut(e) else {
                                            return;
                                        };
                                        if let Health::Alive {
                                            hp: current_hp,
                                            max_hp,
                                        } = current_health.0
                                        {
                                            let new_hp = (current_hp + amount).min(max_hp);
                                            current_health.0 = Health::Alive { hp: new_hp, max_hp };
                                        };
                                    }
                                    Effect::Transform(transform) => {
                                        match transform {
                                            // TODO: duration
                                            Form::Scaled(scale) => {
                                                let Some(form) = forms.get_mut(e) else { return };
                                                form.0 = Form::Scaled(*scale);
                                            }
                                            Form::Invisible => {
                                                let Some(form) = forms.get_mut(e) else { return };
                                                form.0 = Form::Invisible;
                                            }
                                            _ => {
                                                let Some(form) = forms.get_mut(e) else { return };
                                                form.0 = Form::Normal;
                                            }
                                        }
                                        if let Some(duration) = duration {
                                            active_effects
                                                .entry(e)
                                                .expect("active effects entry")
                                                .or_insert_with(ActiveEffects::default)
                                                .effects
                                                .push(TimedEffect {
                                                    effect: effect.clone(),
                                                    remaining_secs: *duration,
                                                });
                                        }
                                    }
                                    Effect::StatChange(_) => {
                                        // StatAggregation system will handle stat calculations
                                        // We only need to add the effect to ActiveEffects if it has a duration
                                        if let Some(duration) = duration {
                                            active_effects
                                                .entry(e)
                                                .expect("active effects entry")
                                                .or_insert_with(ActiveEffects::default)
                                                .effects
                                                .push(TimedEffect {
                                                    effect: effect.clone(),
                                                    remaining_secs: *duration,
                                                });
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            // do what the effect does

                            if item.consumable {
                                equipment.slots.remove(&EquipmentSlot::UtilitySlot);
                            }
                        }
                    }
                    RpgCommand::PlayerCommand(PlayerCommand::Show) => {
                        if let Some((e, _name)) = (entities, &names)
                            .join()
                            .find(|(_, name)| name.0 == player_name)
                        {
                            show_characters
                                .insert(e, ShowCharacter)
                                .expect("failed to show character");
                        }
                    }
                    _ => {}
                },
                GameState::AfterDungeon => {} // no commands at this phase
            }
        }
    }
}
