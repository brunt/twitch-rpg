use std::ops::Deref;
use leptos::html::Canvas;
use leptos::mount::mount_to_body;
use leptos::prelude::{Effect, ElementChild, Get, NodeRef, NodeRefAttribute, request_animation_frame, ClassAttribute};
use leptos::{IntoView, component, view};
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};
use wasm_bindgen::closure::Closure;
use web_sys::{CanvasGradient, CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};

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

#[component]
fn GameCanvas() -> impl IntoView {
    let canvas_ref: NodeRef<Canvas> = NodeRef::new();

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
            let image = HtmlImageElement::new().unwrap();
            let closure_image = image.clone();
            let ctx_clone = ctx.clone(); // Move into closure

            let onload = Closure::<dyn FnMut()>::new(move || {
                //TODO: get sprite coords for useful tiles, draw stuff
                // loop to paint this tile repeatedly on the canvas
                // for row in 0..rows {
                // for col in 0..cols {
                // ..
                
                // draw a sprite from the sprite sheet
                // (sx, sy, sw, sh): source rect in sprite sheet
                // (dx, dy, dw, dh): destination rect on canvas
                let sx = 320.0; // source x in sprite sheet
                let sy = 320.0; // source y in sprite sheet
                let sw = 65.0; // sprite width
                let sh = 65.0; // sprite height
                let dx = 120.0; // canvas x
                let dy = 120.0; // canvas y
                let dw = 100.0; // draw width
                let dh = 100.0; // draw height

                ctx_clone
                    .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                        &closure_image, sx, sy, sw, sh, dx, dy, dw, dh,
                    )
                    .unwrap();
            });

            image.set_onload(Some(onload.as_ref().unchecked_ref()));
            image.set_src("public/sprites/terrain.png");
            onload.forget();
            
        }
    });

    view! { <canvas node_ref=canvas_ref width=1280 height=720 /> }
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