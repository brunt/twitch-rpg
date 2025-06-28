mod sprites;

use crate::sprites::monster_sprites::{CASTER_RED_W_STAFF, SKELETAL_MAGE};
use crate::sprites::terrain_sprites::*;
use crate::sprites::{SpriteRect, TERRAIN_SPRITE_DIMENSION};
use leptos::html::{Canvas, S};
use leptos::mount::mount_to_body;
use leptos::prelude::{ClassAttribute, Effect, ElementChild, Get, NodeRef, NodeRefAttribute};
use leptos::{IntoView, component, view};
use std::ops::Deref;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="flex flex-row gap-2">
            <GameCanvas />
            <SidePanelCharacterSheet />
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
        TERRAIN_SPRITE_DIMENSION,
        TERRAIN_SPRITE_DIMENSION,
    )
}

#[component]
fn GameCanvas() -> impl IntoView {
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
                        let screen_x = (row as f64 - col as f64) * (TERRAIN_SPRITE_DIMENSION / 2.0)
                            + CANVAS_WIDTH / 2.0;
                        let screen_y = (row as f64 + col as f64) * (TERRAIN_SPRITE_DIMENSION / 2.0);

                        // edge calculation
                        let is_perimeter =
                            row == 0 || row == MAP_WIDTH - 1 || col == 0 || col == MAP_HEIGHT - 1;

                        let x = (col as f64 - row as f64) * (TERRAIN_SPRITE_DIMENSION / 2.0)
                            + CANVAS_WIDTH / 2.0
                            - TERRAIN_SPRITE_DIMENSION / 2.0;
                        let y = (col as f64 + row as f64) * (TERRAIN_SPRITE_DIMENSION / 4.0);
                        draw_sprite(&ctx_clone, &closure_terrain_image, &GRASS, x, y).unwrap();

                        if is_perimeter {
                            draw_sprite(&ctx_clone, &closure_terrain_image, &FOREST_THICK, x, y)
                                .unwrap()
                        }
                        // sprites need to be drawn from the top rows down
                        draw_sprite(
                            &ctx_clone,
                            &closure_terrain_image,
                            &SHOP_TABLE_RIGHT,
                            TERRAIN_SPRITE_DIMENSION * 6.0,
                            TERRAIN_SPRITE_DIMENSION * 5.0,
                        )
                        .unwrap();
                        draw_sprite(
                            &ctx_clone,
                            &closure_terrain_image,
                            &STONE_ROOF_HOUSE,
                            TERRAIN_SPRITE_DIMENSION * 5.0,
                            TERRAIN_SPRITE_DIMENSION * 5.0,
                        )
                        .unwrap();
                        draw_sprite(
                            &ctx_clone,
                            &closure_terrain_image,
                            &STONE_ROOF_HOUSE,
                            TERRAIN_SPRITE_DIMENSION * 5.5,
                            TERRAIN_SPRITE_DIMENSION * 5.25,
                        )
                        .unwrap();
                        draw_sprite(
                            &ctx_clone,
                            &closure_monster_image,
                            &CASTER_RED_W_STAFF,
                            TERRAIN_SPRITE_DIMENSION * 7.5,
                            TERRAIN_SPRITE_DIMENSION * 5.0,
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

    view! { <canvas node_ref=canvas_ref width=CANVAS_WIDTH height=CANVAS_HEIGHT /> }
}

#[component]
fn SidePanelCharacterSheet() -> impl IntoView {
    view! {
        <aside class="w-64 bg-panel rounded shadow-lg text-sm overflow-y-auto max-h-[720px]">
            <div class="border-b border-gray-700 px-3 py-2 font-semibold text-base">Characters</div>
            <div class="divide-y divide-gray-700">
                <div class="px-3 py-2">
                    <div class="font-semibold">Warrior</div>
                    <div>HP: 120/120</div>
                    <div>ATK: 15 | DEF: 10</div>
                    <div>SPD: 8</div>
                </div>
                <div class="px-3 py-2">
                    <div class="font-semibold">Mage</div>
                    <div>HP: 80/80</div>
                    <div>ATK: 20 | DEF: 5</div>
                    <div>SPD: 12</div>
                </div>
            </div>
        </aside>
    }
}

#[component]
fn BottomPanel() -> impl IntoView {
    view! {
        <footer class="w-[calc(1280px+0.5rem+256px)] bg-panel rounded shadow-lg text-sm p-3 overflow-y-auto max-h-32">
            <div class="font-semibold mb-1">Game Log</div>
            <div class="space-y-1 text-xs">
                <p>[00:01] Warrior attacked Slime for 12</p>
                <p>[00:02] Mage cast Fireball on Slime for 25</p>
                <p>[00:03] Slime was defeated!</p>
                <p>[00:04] +10 Gold, +1 XP</p>
            </div>
        </footer>
    }
}
