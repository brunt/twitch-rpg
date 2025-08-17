use crate::ecs::components::movement::TargetPosition;
use crate::ecs::components::{DungeonItem, Opened, Player, Position};
use crate::ecs::resources::DungeonLoot;
use common::{EquipmentSlot, ItemQuality, ItemStats, MenuItem, ShopItem};
use specs::{Entities, Entity, Join, ReadStorage, System, WriteExpect, WriteStorage};

pub struct ItemPickup;

impl<'a> System<'a> for ItemPickup {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, DungeonItem>,
        WriteStorage<'a, Opened>,
        WriteStorage<'a, TargetPosition>,
        WriteExpect<'a, Option<DungeonLoot>>,
    );

    fn run(
        &mut self,
        (entities, positions, players, items, mut opened, mut targets, mut dungeon_loot): Self::SystemData,
    ) {
        let Some(dungeon_loot) = dungeon_loot.as_mut() else {
            return;
        };

        // Collect unopened items
        let unopened_items: Vec<(Entity, &Position)> = (&entities, &items, &positions)
            .join()
            .filter(|(e, _, _)| !opened.contains(*e))
            .map(|(e, _, p)| (e, p))
            .collect();

        // Collect items that *any* player is opening
        let mut items_to_open = Vec::new();
        for (player_entity, player_pos, _) in (&entities, &positions, &players).join() {
            if let Some((item_entity, _)) = unopened_items
                .iter()
                .find(|(_, item_pos)| *item_pos == player_pos)
            {
                items_to_open.push((*item_entity, player_entity));
            }
        }

        // Deduplicate by item entity
        items_to_open.sort_by_key(|(item_entity, _)| *item_entity);
        items_to_open.dedup_by_key(|(item_entity, _)| *item_entity);

        // Now apply changes
        for (item_entity, player_entity) in items_to_open {
            opened
                .insert(item_entity, Opened)
                .expect("Failed to mark item opened");
            dungeon_loot.items += 1;
            targets.remove(player_entity);
        }
    }
}
