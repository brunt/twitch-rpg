use crate::ecs::assets::ASSETS;
use common::ShopItem;
use common::{Item, ItemQuality};

#[derive(Default)]
pub struct ShopItemPool {
    pub all_items: Vec<ShopItem>,
}

impl ShopItemPool {
    pub fn new() -> Self {
        use ItemQuality::*;
        let allowed = [Common];

        let all_items = load_items_by_quality(|item| {
            if allowed.contains(&item.quality) {
                Some(item.to_shop_item())
            } else {
                None
            }
        });
        Self { all_items }
    }
}

fn load_items_by_quality<F, T>(filter: F) -> Vec<T>
where
    F: Fn(&Item) -> Option<T>,
{
    let assets = ASSETS
        .load_rec_dir::<Item>("assets.items")
        .expect("Failed to load assets");

    assets
        .read()
        .ids()
        .filter_map(|id| {
            let item = ASSETS.load::<Item>(id).ok()?.read();
            filter(&item)
        })
        .collect()
}

fn allowed_qualities_for_difficulty(difficulty: u32) -> Vec<ItemQuality> {
    use ItemQuality::*;
    match difficulty {
        0 => vec![Common],
        1 => vec![Common, Uncommon],
        2 => vec![Common, Uncommon, Rare],
        3 => vec![Uncommon, Rare],
        4 => vec![Uncommon, Rare, Epic],
        5 => vec![Rare, Epic],
        _ => vec![Epic, Legendary],
    }
}

pub fn initialize_reward_items(difficulty: u32) -> Vec<Item> {
    let allowed = allowed_qualities_for_difficulty(difficulty);

    load_items_by_quality(|item| {
        if allowed.contains(&item.quality) {
            Some(item.clone())
        } else {
            None
        }
    })
}

// TODO: allowed qualities as an arg
pub fn initialize_shop_items() -> Vec<ShopItem> {
    use ItemQuality::*;
    let allowed = [Common, Uncommon];

    load_items_by_quality(|item| {
        if allowed.contains(&item.quality) {
            Some(item.to_shop_item())
        } else {
            None
        }
    })
}

//     id: 288,
//     name: "Academy Teacher's Hat".to_string(),

//     id: 576,
//     name: "Cinder Slippers".to_string(),
