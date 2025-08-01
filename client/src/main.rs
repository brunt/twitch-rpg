mod components;
mod dungeon_floor;
mod item_shop;
mod sprites;

use common::{AttackModifiers, DefenseModifiers, EquipmentSlot, EquippedItem, Form, GameSnapShot, Health, ItemQuality, ItemStats, MenuItem, OtherModifiers, PlayerClass, PlayerSnapshot, PlayerStats, ShopItem};
use components::bottom_panel::BottomPanel;
use components::game_canvas::GameCanvas;
use components::side_panel::SidePanelCharacterSheet;
use leptos::mount::mount_to_body;
use leptos::prelude::{ClassAttribute, Effect, ElementChild, Get, GetUntracked, IntoInner, NodeRefAttribute, Set, signal, StyleAttribute};
use leptos::{IntoView, component, view};
use std::collections::HashMap;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{EventSource, MessageEvent};
use common::ItemQuality::{Common, Epic, Uncommon};
use common::Effect as GameEffect;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (gamestate, set_gamestate) = signal::<Option<GameSnapShot>>(None);
        // MOCK DATA FOR LOCAL DEV
    // Effect::new(move |_| {
    //     let mock_snapshot = GameSnapShot {
    //         // Fill with test data
    //         floor: None,
    //         // ready_timer: Some(SerializedCountdownTimer { remaining: 4 }),
    //         ready_timer: None,
    //         shop_items: Some(initialize_shop_items()),
    //         // shop_items: None,
    //         party: vec![
    //             PlayerSnapshot {
    //                 name: "xMellowMonkeyx".to_string(),
    //                 class: PlayerClass::Paladin,
    //                 health: Health::Alive { hp: 11, max_hp: 10 },
    //                 level: 2,
    //                 gold: 100,
    //                 form: Form::Normal,
    //                 stats: PlayerStats {
    //                     strength: 3,
    //                     intelligence: 2,
    //                     agility: 3,
    //                 },
    //                 show: false,
    //                 equipped_items: HashMap::from([(EquipmentSlot::UtilitySlot,  EquippedItem {
    //                     item_id: 26,
    //                     name: "Healing Potion".to_string(),
    //                     quality: Common,
    //                     slot: EquipmentSlot::MainHand,
    //                     stats: None,
    //                     consumable: true,
    //                     effects: Some(vec![GameEffect::Heal(4)])
    //                 })])
    //             },
    //             PlayerSnapshot {
    //                 name: "Pixelmog".to_string(),
    //                 class: PlayerClass::Ranger,
    //                 health: Health::Alive { hp: 5, max_hp: 10 },
    //                 level: 4,
    //                 gold: 100,
    //                 form: Form::Normal,
    //                 stats: PlayerStats {
    //                     strength: 3,
    //                     intelligence: 2,
    //                     agility: 3,
    //                 },
    //                 show: false,
    //                 equipped_items: Default::default(),
    //             },
    //             PlayerSnapshot {
    //                 name: "ubruntu".to_string(),
    //                 class: PlayerClass::Wizard,
    //                 health: Health::Alive { hp: 7, max_hp: 10 },
    //                 level: 2,
    //                 gold: 100,
    //                 form: Form::Normal,
    //                 stats: PlayerStats {
    //                     strength: 3,
    //                     intelligence: 2,
    //                     agility: 3,
    //                 },
    //                 show: true,
    //                 equipped_items: HashMap::from([(EquipmentSlot::Feet,  EquippedItem {
    //                     item_id: 1,
    //                     name: "boots".to_string(),
    //                     quality: Common,
    //                     slot: EquipmentSlot::Feet,
    //                     stats: None,
    //                     consumable: false,
    //                     effects: None,
    //                 }),
    //                     (EquipmentSlot::MainHand,  EquippedItem {
    //                         item_id: 2,
    //                         name: "flogger".to_string(),
    //                         quality: Uncommon,
    //                         slot: EquipmentSlot::MainHand,
    //                         stats: None,
    //                         consumable: false,
    //                         effects: None,
    //                     }),
    //                     (EquipmentSlot::Body,  EquippedItem {
    //                         item_id: 555,
    //                         name: "flogger".to_string(),
    //                         quality: Epic,
    //                         slot: EquipmentSlot::MainHand,
    //                         stats: None,
    //                         consumable: false,
    //                         effects: None,
    //                     }),
    //                 ])
    //             },
    //             PlayerSnapshot {
    //                 name: "Pittinjury".to_string(),
    //                 class: PlayerClass::Cleric,
    //                 health: Health::Alive { hp: 5, max_hp: 10 },
    //                 level: 3,
    //                 gold: 100,
    //                 form: Form::Normal,
    //                 stats: PlayerStats {
    //                     strength: 3,
    //                     intelligence: 2,
    //                     agility: 3,
    //                 },
    //                 show: false,
    //                 equipped_items: Default::default(),
    //             },
    //             PlayerSnapshot {
    //                 name: "FrankBlankPrime".to_string(),
    //                 class: PlayerClass::Fighter,
    //                 health: Health::Alive { hp: 5, max_hp: 10 },
    //                 level: 3,
    //                 gold: 100,
    //                 form: Form::Normal,
    //                 stats: PlayerStats {
    //                     strength: 3,
    //                     intelligence: 2,
    //                     agility: 3,
    //                 },
    //                 show: false,
    //                 equipped_items: Default::default(),
    //             },
    //             PlayerSnapshot {
    //                 name: "HeroBonZo".to_string(),
    //                 class: PlayerClass::Druid,
    //                 health: Health::Alive { hp: 5, max_hp: 10 },
    //                 level: 3,
    //                 gold: 100,
    //                 form: Form::Normal,
    //                 stats: PlayerStats {
    //                     strength: 3,
    //                     intelligence: 2,
    //                     agility: 3,
    //                 },
    //                 show: false,
    //                 equipped_items: Default::default(),
    //             },
    //             PlayerSnapshot {
    //                 name: "bookslap".to_string(),
    //                 class: PlayerClass::Sorcerer,
    //                 health: Health::Alive { hp: 9, max_hp: 10 },
    //                 level: 2,
    //                 gold: 100,
    //                 form: Form::Normal,
    //                 stats: PlayerStats {
    //                     strength: 3,
    //                     intelligence: 2,
    //                     agility: 3,
    //                 },
    //                 show: false,
    //                 equipped_items: Default::default(),
    //             },
    //             PlayerSnapshot {
    //                 name: "bookslap".to_string(),
    //                 class: PlayerClass::Sorcerer,
    //                 health: Health::Alive { hp: 9, max_hp: 10 },
    //                 level: 2,
    //                 gold: 100,
    //                 form: Form::Normal,
    //                 stats: PlayerStats {
    //                     strength: 3,
    //                     intelligence: 2,
    //                     agility: 3,
    //                 },
    //                 show: false,
    //                 equipped_items: Default::default(),
    //             },
    //             PlayerSnapshot {
    //                 name: "bookslap".to_string(),
    //                 class: PlayerClass::Sorcerer,
    //                 health: Health::Alive { hp: 9, max_hp: 10 },
    //                 level: 2,
    //                 gold: 100,
    //                 form: Form::Normal,
    //                 stats: PlayerStats {
    //                     strength: 3,
    //                     intelligence: 2,
    //                     agility: 3,
    //                 },
    //                 show: false,
    //                 equipped_items: Default::default(),
    //             },
    //             PlayerSnapshot {
    //                 name: "bookslap".to_string(),
    //                 class: PlayerClass::Sorcerer,
    //                 health: Health::Alive { hp: 9, max_hp: 10 },
    //                 level: 2,
    //                 gold: 100,
    //                 form: Form::Normal,
    //                 stats: PlayerStats {
    //                     strength: 3,
    //                     intelligence: 2,
    //                     agility: 3,
    //                 },
    //                 show: false,
    //                 equipped_items: Default::default(),
    //             },
    //             PlayerSnapshot {
    //                 name: "bookslap".to_string(),
    //                 class: PlayerClass::Sorcerer,
    //                 health: Health::Alive { hp: 9, max_hp: 10 },
    //                 level: 2,
    //                 gold: 100,
    //                 form: Form::Normal,
    //                 stats: PlayerStats {
    //                     strength: 3,
    //                     intelligence: 2,
    //                     agility: 3,
    //                 },
    //                 show: false,
    //                 equipped_items: Default::default(),
    //             },
    //         ],
    //         camera_position: None,
    //         floor_positions: None,
    //         difficulty: None,
    //         projectiles: None,
    //     };
    //     set_gamestate.set(Some(mock_snapshot));
    // });

    // TODO: use this for real server communication
    Effect::new(move |_| {
        let sse_event = EventSource::new("/sse").expect_throw("Failed to create EventSource");
        let callback = Closure::wrap(Box::new(move |event: MessageEvent| {
            if let Some(text) = event.data().as_string() {
                leptos::logging::log!("{}", text);
                set_gamestate
                    .set(serde_json::from_str(&text).expect_throw("Failed to parse game state"));
            }
        }) as Box<dyn FnMut(MessageEvent)>);
        sse_event.set_onmessage(Some(callback.as_ref().unchecked_ref()));
    
        callback.forget();
    });
    
    view! {
    <div class="flex flex-row">
        <div>
            <div class="w-[1280px] h-[720px]">
                <GameCanvas gs=gamestate />
            </div>
            <BottomPanel />
        </div>
        <SidePanelCharacterSheet gs=gamestate />
    </div>
}
}

//TODO: delete after local testing
// pub fn initialize_shop_items() -> HashMap<MenuItem, ShopItem> {
//     HashMap::from([
//         (MenuItem(0), ShopItem {
//             id: 288,
//             name: "Academy Teacher's Hat".to_string(),
//             quality: ItemQuality::Legendary,
//             equip_slot: EquipmentSlot::Head,
//             stats: Some(ItemStats {
//                 attack_modifiers: None,
//                 defense_modifiers: Some(DefenseModifiers {
//                     damage_reduction: 1,
//                     evasion_rating: 0,
//                 }),
//                 other_modifiers: None,
// 
//                 strength: None,
//                 intelligence: Some(9),
//                 agility: None,
//             }),
//             price: 230,
//             description: "Spells have longer durations and affect larger areas".to_string(),
//             consumable: false,
//             effects: None,
//         }),
//         (MenuItem(1), ShopItem {
//             id: 2,
//             name: "Mace".to_string(),
//             quality: Common,
//             equip_slot: EquipmentSlot::MainHand,
//             stats: Some(ItemStats {
//                 attack_modifiers: Some(AttackModifiers {
//                     damage_bonus: 1,
//                     hit_rating_bonus: 3,
//                     range_bonus: 0,
//                     cooldown_reduction_ms: 0,
//                 }),
//                 defense_modifiers: None,
//                 other_modifiers: None,
//                 strength: None,
//                 intelligence: None,
//                 agility: None,
//             }),
//             price: 15,
//             description: "Staggers opponents on hit".to_string(),
//             consumable: false,
//             effects: None,
//         }),
//         (MenuItem(2),ShopItem {
//             id: 4,
//             name: "Trident".to_string(),
//             quality: ItemQuality::Uncommon,
//             equip_slot: EquipmentSlot::MainHand,
// 
//             price: 18,
//             stats: Some(ItemStats {
//                 attack_modifiers: None,
//                 defense_modifiers: None,
//                 other_modifiers: None,
//                 strength: None,
//                 intelligence: None,
//                 agility: None,
//             }),
//             description: "Melee attacks have longer reach".to_string(),
//             consumable: false,
//             effects: None,
//         }),
//         (MenuItem(3),ShopItem {
//             id: 4,
//             name: "Trident".to_string(),
//             quality: ItemQuality::Common,
//             equip_slot: EquipmentSlot::MainHand,
//             stats: Some(ItemStats {
//                 attack_modifiers: None,
//                 defense_modifiers: None,
//                 other_modifiers: None,
//                 strength: None,
//                 intelligence: None,
//                 agility: None,
//             }),
//             price: 18,
//             description: "Melee attacks have longer reach".to_string(),
//             consumable: false,
//             effects: None,
//         }),
//         (MenuItem(4),ShopItem {
//             id: 5,
//             name: "Greatsword".to_string(),
//             quality: ItemQuality::Uncommon,
//             equip_slot: EquipmentSlot::MainHand,
//             stats: Some(ItemStats {
//                 attack_modifiers: Some(AttackModifiers {
//                     damage_bonus: 3,
//                     hit_rating_bonus: 2,
//                     range_bonus: 2,
//                     cooldown_reduction_ms: 0,
//                 }),
//                 defense_modifiers: None,
//                 other_modifiers: None,
//                 strength: None,
//                 intelligence: None,
//                 agility: None,
//             }),
//             price: 28,
//             description: "Melee attacks have longer reach".to_string(),
//             consumable: false,
//             effects: None,
//         }),
//         (MenuItem(5), ShopItem {
//             id: 6,
//             name: "Longsword".to_string(),
//             quality: ItemQuality::Rare,
//             equip_slot: EquipmentSlot::Head,
//             stats: Some(ItemStats {
//                 attack_modifiers: None,
//                 defense_modifiers: None,
//                 other_modifiers: None,
//                 strength: None,
//                 intelligence: None,
//                 agility: None,
//             }),
//             price: 34,
//             description: "Melee attacks have longer reach".to_string(),
//             consumable: false,
//             effects: None,
//         }),
//         (MenuItem(6), ShopItem {
//             id: 7,
//             name: "Scimitar".to_string(),
//             quality: ItemQuality::Uncommon,
//             equip_slot: EquipmentSlot::MainHand,
//             stats: Some(ItemStats {
//                 attack_modifiers: None,
//                 defense_modifiers: None,
//                 other_modifiers: None,
//                 strength: None,
//                 intelligence: None,
//                 agility: None,
//             }),
//             price: 20,
//             description: "Melee attacks cut deeper".to_string(),
//             consumable: false,
//             effects: None,
//         }),
//         (MenuItem(7), ShopItem {
//             id: 8,
//             name: "Netherwood Staff".to_string(),
//             quality: ItemQuality::Uncommon,
//             equip_slot: EquipmentSlot::MainHand,
//             stats: Some(ItemStats {
//                 attack_modifiers: Some(AttackModifiers {
//                     damage_bonus: 3,
//                     hit_rating_bonus: 2,
//                     range_bonus: 20,
//                     cooldown_reduction_ms: 0,
//                 }),
//                 defense_modifiers: None,
//                 other_modifiers: None,
//                 strength: None,
//                 intelligence: None,
//                 agility: None,
//             }),
//             price: 20,
//             description: "Curses last longer".to_string(),
//             consumable: false,
//             effects: None,
//         }),
//         (MenuItem(8), ShopItem {
//             id: 9,
//             name: "Nether-orb Staff".to_string(),
//             quality: ItemQuality::Rare,
//             equip_slot: EquipmentSlot::MainHand,
//             stats: Some(ItemStats {
//                 attack_modifiers: Some(AttackModifiers {
//                     damage_bonus: 3,
//                     hit_rating_bonus: 2,
//                     range_bonus: 20,
//                     cooldown_reduction_ms: 0,
//                 }),
//                 defense_modifiers: None,
//                 other_modifiers: None,
//                 strength: None,
//                 intelligence: None,
//                 agility: None,
//             }),
//             price: 40,
//             description: "More elemental damage".to_string(),
//             consumable: false,
//             effects: None,
//         }),
//         (MenuItem(9), ShopItem {
//             id: 576,
//             name: "Cinder Slippers".to_string(),
//             quality: ItemQuality::Rare,
//             equip_slot: EquipmentSlot::Feet,
//             stats: Some(ItemStats {
//                 attack_modifiers: None,
//                 defense_modifiers: None,
//                 other_modifiers: Some(OtherModifiers {
//                     movement_speed_increase: 1,
//                 }),
//                 strength: None,
//                 intelligence: None,
//                 agility: None,
//             }),
//             price: 40,
//             description: "More elemental resistance".to_string(),
//             consumable: false,
//             effects: None,
//         }),
//         (MenuItem(10), ShopItem {
//             id: 1,
//             name: "Elven Boots".to_string(),
//             quality: ItemQuality::Uncommon,
//             equip_slot: EquipmentSlot::Head,
//             stats: Some(ItemStats {
//                 attack_modifiers: None,
//                 defense_modifiers: None,
//                 other_modifiers: Some(OtherModifiers {
//                     movement_speed_increase: 1,
//                 }),
//                 strength: None,
//                 intelligence: None,
//                 agility: None,
//             }),
//             price: 30,
//             description: "More movement speed".to_string(),
//             consumable: false,
//             effects: None,
//         }),
//         (MenuItem(11),ShopItem {
//             id: 26,
//             name: "Healing Potion".to_string(),
//             quality: Common,
//             equip_slot: EquipmentSlot::UtilitySlot,
//             stats: None,
//             price: 3,
//             description: "Restores 4hp".to_string(),
//             consumable: true,
//             effects: Some(vec![common::Effect::Heal(4)])
//         })
//     ])
// }