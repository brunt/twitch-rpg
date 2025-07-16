use crate::components::{draw_item_sprite, draw_sprite};
use crate::sprites::items_sprites::ITEMS_SPRITES;
use crate::sprites::monsters_sprites::player_sprite;
use crate::sprites::{ITEM_SPRITE_DIMENSION, SPRITE_DIMENSION, SpriteRect};
use common::GameSnapShot;
use leptos::html::{Canvas, canvas};
use leptos::prelude::{ClassAttribute, Get, Signal};
use leptos::prelude::{Effect, ElementChild, NodeRef, NodeRefAttribute};
use leptos::{IntoView, component, view};
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

//TODO: display active buffs
#[component]
pub fn SidePanelCharacterSheet(#[prop(into)] gs: Signal<Option<GameSnapShot>>) -> impl IntoView {
    view! {
        <aside class="w-[42rem] bg-panel shadow-lg text-sm overflow-y-auto max-h-[720px]">
            <div class="border-gray-100 px-3 py-2 font-semibold text-base">Characters</div>
            <div class="grid grid-cols-2 auto-rows-min gap-1 px-1">
                {move || {
                    let Some(gs) = gs.get() else { return vec![] };
                    gs.party
                        .iter()
                        .map(|player| {
                            view! {
                                <div class="flex items-center gap-2 p-2 bg-stone-900 border border-amber-300">
                                    <PlayerSpriteCanvas sprite=player_sprite((
                                        &player.form,
                                        &player.class,
                                        player.level,
                                    )) />
                                    <div class="flex flex-col grow">
                                        <div class="font-semibold text-lg text-amber-500">
                                            {player.name.clone()}
                                        </div>

                                        <div class="text-sm flex items-center gap-2 font-semibold">
                                            {format!("level {} {}", player.level, player.class.clone())}
                                            <div class="flex items-center gap-1 font-semibold">
                                                <GoldCoinCanvas />
                                                {player.gold}
                                            </div>
                                        </div>
                                        <div class="text-sm">{player.health.to_string()}</div>
                                    </div>
                                </div>
                            }
                        })
                        .collect::<Vec<_>>()
                }}
            </div>
        </aside>
    }
}

#[component]
fn PlayerSpriteCanvas(#[prop(into)] sprite: &'static SpriteRect) -> impl IntoView {
    const SCALE: f64 = 1.5;
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
        // Create and load the image
        let monster_image = HtmlImageElement::new().unwrap();
        let closure_monster_image = monster_image.clone();
        let ctx_clone = ctx.clone();
        let onload = Closure::<dyn FnMut()>::new(move || {
            draw_sprite(
                &ctx_clone,
                &closure_monster_image,
                &sprite,
                0.0,
                0.0,
                SCALE,
                None,
            );
        });
        monster_image.set_onload(Some(onload.as_ref().unchecked_ref()));
        monster_image.set_src("public/sprites/monsters.png");
        onload.forget();
    });

    view! {
        <canvas
            node_ref=canvas_ref
            width=SPRITE_DIMENSION * SCALE
            height=SPRITE_DIMENSION * SCALE
        />
    }
}

#[component]
fn GoldCoinCanvas() -> impl IntoView {
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
            draw_item_sprite(
                &ctx,
                &closure_coin_image,
                ITEMS_SPRITES.get("small_gold_pile"),
                0.0,
                0.0,
            )
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
