use specs::{Entities, Entity, Join, ReadStorage, System, WriteExpect, WriteStorage};
use common::{EquipmentSlot, ItemQuality, ItemStats, MenuItem, ShopItem};
use crate::ecs::components::{DungeonItem, Opened, Player, Position};
use crate::ecs::components::movement::TargetPosition;
use crate::ecs::resources::DungeonLoot;

pub struct ItemPickup;

impl<'a> System<'a> for ItemPickup {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, DungeonItem>,
        WriteStorage<'a, Opened>,
        WriteStorage<'a, TargetPosition>,
        WriteExpect<'a, DungeonLoot>,
    );

    fn run(&mut self, (entities, positions, players, items, mut opened, mut targets, mut dungeon_loot): Self::SystemData) {
        // Create a map of unopened item positions
        let mut unopened_items: Vec<(Entity, &Position)> =
            (&entities, &items, &positions)
                .join()
                .filter(|(e, _, _)| !opened.contains(*e))
                .map(|(e, _, p)| (e, p))
                .collect();

        for (player_entity, player_pos, _) in (&entities, &positions, &players).join() {
            if let Some((item_entity, _)) = unopened_items
                .iter()
                .find(|(_, item_pos)| *item_pos == player_pos)
            {
                // Mark item as opened
                opened.insert(*item_entity, Opened).expect("Failed to mark item opened");
                
                let item_idx = dungeon_loot.items.clone().len() + 1;
                // TODO: lottery function
                dungeon_loot.items.insert(MenuItem::from(item_idx), ShopItem {
                    sprite: "longsword".parse().unwrap(),
                    name: "Longsword".parse().unwrap(),
                    quality: ItemQuality::Rare,
                    equip_slot: EquipmentSlot::MainHand,
                    price: 0,
                    stats: ItemStats {
                        strength: Some(3),
                        dexterity: None,
                        intelligence: None,
                    },
                    description: "Melee attacks have longer reach".parse().unwrap(),
                });
                // Clear player's target if it was the item
                targets.remove(player_entity);
            }
        }
    }
}
