use crate::components::draw_item_sprite;
use crate::sprites::ITEM_SPRITE_DIMENSION;
use crate::sprites::items_sprites::ITEMS_SPRITES;
use common::GameSnapShot;
use leptos::html::Canvas;
use leptos::prelude::Get;
use leptos::prelude::NodeRefAttribute;
use leptos::prelude::*;
use leptos::prelude::{ClassAttribute, Effect, NodeRef};
use leptos::{IntoView, component, view};
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlImageElement;

#[component]
pub fn BottomPanel(#[prop(into)] gs: Signal<Option<GameSnapShot>>) -> impl IntoView {
    view! {
        <footer class="w-[1280px] bg-panel rounded shadow-lg text-sm p-3 overflow-y-auto max-h-32">
            <div class="font-semibold mb-1">Treasures Found:</div>
            <div class="space-y-1 text-xs">
                <DungeonLootCanvas />
                {move || {
                    gs.get()
                        .and_then(|gs| gs.loot)
                        .map(|loot| {
                            view! {
                                <div class="font-semibold mb-1">
                                    <span>{loot}</span>
                                </div>
                            }
                                .into_any()
                        })
                        .unwrap_or_else(|| view! {}.into_any())
                }}
            </div>
        </footer>
    }
}

#[component]
fn DungeonLootCanvas() -> impl IntoView {
    let canvas_ref: NodeRef<Canvas> = NodeRef::new();
    Effect::new(move |_| {
        let Some(canvas) = canvas_ref.get() else {
            return;
        };
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        let coin_image = HtmlImageElement::new().unwrap();
        let closure_coin_image = coin_image.clone();
        let onload = Closure::<dyn FnMut()>::new(move || {
            draw_item_sprite(&ctx, &closure_coin_image, ITEMS_SPRITES.get(&704), 0.0, 0.0)
        });
        coin_image.set_onload(Some(onload.as_ref().unchecked_ref()));
        coin_image.set_src("public/sprites/items.png");
        onload.forget();
    });

    view! {
        <canvas
            node_ref=canvas_ref
            width=ITEM_SPRITE_DIMENSION
            height=ITEM_SPRITE_DIMENSION
            class="inline-block align-middle"
        />
    }
}
