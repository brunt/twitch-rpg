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
        let Some(dungeon_loot) = dungeon_loot.as_mut() else { return };
        // Create a map of unopened item positions
        let mut unopened_items: Vec<(Entity, &Position)> = (&entities, &items, &positions)
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
                opened
                    .insert(*item_entity, Opened)
                    .expect("Failed to mark item opened");

                // TODO: lottery function on 3rd state.
                // The actual item type should be determined after the winner is chosen so that the item is guaranteed to be useful for that player.
                dungeon_loot.items += 1;

                targets.remove(player_entity);
            }
        }
    }
}
