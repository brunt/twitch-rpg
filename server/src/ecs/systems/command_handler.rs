// In server/src/ecs/systems/command_handler.rs
use crate::commands::PlayerCommand::Use;
use crate::commands::{PlayerCommand, RpgCommand};
use crate::ecs::components::class::{CharacterClass, ShowCharacter};
use crate::ecs::components::combat::HealthComponent;
use crate::ecs::components::inventory::Equipment;
use crate::ecs::components::{
    Experience, Level, Money, MovementAI, MovementAIKind, Name, Player, Position, Resource, Stats,
};
use crate::ecs::resources::{GameState, ShopInventory};
use common::{Effect, EquipmentSlot, Health};
use specs::{
    Entities, Entity, Join, ReadExpect, ReadStorage, System, Write, WriteExpect, WriteStorage,
};
use std::collections::VecDeque;
use tatami_dungeon::Dungeon;

pub type CommandQueue = VecDeque<(String, RpgCommand, bool)>;

pub struct CommandHandlerSystem;

impl<'a> System<'a> for CommandHandlerSystem {
    type SystemData = (
        WriteExpect<'a, CommandQueue>,
        WriteExpect<'a, GameState>,
        WriteStorage<'a, Name>,
        WriteStorage<'a, Level>,
        WriteStorage<'a, HealthComponent>,
        WriteStorage<'a, CharacterClass>,
        WriteStorage<'a, Money>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Equipment>,
        WriteStorage<'a, Stats>,
        WriteStorage<'a, ShowCharacter>,
        ReadExpect<'a, ShopInventory>,
        Entities<'a>,
    );

    fn run(
        &mut self,
        (
            mut command_queue,
            mut game_state,
            mut names,
            mut levels,
            mut healths,
            mut classes,
            mut money,
            mut players,
            mut equipment,
            mut stats,
            mut show_characters,
            ref shop_inventory,
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
                            names
                                .insert(player_entity, Name(player_name))
                                .expect("failed to set name");
                            healths
                                .insert(player_entity, HealthComponent::new_from_class(&class))
                                .expect("failed to set default health");
                            stats
                                .insert(player_entity, Stats::new(&class))
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
                            // get the entity of the player, get that entity's money, subtract gold from their money,
                            // get the entity's inventory, add item at MenuItem(#) to their inventory
                            if let Some((e, _name)) = (entities, &names)
                                .join()
                                .find(|(_, name)| name.0 == player_name)
                            {
                                // if an item is purchased, it is equipped in an item slot, old item is overwritten
                                money.get_mut(e).map(|gold| {
                                    let price =
                                        shop_inventory.items.get(&item).map_or(5, |f| f.price);
                                    if gold.0 < price {
                                        return;
                                    }
                                    gold.0 -= price;
                                });
                                equipment.get_mut(e).map(|equip_slots| {
                                    let item = shop_inventory.items.get(&item).unwrap();
                                    equip_slots
                                        .slots
                                        .insert(item.equip_slot.clone(), item.to_equipped_item())
                                });
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
                    }
                }
                GameState::OnAdventure => match command {
                    RpgCommand::PlayerCommand(Use(_item)) => {
                        if let Some((e, _)) = (entities, &names)
                            .join()
                            .find(|(_, name)| name.0 == player_name)
                        {
                            let Some(equipment) = equipment.get_mut(e) else {
                                return;
                            };
                            let Some(item) = equipment.slots.get(&EquipmentSlot::UtilitySlot)
                            else {
                                return;
                            };
                            let Some(effects) = &item.effects else { return };
                            for effect in effects {
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
