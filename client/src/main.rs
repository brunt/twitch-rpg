mod components;
mod dungeon_floor;
mod item_shop;
mod sprites;

use common::GameSnapShot;
use components::bottom_panel::BottomPanel;
use components::game_canvas::GameCanvas;
use components::side_panel::SidePanelCharacterSheet;
use leptos::mount::mount_to_body;
use leptos::prelude::{
    ClassAttribute, Effect, ElementChild, Get, GetUntracked, IntoInner, NodeRefAttribute, Set,
    signal,
};
use leptos::{IntoView, component, view};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{EventSource, MessageEvent};

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
    //         floor: Some(),
    //         // ready_timer: Some(SerializedCountdownTimer { remaining: 4 }),
    //         ready_timer: None,
    //         // shop_items: Some(generate_hardcoded_shop_inventory()),
    //         shop_items: None,
    //         party: vec![
    //             PlayerSnapshot {
    //                 name: "xMellowMonkeyx".to_string(),
    //                 class: PlayerClass::Paladin,
    //                 health: Health::Alive { hp: 11, max_hp: 10 },
    //                 level: 2,
    //                 gold: 100,
    //                 form: Form::Normal,
    //             },
    //             PlayerSnapshot {
    //                 name: "Pixelmog".to_string(),
    //                 class: PlayerClass::Ranger,
    //                 health: Health::Alive { hp: 5, max_hp: 10 },
    //                 level: 4,
    //                 gold: 100,
    //                 form: Form::Normal,
    //             },
    //             PlayerSnapshot {
    //                 name: "ubruntu".to_string(),
    //                 class: PlayerClass::Wizard,
    //                 health: Health::Alive { hp: 7, max_hp: 10 },
    //                 level: 2,
    //                 gold: 100,
    //                 form: Form::Normal,
    //             },
    //             PlayerSnapshot {
    //                 name: "Pittinjury".to_string(),
    //                 class: PlayerClass::Cleric,
    //                 health: Health::Alive { hp: 5, max_hp: 10 },
    //                 level: 3,
    //                 gold: 100,
    //                 form: Form::Normal,
    //             },
    //             PlayerSnapshot {
    //                 name: "FrankBlankPrime".to_string(),
    //                 class: PlayerClass::Fighter,
    //                 health: Health::Alive { hp: 5, max_hp: 10 },
    //                 level: 3,
    //                 gold: 100,
    //                 form: Form::Normal,
    //             },
    //             PlayerSnapshot {
    //                 name: "HeroBonZo".to_string(),
    //                 class: PlayerClass::Druid,
    //                 health: Health::Alive { hp: 5, max_hp: 10 },
    //                 level: 3,
    //                 gold: 100,
    //                 form: Form::Normal,
    //             },
    //             PlayerSnapshot {
    //                 name: "bookslap".to_string(),
    //                 class: PlayerClass::Sorcerer,
    //                 health: Health::Alive { hp: 9, max_hp: 10 },
    //                 level: 2,
    //                 gold: 100,
    //                 form: Form::Normal,
    //             },
    //         ],
    //         floor_positions: None,
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
            <GameCanvas gs=gamestate />
            <SidePanelCharacterSheet gs=gamestate />
        </div>
        <BottomPanel />
    }
}

//TODO: delete after local testing
// fn generate_hardcoded_shop_inventory() -> HashMap<MenuItem, ShopItem> {
//     let mut items = HashMap::new();
//
//     items.insert(
//         MenuItem(0),
//             ShopItem {
//                 sprite: "longsword".parse().unwrap(),
//                 name: "Longsword".parse().unwrap(),
//                 quality: ItemQuality::Rare,
//                 equip_slot: EquipmentSlot::Feet,
//                 price: 34,
//                 description: "Melee attacks have longer reach".parse().unwrap(),
//             },
//     );
//
//     items.insert(
//         MenuItem(1),
//             ShopItem {
//                 sprite: "trident".parse().unwrap(),
//                 name: "Trident".parse().unwrap(),
//                 quality: ItemQuality::Common,
//                 equip_slot: EquipmentSlot::Feet,
//                 price: 18,
//                 description: "Melee attacks have longer reach".parse().unwrap(),
//             },
//     );
//
//     items.insert(
//         MenuItem(2),
//
//             ShopItem {
//                 sprite: "greatsword".parse().unwrap(),
//                 name: "Greatsword".parse().unwrap(),
//                 quality: ItemQuality::Uncommon,
//                 equip_slot: EquipmentSlot::Feet,
//                 price: 28,
//                 description: "Melee attacks have longer reach".parse().unwrap(),
//             },
//     );
//
//     items.insert(
//         MenuItem(3),
//
//             ShopItem {
//                 sprite: "purple_tip_silver_staff".parse().unwrap(),
//                 name: "Nether-orb Staff".parse().unwrap(),
//                 quality: ItemQuality::Rare,
//                 equip_slot: EquipmentSlot::Feet,
//                 price: 40,
//                 description: "More elemental damage".parse().unwrap(),
//             },
//
//     );
//
//     items.insert(
//         MenuItem(4),
//
//             ShopItem {
//                 sprite: "green_leather_boots".parse().unwrap(),
//                 name: "Elven Boots".parse().unwrap(),
//                 quality: ItemQuality::Uncommon,
//                 equip_slot: EquipmentSlot::Feet,
//                 price: 30,
//                 description: "More movement speed".parse().unwrap(),
//             },
//
//     );
//
//     items.insert(
//         MenuItem(5),
//
//             ShopItem {
//                 sprite: "trident".parse().unwrap(),
//                 name: "Trident".parse().unwrap(),
//                 quality: ItemQuality::Common,
//                 equip_slot: EquipmentSlot::Feet,
//                 price: 18,
//                 description: "Melee attacks have longer reach".parse().unwrap(),
//             },
//
//     );
//
//     items.insert(
//         MenuItem(6),
//
//             ShopItem {
//                 sprite: "white_wizard_hat".parse().unwrap(),
//                 name: "Academy Teacher's Hat".parse().unwrap(),
//                 quality: ItemQuality::Legendary,
//                 equip_slot: EquipmentSlot::Feet,
//                 price: 230,
//                 description: "Spells have longer durations and affect larger areas".parse().unwrap(),
//             },
//
//     );
//
//     items.insert(
//         MenuItem(7),
//
//             ShopItem {
//                 sprite: "red_cloth_boots".parse().unwrap(),
//                 name: "Cinder Slippers".parse().unwrap(),
//                 quality: ItemQuality::Rare,
//                 equip_slot: EquipmentSlot::Feet,
//                 price: 40,
//                 description: "More elemental resistance".parse().unwrap(),
//             },
//
//     );
//
//     items
// }
