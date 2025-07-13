use common::{ItemQuality, ShopItem};


#[derive(Default)]
pub struct ShopItemPool {
    pub all_items: Vec<ShopItem>,
}

pub fn initialize_shop_items() -> Vec<ShopItem> {
    vec![
        ShopItem{
            sprite: "white_wizard_hat".to_string(),
            name: "Academy Teacher's Hat".to_string(),
            quality: ItemQuality::Legendary,
            price: 230,
            description: "Spells have longer durations and affect larger areas".to_string(),
        },
        ShopItem{
            sprite: "mace".to_string(),
            name: "Mace".to_string(),
            quality: ItemQuality::Common,
            price: 15,
            description: "Staggers opponents on hit".to_string(),
        },
        ShopItem{
            sprite: "trident".to_string(),
            name: "Trident".to_string(),
            quality: ItemQuality::Common,
            price: 18,
            description: "Melee attacks have longer reach".to_string(),
        },
        ShopItem{
            sprite: "trident".to_string(),
            name: "Trident".to_string(),
            quality: ItemQuality::Common,
            price: 18,
            description: "Melee attacks have longer reach".to_string(),
        },
        ShopItem{
            sprite: "greatsword".to_string(),
            name: "Greatsword".to_string(),
            quality: ItemQuality::Uncommon,
            price: 28,
            description: "Melee attacks have longer reach".to_string(),
        },
        ShopItem{
            sprite: "longsword".to_string(),
            name: "Longsword".to_string(),
            quality: ItemQuality::Rare,
            price: 34,
            description: "Melee attacks have longer reach".to_string(),
        },
        ShopItem{
            sprite: "scimitar".to_string(),
            name: "Scimitar".to_string(),
            quality: ItemQuality::Uncommon,
            price: 20,
            description: "Melee attacks cut deeper".to_string(),
        },
        ShopItem{
            sprite: "purple_tip_wood_staff".to_string(),
            name: "Netherwood Staff".to_string(),
            quality: ItemQuality::Uncommon,
            price: 20,
            description: "Curses last longer".to_string(),
        },
        ShopItem{
            sprite: "purple_tip_silver_staff".to_string(),
            name: "Nether-orb Staff".to_string(),
            quality: ItemQuality::Rare,
            price: 40,
            description: "More elemental damage".to_string(),
        },
        ShopItem{
            sprite: "red_cloth_boots".to_string(),
            name: "Cinder Slippers".to_string(),
            quality: ItemQuality::Rare,
            price: 40,
            description: "More elemental resistance".to_string(),
        },
        ShopItem{
            sprite: "green_leather_boots".to_string(),
            name: "Elven Boots".to_string(),
            quality: ItemQuality::Uncommon,
            price: 30,
            description: "More movement speed".to_string(),
        },
    ]
}