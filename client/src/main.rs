mod dungeon_floor;
mod item_shop;
mod sprites;
mod util;

use crate::dungeon_floor::draw_dungeon_floor;
use crate::item_shop::{draw_shop_interface};
use crate::sprites::items_sprites::{
    ITEMS_SPRITE_0, ITEMS_SPRITE_25, ITEMS_SPRITE_26, ITEMS_SPRITE_27,
};
use crate::sprites::monsters_sprites::*;
use crate::sprites::terrain_sprites::*;
use crate::sprites::{SPRITE_DIMENSION, SpriteRect};
use crate::util::{AnimationState, draw_item_sprite, draw_sprite, load_images};
use common::{GameSnapShot, Health, PlayerClass, PlayerSnapshot, ShopItem};
use leptos::html::Canvas;
use leptos::mount::mount_to_body;
use leptos::prelude::{
    ClassAttribute, Effect, ElementChild, Get, GetUntracked, IntoInner, NodeRef, NodeRefAttribute,
    Set, Signal, signal,
};
use leptos::{IntoView, component, view};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};
use web_sys::{CanvasRenderingContext2d, EventSource, HtmlImageElement, MessageEvent, window};

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

    let latest_snapshot = Rc::new(RefCell::new(None));
    let animation_state = Rc::new(RefCell::new(AnimationState {
        frame_count: 0,
        last_time: 0.0,
    }));
    let animation_state_for_effect = animation_state.clone();
    let latest_snapshot_for_effect = latest_snapshot.clone();
    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();
    let g_for_effect = g.clone();

    {
        let latest_snapshot = latest_snapshot.clone();
        Effect::new(move |_| {
            *latest_snapshot.borrow_mut() = gs.get();
        });
    }

    Effect::new(move |_| {
        let animation_state = animation_state_for_effect.clone();
        let latest_snapshot = latest_snapshot_for_effect.clone();
        let g = g_for_effect.clone();

        let Some(canvas) = canvas_ref.get() else {
            return;
        };
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        // clone for usage in closure
        let closure_terrain_image = terrain_image.clone();
        let closure_monster_image = monster_image.clone();
        let closure_item_image = item_image.clone();
        let closure = Closure::<dyn FnMut()>::new(move || {
            let mut anim = animation_state.borrow_mut();
            anim.frame_count += 1;

            // Optional: compute time delta for smooth motion
            let now = window().unwrap().performance().unwrap().now();
            let dt = now - anim.last_time;

            // backmost layer: black
            ctx.set_fill_style_str("black");
            ctx.fill_rect(0.0, 0.0, CANVAS_WIDTH, CANVAS_HEIGHT);


            if let Some(snapshot) = latest_snapshot.borrow().as_ref() {
                if let Some(_floor) = gs.get_untracked().and_then(|gs| gs.floor) {
                    draw_dungeon_floor(
                        // &ctx_clone,
                        &ctx,
                        &closure_terrain_image,
                        &closure_monster_image,
                        MAP_WIDTH,
                        MAP_HEIGHT,
                        CANVAS_WIDTH,
                        CANVAS_HEIGHT,
                    );
                } else {
                    draw_shop_interface(
                        &ctx,
                        &closure_item_image,
                        &gs.get().unwrap().shop_items.unwrap_or_default(),
                        30.0,
                        30.0,
                        100.0,
                        100.0,
                        2,
                    )
                }

                ctx.set_font("bold 16px sans-serif");
                ctx.set_fill_style_str("white");
                ctx.fill_text((gs.get_untracked().unwrap().ready_timer.unwrap().remaining).to_string().as_str(), 10.0, 10.0).expect("failed to count down");
            }
            let window = window().unwrap();
            let cb = g.borrow();
            let cb_ref = cb.as_ref().unwrap();
            window.request_animation_frame(cb_ref.as_ref().unchecked_ref()).unwrap();
        });

        *f.borrow_mut() = Some(closure);

        window()
            .unwrap()
            .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .unwrap();
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
                        gs.party
                            .iter()
                            .map(|player| {
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
