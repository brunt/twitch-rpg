mod sprites;
mod state;

use crate::sprites::monsters_sprites::*;
use crate::sprites::terrain_sprites::*;
use crate::sprites::{SPRITE_DIMENSION, SpriteRect};
use common::{GameSnapShot, Health, PlayerClass, PlayerSnapshot};
use leptos::html::Canvas;
use leptos::mount::mount_to_body;
use leptos::prelude::{
    ClassAttribute, Effect, ElementChild, Get, IntoInner, LocalStorage, NodeRef, NodeRefAttribute,
    ReadSignal, Set, Signal, on_cleanup, signal, signal_local,
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
    // let (gamestate, set_gamestate) = signal_local::<Option<GameSnapShot>>(None);
    let (gamestate, set_gamestate) = signal::<Option<GameSnapShot>>(None);

    // let set_gamestate_clone = set_gamestate.clone();
    Effect::new(move |_| {
        let mut sse_event = EventSource::new("/sse").expect_throw("Failed to create EventSource");
        let callback = Closure::wrap(Box::new(move |event: MessageEvent| {
            if let Some(text) = event.data().as_string() {
                // leptos::logging::log!("{}", text.clone());
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

fn draw_sprite(
    ctx: &CanvasRenderingContext2d,
    image: &HtmlImageElement,
    sprite: &SpriteRect,
    x: f64,
    y: f64,
) -> Result<(), JsValue> {
    ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        image,
        sprite.x,
        sprite.y,
        sprite.w,
        sprite.h,
        x,
        y,
        SPRITE_DIMENSION,
        SPRITE_DIMENSION,
    )
}

#[component]
fn SidePanelCharacterSheet(#[prop(into)] gs: Signal<Option<GameSnapShot>>) -> impl IntoView {
    //TODO: subset game state cloned into this function as well as the main canvas

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

            // Create and load the image
            let terrain_image = HtmlImageElement::new().unwrap();
            let closure_terrain_image = terrain_image.clone();
            let monster_image = HtmlImageElement::new().unwrap();
            let closure_monster_image = monster_image.clone();
            let ctx_clone = ctx.clone();

            let onload = Closure::<dyn FnMut()>::new(move || {
                // backmost layer: black
                ctx.set_fill_style(&JsValue::from_str("black"));
                ctx.fill_rect(0.0, 0.0, CANVAS_WIDTH, CANVAS_HEIGHT);

                // second backmost layer: terrain
                // let cols = (CANVAS_WIDTH / TERRAIN_SPRITE_DIMENSION) as usize * 2;
                // let rows = (CANVAS_HEIGHT / (TERRAIN_SPRITE_DIMENSION / 2.0)).ceil() as usize;
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
                        )
                        .unwrap(); //grass sprite

                        if is_ne_sw {
                            draw_sprite(
                                &ctx_clone,
                                &closure_terrain_image,
                                &TERRAIN_SPRITE_467,
                                x,
                                y,
                            ) //tree sprites
                            .unwrap()
                        }
                        if is_nw_se {
                            draw_sprite(
                                &ctx_clone,
                                &closure_terrain_image,
                                &TERRAIN_SPRITE_432,
                                x,
                                y,
                            )
                            .unwrap()
                        }

                        if is_top_left_corner {
                            draw_sprite(
                                &ctx_clone,
                                &closure_terrain_image,
                                &TERRAIN_SPRITE_432,
                                x,
                                y,
                            )
                            .unwrap()
                        }

                        // sprites need to be drawn from the top rows down
                        draw_sprite(
                            &ctx_clone,
                            &closure_terrain_image,
                            &TERRAIN_SPRITE_158,
                            SPRITE_DIMENSION * 6.0,
                            SPRITE_DIMENSION * 5.0,
                        )
                        .unwrap();
                        draw_sprite(
                            &ctx_clone,
                            &closure_terrain_image,
                            &TERRAIN_SPRITE_218,
                            SPRITE_DIMENSION * 5.0,
                            SPRITE_DIMENSION * 5.0,
                        )
                        .unwrap();
                        draw_sprite(
                            &ctx_clone,
                            &closure_terrain_image,
                            &TERRAIN_SPRITE_219,
                            SPRITE_DIMENSION * 5.5,
                            SPRITE_DIMENSION * 5.25,
                        )
                        .unwrap();
                        draw_sprite(
                            &ctx_clone,
                            &closure_monster_image,
                            &MONSTERS_SPRITE_934,
                            SPRITE_DIMENSION * 7.5,
                            SPRITE_DIMENSION * 5.0,
                        )
                        .unwrap();
                    }
                }
            });

            terrain_image.set_onload(Some(onload.as_ref().unchecked_ref()));
            terrain_image.set_src("public/sprites/terrain.png");
            monster_image.set_onload(Some(onload.as_ref().unchecked_ref()));
            monster_image.set_src("public/sprites/monsters.png");
            onload.forget();
        }
    });

    // spawn_local(async move {
    //    let mut sse_event = EventSource::new("/sse").expect_throw("Failed to create EventSource");
    //     let callback = Closure::wrap(Box::new(move |event: MessageEvent| {
    //         if let Ok(text) = event.data().as_string() {
    //
    //         }
    //     }));
    //
    //     while let Some(callback) = sse_event.onmessage() {
    //
    //     }
    //     // while let Some(event) = sse_event.onmessage() {
    //     //     if let Ok(event) = serde_json::from_str::<GameSnapShot>(event.data()) {
    //     //
    //     //     }
    //     // }
    // });

    view! { <canvas node_ref=canvas_ref width=CANVAS_WIDTH height=CANVAS_HEIGHT /> }
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
                draw_sprite(&ctx_clone, &closure_monster_image, &sprite, 0.0, 0.0).unwrap();
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

// #[component]
// pub fn SseListener(props: SseListenerProps) -> impl IntoView {
//     use_effect(move || {
//         let event_source = EventSource::new("https://your-sse-endpoint")
//             .expect("failed to create EventSource");
//
//         let set_messages = props.set_messages;
//         let on_message = Closure::wrap(Box::new(move |event: MessageEvent| {
//             if let Ok(data) = event.data().as_string() {
//                 set_messages.update(|msgs| msgs.push(data));
//             }
//         }) as Box<dyn FnMut(MessageEvent)>);
//
//         event_source.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
//         on_message.forget();
//
//         move || {
//             event_source.close();
//         }
//     });
//
//     view! { <></> }
// }
