use crate::ecs::resources::{GameState, ShopInventory};
use crate::ecs::shop::ShopItemPool;
use common::{MenuItem, ShopItem};
use rand::rngs::ThreadRng;
use rand::seq::IteratorRandom;
use specs::{ReadExpect, System, WriteExpect};

/// ShopPopulation is a system to fill the shop with items
pub struct ShopPopulation;

impl<'a> System<'a> for ShopPopulation {
    type SystemData = (
        ReadExpect<'a, GameState>,
        ReadExpect<'a, ShopItemPool>,
        WriteExpect<'a, ShopInventory>,
    );

    fn run(&mut self, (game_state, shop_item_pool, mut shop_inventory): Self::SystemData) {
        if matches!(*game_state, GameState::InTown) && shop_inventory.items.is_empty() {
            let mut rng = ThreadRng::default();
            shop_inventory.items = generate_shop_items(&shop_item_pool, &mut rng);
        }
    }
}

fn generate_shop_items(pool: &ShopItemPool, rng: &mut ThreadRng) -> Vec<(MenuItem, ShopItem)> {
    pool.all_items
        .iter()
        .choose_multiple(rng, 8)
        .into_iter()
        .enumerate()
        .map(|(i, shop_item)| (MenuItem::from(i), shop_item.clone()))
        .collect()
}
