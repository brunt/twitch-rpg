use assets_manager::AssetCache;
use common::{Item, ItemQuality};
use common::ShopItem;
use std::sync::LazyLock;

#[derive(Default)]
pub struct ShopItemPool {
    pub all_items: Vec<ShopItem>,
}

static ASSETS: LazyLock<AssetCache> = LazyLock::new(|| {
    AssetCache::new("server").unwrap()
});


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

pub fn initialize_shop_items() -> Vec<ShopItem> {
    use ItemQuality::*;
    let allowed = vec![Common, Uncommon];

    load_items_by_quality(|item| {
        if allowed.contains(&item.quality) {
            Some(item.to_shop_item())
        } else {
            None
        }
    })
}



// vec![
        // ShopItem {
        //     id: 288,
        //     name: "Academy Teacher's Hat".to_string(),
        //     quality: ItemQuality::Legendary,
        //     equip_slot: EquipmentSlot::Head,
        //     stats: Some(ItemStats {
        //         attack_modifiers: None,
        //         defense_modifiers: Some(DefenseModifiers {
        //             damage_reduction: 1,
        //             evasion_rating: 0,
        //         }),
        //         other_modifiers: None,
        //
        //         strength: None,
        //         intelligence: Some(9),
        //         agility: None,
        //     }),
        //     price: 230,
        //     description: "Spells have longer durations and affect larger areas".to_string(),
        //     consumable: false,
        //     effects: None,
        // },



        // ShopItem {
        //     id: 576,
        //     name: "Cinder Slippers".to_string(),
        //     quality: ItemQuality::Rare,
        //     equip_slot: EquipmentSlot::Feet,
        //     stats: Some(ItemStats {
        //         attack_modifiers: None,
        //         defense_modifiers: None,
        //         other_modifiers: Some(OtherModifiers {
        //             movement_speed_increase: 1,
        //         }),
        //         strength: None,
        //         intelligence: None,
        //         agility: None,
        //     }),
        //     price: 40,
        //     description: "More elemental resistance".to_string(),
        //     consumable: false,
        //     effects: None,
        // },


    // ]

