mod components;
mod dungeon_floor;
mod item_shop;
mod sprites;

use common::EquipmentSlot;
use common::GameSnapShot;
use common::Health;
use common::Item;
use common::ItemQuality::Common;
use common::PlayerSnapshot;
use common::PlayerStats;
use common::*;
use components::bottom_panel::BottomPanel;
use components::game_canvas::GameCanvas;
use components::side_panel::SidePanelCharacterSheet;
use leptos::context::provide_context;
use leptos::mount::mount_to_body;
use leptos::prelude::{
    ClassAttribute, Effect, ElementChild, Get, GetUntracked, IntoInner, NodeRefAttribute, Set,
    StyleAttribute, signal, signal_local,
};
use leptos::{IntoView, component, view};
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{EventSource, HtmlImageElement, MessageEvent};

#[derive(Clone)]
pub struct SpriteSheets {
    pub terrain: Rc<HtmlImageElement>,
    pub items: Rc<HtmlImageElement>,
    pub monsters: Rc<HtmlImageElement>,
    pub projectiles: Rc<HtmlImageElement>,
    pub spellfx1: Rc<HtmlImageElement>,
    pub spellfx2: Rc<HtmlImageElement>,
    pub spellfx3: Rc<HtmlImageElement>,
    pub spellfx4: Rc<HtmlImageElement>,
    pub spellfx5: Rc<HtmlImageElement>,
    pub damage_fx: Rc<HtmlImageElement>,
    pub background: Rc<HtmlImageElement>,
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    // create a single context for all sprites
    let terrain = Rc::new(HtmlImageElement::new().unwrap());
    let monsters = Rc::new(HtmlImageElement::new().unwrap());
    let items = Rc::new(HtmlImageElement::new().unwrap());
    let spellfx1 = Rc::new(HtmlImageElement::new().unwrap());
    let spellfx2 = Rc::new(HtmlImageElement::new().unwrap());
    let spellfx3 = Rc::new(HtmlImageElement::new().unwrap());
    let spellfx4 = Rc::new(HtmlImageElement::new().unwrap());
    let spellfx5 = Rc::new(HtmlImageElement::new().unwrap());
    let damage_fx = Rc::new(HtmlImageElement::new().unwrap());
    let projectiles = Rc::new(HtmlImageElement::new().unwrap());
    let background = Rc::new(HtmlImageElement::new().unwrap());

    terrain.set_src("public/sprites/terrain.png");
    items.set_src("public/sprites/items.png");
    monsters.set_src("public/sprites/monsters.png");
    spellfx1.set_src("public/sprites/SpellFXAnim1.png");
    spellfx2.set_src("public/sprites/SpellFXAnim2.png");
    spellfx3.set_src("public/sprites/SpellFXAnim3.png");
    spellfx4.set_src("public/sprites/SpellFXAnim4.png");
    spellfx5.set_src("public/sprites/SpellFXAnim5.png");
    damage_fx.set_src("public/sprites/DamageFX.png");
    projectiles.set_src("public/sprites/SpellFXMissiles.png");
    projectiles.set_src("public/sprites/SpellFXMissiles.png");
    background.set_src("public/img/bg.jpg");

    let (sprites, _) = signal_local(SpriteSheets {
        items,
        monsters,
        projectiles,
        spellfx1,
        spellfx2,
        spellfx3,
        spellfx4,
        spellfx5,
        terrain,
        damage_fx,
        background,
    });
    provide_context(sprites);

    let (gamestate, set_gamestate) = signal::<Option<GameSnapShot>>(None);

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

    // MOCK DATA FOR LOCAL DEV
    // Effect::new(move |_| {
    //     use ItemQuality::*;
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
    //                 equipped_items: HashMap::from([(
    //                     EquipmentSlot::UtilitySlot,
    //                     Item {
    //                         item_id: 26,
    //                         name: "Healing Potion".to_string(),
    //                         quality: Common,
    //                         equip_slot: EquipmentSlot::MainHand,
    //                         stats: None,
    //                         consumable: true,
    //                         effects: None,
    //                         description: "".to_string(),
    //                     },
    //                 )]),
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
    //                 equipped_items: HashMap::from([
    //                     (
    //                         EquipmentSlot::Feet,
    //                         Item {
    //                             item_id: 1,
    //                             name: "boots".to_string(),
    //                             quality: Common,
    //                             equip_slot: EquipmentSlot::Feet,
    //                             stats: None,
    //                             consumable: false,
    //                             effects: None,
    //                             description: "".to_string(),
    //                         },
    //                     ),
    //                     (
    //                         EquipmentSlot::MainHand,
    //                         Item {
    //                             item_id: 2,
    //                             name: "flogger".to_string(),
    //                             quality: Uncommon,
    //                             equip_slot: EquipmentSlot::MainHand,
    //                             stats: None,
    //                             consumable: false,
    //                             effects: None,
    //                             description: "".to_string(),
    //                         },
    //                     ),
    //                     (
    //                         EquipmentSlot::Body,
    //                         Item {
    //                             item_id: 555,
    //                             name: "flogger".to_string(),
    //                             quality: Epic,
    //                             equip_slot: EquipmentSlot::MainHand,
    //                             stats: None,
    //                             consumable: false,
    //                             effects: None,
    //                             description: "".to_string(),
    //                         },
    //                     ),
    //                     (
    //                         EquipmentSlot::Finger,
    //                         Item {
    //                             item_id: 44,
    //                             name: "flogger".to_string(),
    //                             quality: Epic,
    //                             equip_slot: EquipmentSlot::Finger,
    //                             stats: None,
    //                             consumable: false,
    //                             effects: None,
    //                             description: "".to_string(),
    //                         },
    //                     ),
    //                 ]),
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
    //         loot: None,
    //     };
    //     set_gamestate.set(Some(mock_snapshot));
    // });

    view! {
        <div class="flex flex-row">
            <div>
                <div class="w-[1280px] h-[720px]">
                    <GameCanvas gs=gamestate />
                </div>
                <BottomPanel gs=gamestate />
            </div>
            <SidePanelCharacterSheet gs=gamestate />
        </div>
    }
}
// //TODO: delete after local testing
// pub fn initialize_shop_items() -> Vec<(MenuItem, ShopItem)> {
//     vec![
//         (
//             MenuItem(0),
//             ShopItem {
//                 id: 288,
//                 name: "Academy Teacher's Hat".to_string(),
//                 quality: ItemQuality::Legendary,
//                 equip_slot: EquipmentSlot::Head,
//                 stats: Some(ItemStats {
//                     attack_modifiers: None,
//                     defense_modifiers: Some(DefenseModifiers {
//                         damage_reduction: Some(1),
//                         evasion_rating: None,
//                     }),
//                     other_modifiers: None,
//                     strength: None,
//                     intelligence: Some(9),
//                     agility: None,
//                 }),
//                 price: 230,
//                 description: "Spells have longer durations and affect larger areas".to_string(),
//                 consumable: false,
//                 effects: None,
//             },
//         ),
//         (
//             MenuItem(1),
//             ShopItem {
//                 id: 2,
//                 name: "Mace".to_string(),
//                 quality: Common,
//                 equip_slot: EquipmentSlot::MainHand,
//                 stats: Some(ItemStats {
//                     attack_modifiers: Some(AttackModifiers {
//                         damage_bonus: Some(1),
//                         hit_rating_bonus: Some(3),
//                         range_bonus: None,
//                         cooldown_reduction_ms: None,
//                         crit_damage_multiplier: None,
//                     }),
//                     defense_modifiers: None,
//                     other_modifiers: None,
//                     strength: None,
//                     intelligence: None,
//                     agility: None,
//                 }),
//                 price: 15,
//                 description: "Staggers opponents on hit".to_string(),
//                 consumable: false,
//                 effects: None,
//             },
//         ),
//         (
//             MenuItem(2),
//             ShopItem {
//                 id: 538,
//                 name: "Wooden Buckler".to_string(),
//                 quality: ItemQuality::Common,
//                 equip_slot: EquipmentSlot::OffHand,
//                 stats: Some(ItemStats {
//                     attack_modifiers: Some(AttackModifiers {
//                         damage_bonus: Some(-4),
//                         hit_rating_bonus: Some(-5),
//                         range_bonus: Some(-10),
//                         cooldown_reduction_ms: Some(-1000),
//                         crit_damage_multiplier: None,
//                     }),
//                     defense_modifiers: Some(DefenseModifiers {
//                         damage_reduction: Some(5),
//                         evasion_rating: Some(4),
//                     }),
//                     other_modifiers: None,
//                     strength: Some(4),
//                     intelligence: Some(2),
//                     agility: None,
//                 }),
//                 price: 15,
//                 description: "Reduces incoming and outgoing damage".to_string(),
//                 consumable: false,
//                 effects: None,
//             },
//         ),
//         (
//             MenuItem(3),
//             ShopItem {
//                 id: 4,
//                 name: "Trident".to_string(),
//                 quality: ItemQuality::Common,
//                 equip_slot: EquipmentSlot::MainHand,
//                 stats: Some(ItemStats {
//                     attack_modifiers: None,
//                     defense_modifiers: None,
//                     other_modifiers: None,
//                     strength: None,
//                     intelligence: None,
//                     agility: None,
//                 }),
//                 price: 18,
//                 description: "Melee attacks have longer reach".to_string(),
//                 consumable: false,
//                 effects: None,
//             },
//         ),
//         (
//             MenuItem(4),
//             ShopItem {
//                 id: 5,
//                 name: "Greatsword".to_string(),
//                 quality: ItemQuality::Uncommon,
//                 equip_slot: EquipmentSlot::MainHand,
//                 stats: Some(ItemStats {
//                     attack_modifiers: Some(AttackModifiers {
//                         damage_bonus: Some(3),
//                         hit_rating_bonus: Some(2),
//                         range_bonus: Some(2),
//                         cooldown_reduction_ms: None,
//                         crit_damage_multiplier: None,
//                     }),
//                     defense_modifiers: None,
//                     other_modifiers: None,
//                     strength: None,
//                     intelligence: None,
//                     agility: None,
//                 }),
//                 price: 28,
//                 description: "Melee attacks have longer reach".to_string(),
//                 consumable: false,
//                 effects: None,
//             },
//         ),
//         (
//             MenuItem(5),
//             ShopItem {
//                 id: 6,
//                 name: "Longsword".to_string(),
//                 quality: ItemQuality::Rare,
//                 equip_slot: EquipmentSlot::Head,
//                 stats: Some(ItemStats {
//                     attack_modifiers: Some(AttackModifiers {
//                         damage_bonus: Some(-4),
//                         hit_rating_bonus: Some(-5),
//                         range_bonus: Some(-10),
//                         cooldown_reduction_ms: Some(-1000),
//                         crit_damage_multiplier: None,
//                     }),
//                     defense_modifiers: Some(DefenseModifiers {
//                         damage_reduction: Some(5),
//                         evasion_rating: Some(4),
//                     }),
//                     other_modifiers: None,
//                     strength: Some(4),
//                     intelligence: Some(2),
//                     agility: None,
//                 }),
//                 price: 34,
//                 description: "Melee attacks have longer reach".to_string(),
//                 consumable: false,
//                 effects: None,
//             },
//         ),
//         (
//             MenuItem(6),
//             ShopItem {
//                 id: 7,
//                 name: "Scimitar".to_string(),
//                 quality: ItemQuality::Uncommon,
//                 equip_slot: EquipmentSlot::MainHand,
//                 stats: Some(ItemStats {
//                     attack_modifiers: None,
//                     defense_modifiers: None,
//                     other_modifiers: None,
//                     strength: None,
//                     intelligence: None,
//                     agility: None,
//                 }),
//                 price: 20,
//                 description: "Melee attacks cut deeper".to_string(),
//                 consumable: false,
//                 effects: None,
//             },
//         ),
//         (
//             MenuItem(7),
//             ShopItem {
//                 id: 8,
//                 name: "Netherwood Staff".to_string(),
//                 quality: ItemQuality::Uncommon,
//                 equip_slot: EquipmentSlot::MainHand,
//                 stats: Some(ItemStats {
//                     attack_modifiers: Some(AttackModifiers {
//                         damage_bonus: Some(3),
//                         hit_rating_bonus: Some(2),
//                         range_bonus: Some(20),
//                         cooldown_reduction_ms: None,
//                         crit_damage_multiplier: None,
//                     }),
//                     defense_modifiers: None,
//                     other_modifiers: None,
//                     strength: None,
//                     intelligence: None,
//                     agility: None,
//                 }),
//                 price: 20,
//                 description: "Curses last longer".to_string(),
//                 consumable: false,
//                 effects: None,
//             },
//         ),
//     ]
// }
