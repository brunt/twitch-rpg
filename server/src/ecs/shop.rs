// use crate::commands::{Item, MenuItem};
// use crate::ecs::components::{Inventory, InventoryItem, PlayerInfo};
// use specs::{Entity, World, WorldExt};
//
// /// Attempts to purchase an item for a player
// /// Returns true if purchase was successful
// pub fn purchase_item(
//     world: &mut World,
//     player_entity: Entity,
//     item_id: MenuItem,
// ) -> bool {
//     // Get shop inventory
//     let item_and_price = {
//         if let Some(shop) = world.get::<ShopInventory>() {
//             shop.items.get(&item_id).cloned()
//         } else {
//             None
//         }
//     };
//
//     // Check if item exists
//     if let Some((item, price)) = item_and_price {
//         // Check if player has enough gold
//         let has_gold = {
//             let mut player_info_storage = world.write_storage::<PlayerInfo>();
//             if let Some(player_info) = player_info_storage.get_mut(player_entity) {
//                 if player_info.gold >= price {
//                     player_info.gold -= price;
//                     true
//                 } else {
//                     false
//                 }
//             } else {
//                 false
//             }
//         };
//
//         // Add to inventory if player had enough gold
//         if has_gold {
//             let mut inventory_storage = world.write_storage::<Inventory>();
//             if let Some(inventory) = inventory_storage.get_mut(player_entity) {
//                 if inventory.items.len() < inventory.capacity {
//                     inventory.items.push(InventoryItem {
//                         item: item.clone(),
//                         quantity: 1,
//                     });
//                     return true;
//                 } else {
//                     // Refund gold if inventory is full
//                     let mut player_info_storage = world.write_storage::<PlayerInfo>();
//                     if let Some(player_info) = player_info_storage.get_mut(player_entity) {
//                         player_info.gold += price;
//                     }
//                 }
//             }
//         }
//     }
//
//     false
// }
//
// /// Find a player entity by name
// pub fn find_player_by_name(world: &World, player_name: &str) -> Option<Entity> {
//     if let Some(town) = world.get::<TownPlayers>() {
//         town.players.get(player_name).copied()
//     } else {
//         None
//     }
// }
