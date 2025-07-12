mod components;
mod dungeon_floor;
mod item_shop;
mod sprites;

use common::{
    GameSnapShot, Health, PlayerClass, PlayerSnapshot, SerializedCountdownTimer, ShopItem,
};
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
    Effect::new(move |_| {
        let mock_snapshot = GameSnapShot {
            // Fill with test data
            floor: None,
            ready_timer: Some(SerializedCountdownTimer { remaining: 4 }),
            shop_items: Some(vec![
                ShopItem {
                    sprite: "red_potion".to_string(),
                    name: "Healing Potion".to_string(),
                    price: 10,
                    description: "Restores 5 health once and then it goes away forever".to_string(),
                },
                ShopItem {
                    sprite: "wizard_boots".to_string(),
                    name: "Wizard Boots".to_string(),
                    price: 60,
                    description: "Increases defense slightly".to_string(),
                },
                ShopItem {
                    sprite: "elven_boots".to_string(),
                    name: "Elven Boots".to_string(),
                    price: 100,
                    description: "Increases movement speed".to_string(),
                },
                ShopItem {
                    sprite: "elven_boots".to_string(),
                    name: "Elven Boots".to_string(),
                    price: 100,
                    description: "Increases movement speed".to_string(),
                },
                ShopItem {
                    sprite: "elven_boots".to_string(),
                    name: "Elven Boots".to_string(),
                    price: 100,
                    description: "Increases movement speed".to_string(),
                },
                ShopItem {
                    sprite: "elven_boots".to_string(),
                    name: "Elven Boots".to_string(),
                    price: 100,
                    description: "Increases movement speed".to_string(),
                },
                ShopItem {
                    sprite: "elven_boots".to_string(),
                    name: "Elven Boots".to_string(),
                    price: 100,
                    description: "Increases movement speed".to_string(),
                },
                ShopItem {
                    sprite: "elven_boots".to_string(),
                    name: "Elven Boots".to_string(),
                    price: 100,
                    description: "Increases movement speed".to_string(),
                }
            ]),
            party: vec![
                PlayerSnapshot {
                    name: "xMellowMonkeyx".to_string(),
                    sprite_name: "wizard_max".to_string(),
                    class: Some(PlayerClass::Wizard),
                    health: Health::Alive {hp: 4, max_hp: 6},
                    level: 5,
                    gold: 100,
                }
            ],
            floor_positions: None,
        };
        set_gamestate.set(Some(mock_snapshot));
    });

    // TODO: use this for real server communication
    // Effect::new(move |_| {
    //     let mut sse_event = EventSource::new("/sse").expect_throw("Failed to create EventSource");
    //     let callback = Closure::wrap(Box::new(move |event: MessageEvent| {
    //         if let Some(text) = event.data().as_string() {
    //             set_gamestate
    //                 .set(serde_json::from_str(&text).expect_throw("Failed to parse game state"));
    //         }
    //     }) as Box<dyn FnMut(MessageEvent)>);
    //     sse_event.set_onmessage(Some(callback.as_ref().unchecked_ref()));
    // 
    //     callback.forget();
    // });

    view! {
        <div class="flex flex-row gap-2">
            <GameCanvas gs=gamestate />
            <SidePanelCharacterSheet gs=gamestate />
        </div>
        <BottomPanel />
    }
}
