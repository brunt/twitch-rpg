mod item_shop;
mod sprites;
mod util;

use crate::item_shop::{ShopItem, draw_shop_interface};
use crate::sprites::items_sprites::{
    ITEMS_SPRITE_0, ITEMS_SPRITE_25, ITEMS_SPRITE_26, ITEMS_SPRITE_27,
};
use crate::sprites::monsters_sprites::*;
use crate::sprites::terrain_sprites::*;
use crate::sprites::{SPRITE_DIMENSION, SpriteRect};
use crate::util::{draw_item_sprite, draw_sprite, load_images};
use common::{GameSnapShot, Health, PlayerClass, PlayerSnapshot};
use leptos::html::Canvas;
use leptos::mount::mount_to_body;
use leptos::prelude::{
    ClassAttribute, Effect, ElementChild, Get, IntoInner, NodeRef, NodeRefAttribute, Set, Signal,
    signal,
};
use leptos::{IntoView, component, view};
use std::ops::Deref;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};
use web_sys::{CanvasRenderingContext2d, EventSource, HtmlImageElement, MessageEvent};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (gamestate, set_gamestate) = signal::<Option<GameSnapShot>>(None);

    Effect::new(move |_| {
        let mut sse_event = EventSource::new("/sse").expect_throw("Failed to create EventSource");
        let callback = Closure::wrap(Box::new(move |event: MessageEvent| {
            if let Some(text) = event.data().as_string() {
                set_gamestate
                    .set(serde_json::from_str(&text).expect_throw("Failed to parse game state"));
            }
        }) as Box<dyn FnMut(MessageEvent)>);
        sse_event.set_onmessage(Some(callback.as_ref().unchecked_ref()));

        callback.forget();
    });

    view! {
        <div class="flex flex-row gap-2">
            <GameCanvas gs=gamestate />
            <SidePanelCharacterSheet gs=gamestate />
        </div>
        <BottomPanel />
    }
}

#[component]
fn GameCanvas(#[prop(into)] gs: Signal<Option<GameSnapShot>>) -> impl IntoView {
    let canvas_ref: NodeRef<Canvas> = NodeRef::new();
    const CANVAS_WIDTH: f64 = 1280.0;
    const CANVAS_HEIGHT: f64 = 720.0;
    const MAP_WIDTH: usize = 20;
    const MAP_HEIGHT: usize = 25;

    // TODO: offscreen canvas 🤷🏻‍♂️
    // request_animation_frame(move || canvas_ref);

    Effect::new(move |_| {
        if let Some(canvas) = canvas_ref.get() {
            let ctx = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();

            // load sprites
            let [
                terrain_image,
                monster_image,
                item_image,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
            ] = load_images();

            // clone for usage in closure
            let closure_terrain_image = terrain_image.clone();
            let closure_monster_image = monster_image.clone();
            let closure_item_image = item_image.clone();

            let ctx_clone = ctx.clone();

            let onload = Closure::<dyn FnMut()>::new(move || {
                // backmost layer: black
                ctx.set_fill_style_str("black");
                ctx.fill_rect(0.0, 0.0, CANVAS_WIDTH, CANVAS_HEIGHT);

                // // second backmost layer: terrain
                for row in 0..MAP_WIDTH {
                    // rows {
                    for col in 0..MAP_HEIGHT {
                        //cols {
                        let screen_x = (row as f64 - col as f64) * (SPRITE_DIMENSION / 2.0)
                            + CANVAS_WIDTH / 2.0;
                        let screen_y = (row as f64 + col as f64) * (SPRITE_DIMENSION / 2.0);

                        // edge calculation
                        let is_top_left_corner = row == 0 && col == 0;
                        let is_top_right_corner = row == 0 && col == MAP_HEIGHT - 1;
                        let is_bottom_left_corner = row == MAP_WIDTH - 1 && col == 0;
                        let is_top_corner = row == MAP_WIDTH - 1 && col == MAP_HEIGHT - 1;
                        let is_ne_sw = row == 0 || row == MAP_WIDTH - 1;
                        let is_nw_se = col == 0 || col == MAP_HEIGHT - 1;

                        let x = (col as f64 - row as f64) * (SPRITE_DIMENSION / 2.0)
                            + CANVAS_WIDTH / 2.0
                            - SPRITE_DIMENSION / 2.0;
                        let y = (col as f64 + row as f64) * (SPRITE_DIMENSION / 4.0);
                        draw_sprite(
                            &ctx_clone,
                            &closure_terrain_image,
                            &TERRAIN_SPRITE_653,
                            x,
                            y,
                            None,
                        ); //grass sprite

                        if is_ne_sw {
                            draw_sprite(
                                &ctx_clone,
                                &closure_terrain_image,
                                &TERRAIN_SPRITE_467,
                                x,
                                y,
                                None,
                            ); //tree sprites
                        }
                        if is_nw_se {
                            draw_sprite(
                                &ctx_clone,
                                &closure_terrain_image,
                                &TERRAIN_SPRITE_432,
                                x,
                                y,
                                None,
                            );
                        }

                        if is_top_left_corner {
                            draw_sprite(
                                &ctx_clone,
                                &closure_terrain_image,
                                &TERRAIN_SPRITE_432,
                                x,
                                y,
                                None,
                            );
                        }

                        //         // sprites need to be drawn from the top rows down
                        draw_sprite(
                            &ctx_clone,
                            &closure_terrain_image,
                            &TERRAIN_SPRITE_158,
                            SPRITE_DIMENSION * 6.0,
                            SPRITE_DIMENSION * 5.0,
                            None,
                        );
                        draw_sprite(
                            &ctx_clone,
                            &closure_terrain_image,
                            &TERRAIN_SPRITE_218,
                            SPRITE_DIMENSION * 5.0,
                            SPRITE_DIMENSION * 5.0,
                            None,
                        );
                        draw_sprite(
                            &ctx_clone,
                            &closure_terrain_image,
                            &TERRAIN_SPRITE_219,
                            SPRITE_DIMENSION * 5.5,
                            SPRITE_DIMENSION * 5.25,
                            None,
                        );
                        draw_sprite(
                            &ctx_clone,
                            &closure_monster_image,
                            &MONSTERS_SPRITE_339,
                            SPRITE_DIMENSION * 7.5,
                            SPRITE_DIMENSION * 5.0,
                            None,
                        );
                    }
                }

                let shop_item = ShopItem {
                    name: "Name".to_string(),
                    description: "desc".to_string(),
                    price: 10,
                    sprite: ITEMS_SPRITE_27,
                };
                draw_shop_interface(
                    &ctx,
                    &closure_item_image,
                    &[shop_item],
                    30.0,
                    30.0,
                    100.0,
                    100.0,
                    2,
                )
            });
            terrain_image.set_onload(Some(onload.as_ref().unchecked_ref()));
            monster_image.set_onload(Some(onload.as_ref().unchecked_ref()));
            item_image.set_onload(Some(onload.as_ref().unchecked_ref()));
            onload.forget();
        }
    });

    view! { <canvas node_ref=canvas_ref width=CANVAS_WIDTH height=CANVAS_HEIGHT /> }
}

#[component]
fn SidePanelCharacterSheet(#[prop(into)] gs: Signal<Option<GameSnapShot>>) -> impl IntoView {
    view! {
        <aside class="w-64 bg-panel rounded shadow-lg text-sm overflow-y-auto max-h-[720px]">
            <div class="border-b border-gray-700 px-3 py-2 font-semibold text-base">Characters</div>
            <div class="divide-y divide-gray-700">
                {move || {
                    let gs = gs.get();
                    if let Some(gs) = gs {
                        leptos::logging::log!("{:?}", gs.party.clone());
                        gs.party
                            .iter()
                            .map(|player| {
                                leptos::logging::log!("{:?}", player);

                                view! {
                                    <div class="flex items-center gap-2 px-3 py-2">
                                        <PlayerSpriteCanvas sprite=WIZARD_SPRITES
                                            .get(&player.sprite_name)
                                            .unwrap() />
                                        <div class="font-semibold text-base">
                                            {player.name.clone()}
                                        </div>
                                        <div class="font-semibold text-base">
                                            Level: {player.level}
                                        </div>
                                        <div class="font-semibold text-base">
                                            Gold: {player.gold}
                                        </div>

                                    </div>
                                }
                            })
                            .collect::<Vec<_>>()
                    } else {
                        leptos::logging::log!("gs is none");
                        vec![]
                    }
                }}

            </div>
        </aside>
    }
}

#[component]
fn PlayerSpriteCanvas(#[prop(into)] sprite: &'static SpriteRect) -> impl IntoView {
    let canvas_ref: NodeRef<Canvas> = NodeRef::new();
    Effect::new(move |_| {
        if let Some(canvas) = canvas_ref.get() {
            let ctx = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();

            // Create and load the image
            let monster_image = HtmlImageElement::new().unwrap();
            let closure_monster_image = monster_image.clone();
            let ctx_clone = ctx.clone();

            let onload = Closure::<dyn FnMut()>::new(move || {
                draw_sprite(&ctx_clone, &closure_monster_image, &sprite, 0.0, 0.0, None);
            });

            monster_image.set_onload(Some(onload.as_ref().unchecked_ref()));
            monster_image.set_src("public/sprites/monsters.png");
            onload.forget();
        }
    });

    view! { <canvas node_ref=canvas_ref width=SPRITE_DIMENSION height=SPRITE_DIMENSION /> }
}

#[component]
fn BottomPanel() -> impl IntoView {
    view! {
        <footer class="w-[calc(1280px+0.5rem+256px)] bg-panel rounded shadow-lg text-sm p-3 overflow-y-auto max-h-32">
            <div class="font-semibold mb-1">Game Log</div>
            <div class="space-y-1 text-xs"></div>
        </footer>
    }
}
