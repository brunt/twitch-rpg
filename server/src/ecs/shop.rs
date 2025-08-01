use common::ItemQuality::Common;
use common::{
    AttackModifiers, DefenseModifiers, Effect, EquipmentSlot, ItemQuality, ItemStats,
    OtherModifiers, ShopItem,
};

#[derive(Default)]
pub struct ShopItemPool {
    pub all_items: Vec<ShopItem>,
}

pub fn initialize_shop_items() -> Vec<ShopItem> {
    vec![
        ShopItem {
            id: 288,
            name: "Academy Teacher's Hat".to_string(),
            quality: ItemQuality::Legendary,
            equip_slot: EquipmentSlot::Head,
            stats: Some(ItemStats {
                attack_modifiers: None,
                defense_modifiers: Some(DefenseModifiers {
                    damage_reduction: 1,
                    evasion_rating: 0,
                }),
                other_modifiers: None,

                strength: None,
                intelligence: Some(9),
                agility: None,
            }),
            price: 230,
            description: "Spells have longer durations and affect larger areas".to_string(),
            consumable: false,
            effects: None,
        },
        ShopItem {
            id: 2,
            name: "Mace".to_string(),
            quality: Common,
            equip_slot: EquipmentSlot::MainHand,
            stats: Some(ItemStats {
                attack_modifiers: Some(AttackModifiers {
                    damage_bonus: 1,
                    hit_rating_bonus: 3,
                    range_bonus: 0,
                    cooldown_reduction_ms: 0,
                }),
                defense_modifiers: None,
                other_modifiers: None,
                strength: None,
                intelligence: None,
                agility: None,
            }),
            price: 15,
            description: "Staggers opponents on hit".to_string(),
            consumable: false,
            effects: None,
        },
        ShopItem {
            id: 4,
            name: "Trident".to_string(),
            quality: ItemQuality::Uncommon,
            equip_slot: EquipmentSlot::MainHand,

            price: 18,
            stats: Some(ItemStats {
                attack_modifiers: None,
                defense_modifiers: None,
                other_modifiers: None,
                strength: None,
                intelligence: None,
                agility: None,
            }),
            description: "Melee attacks have longer reach".to_string(),
            consumable: false,
            effects: None,
        },
        ShopItem {
            id: 4,
            name: "Trident".to_string(),
            quality: ItemQuality::Common,
            equip_slot: EquipmentSlot::MainHand,
            stats: Some(ItemStats {
                attack_modifiers: None,
                defense_modifiers: None,
                other_modifiers: None,
                strength: None,
                intelligence: None,
                agility: None,
            }),
            price: 18,
            description: "Melee attacks have longer reach".to_string(),
            consumable: false,
            effects: None,
        },
        ShopItem {
            id: 5,
            name: "Greatsword".to_string(),
            quality: ItemQuality::Uncommon,
            equip_slot: EquipmentSlot::MainHand,
            stats: Some(ItemStats {
                attack_modifiers: Some(AttackModifiers {
                    damage_bonus: 3,
                    hit_rating_bonus: 2,
                    range_bonus: 2,
                    cooldown_reduction_ms: 0,
                }),
                defense_modifiers: None,
                other_modifiers: None,
                strength: None,
                intelligence: None,
                agility: None,
            }),
            price: 28,
            description: "Melee attacks have longer reach".to_string(),
            consumable: false,
            effects: None,
        },
        ShopItem {
            id: 6,
            name: "Longsword".to_string(),
            quality: ItemQuality::Rare,
            equip_slot: EquipmentSlot::Head,
            stats: Some(ItemStats {
                attack_modifiers: None,
                defense_modifiers: None,
                other_modifiers: None,
                strength: None,
                intelligence: None,
                agility: None,
            }),
            price: 34,
            description: "Melee attacks have longer reach".to_string(),
            consumable: false,
            effects: None,
        },
        ShopItem {
            id: 7,
            name: "Scimitar".to_string(),
            quality: ItemQuality::Uncommon,
            equip_slot: EquipmentSlot::MainHand,
            stats: Some(ItemStats {
                attack_modifiers: None,
                defense_modifiers: None,
                other_modifiers: None,
                strength: None,
                intelligence: None,
                agility: None,
            }),
            price: 20,
            description: "Melee attacks cut deeper".to_string(),
            consumable: false,
            effects: None,
        },
        ShopItem {
            id: 8,
            name: "Netherwood Staff".to_string(),
            quality: ItemQuality::Uncommon,
            equip_slot: EquipmentSlot::MainHand,
            stats: Some(ItemStats {
                attack_modifiers: Some(AttackModifiers {
                    damage_bonus: 3,
                    hit_rating_bonus: 2,
                    range_bonus: 20,
                    cooldown_reduction_ms: 0,
                }),
                defense_modifiers: None,
                other_modifiers: None,
                strength: None,
                intelligence: None,
                agility: None,
            }),
            price: 20,
            description: "Curses last longer".to_string(),
            consumable: false,
            effects: None,
        },
        ShopItem {
            id: 9,
            name: "Nether-orb Staff".to_string(),
            quality: ItemQuality::Rare,
            equip_slot: EquipmentSlot::MainHand,
            stats: Some(ItemStats {
                attack_modifiers: Some(AttackModifiers {
                    damage_bonus: 3,
                    hit_rating_bonus: 2,
                    range_bonus: 20,
                    cooldown_reduction_ms: 0,
                }),
                defense_modifiers: None,
                other_modifiers: None,
                strength: None,
                intelligence: None,
                agility: None,
            }),
            price: 40,
            description: "More elemental damage".to_string(),
            consumable: false,
            effects: None,
        },
        ShopItem {
            id: 576,
            name: "Cinder Slippers".to_string(),
            quality: ItemQuality::Rare,
            equip_slot: EquipmentSlot::Feet,
            stats: Some(ItemStats {
                attack_modifiers: None,
                defense_modifiers: None,
                other_modifiers: Some(OtherModifiers {
                    movement_speed_increase: 1,
                }),
                strength: None,
                intelligence: None,
                agility: None,
            }),
            price: 40,
            description: "More elemental resistance".to_string(),
            consumable: false,
            effects: None,
        },
        ShopItem {
            id: 1,
            name: "Elven Boots".to_string(),
            quality: ItemQuality::Uncommon,
            equip_slot: EquipmentSlot::Head,
            stats: Some(ItemStats {
                attack_modifiers: None,
                defense_modifiers: None,
                other_modifiers: Some(OtherModifiers {
                    movement_speed_increase: 1,
                }),
                strength: None,
                intelligence: None,
                agility: None,
            }),
            price: 30,
            description: "More movement speed".to_string(),
            consumable: false,
            effects: None,
        },
        ShopItem {
            id: 26,
            name: "Healing Potion".to_string(),
            quality: Common,
            equip_slot: EquipmentSlot::UtilitySlot,
            stats: None,
            price: 3,
            description: "Restores 4hp".to_string(),
            consumable: true,
            effects: Some(vec![Effect::Heal(4)]),
        },
    ]
}
