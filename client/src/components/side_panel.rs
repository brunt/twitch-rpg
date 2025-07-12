use crate::components::draw_sprite;
use crate::sprites::monsters_sprites::WIZARD_SPRITES;
use crate::sprites::{SPRITE_DIMENSION, SpriteRect};
use common::GameSnapShot;
use leptos::html::Canvas;
use leptos::prelude::{ClassAttribute, Get, Signal};
use leptos::prelude::{Effect, ElementChild, NodeRef, NodeRefAttribute};
use leptos::{IntoView, component, view};
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

#[component]
pub fn SidePanelCharacterSheet(#[prop(into)] gs: Signal<Option<GameSnapShot>>) -> impl IntoView {
    view! {
        <aside class="w-80 bg-panel rounded shadow-lg text-sm overflow-y-auto max-h-[720px]">
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
