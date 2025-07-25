use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::time::Duration;
use crate::components::{draw_item_sprite, draw_sprite};
use crate::sprites::items_sprites::ITEMS_SPRITES;
use crate::sprites::monsters_sprites::player_sprite;
use crate::sprites::{ITEM_SPRITE_DIMENSION, SPRITE_DIMENSION, SpriteRect};
use common::{GameSnapShot, PlayerSnapshot};
use leptos::html::{Canvas, canvas};
use leptos::prelude::{set_timeout, signal, ClassAttribute, Get, IntoAny, Set, Signal, StyleAttribute, Update};
use leptos::prelude::{Effect, ElementChild, NodeRef, NodeRefAttribute};
use leptos::{IntoView, component, view};
use leptos::control_flow::Show;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

//TODO: display active buffs
#[component]
pub fn SidePanelCharacterSheet(#[prop(into)] gs: Signal<Option<GameSnapShot>>) -> impl IntoView {
    let (show_player_stats, set_show_player_stats) = signal::<HashSet<usize>>(HashSet::new());
    let timers: Rc<RefCell<HashMap<usize, Duration>>> = Rc::new(RefCell::new(HashMap::new()));

    Effect::new({
        let set_show_player_stats = set_show_player_stats.clone();
        let show_player_stats = show_player_stats.clone();
        let timers = timers.clone();

        move |_| {
            if let Some(gs) = gs.get() {
                for (i, player) in gs.party.iter().enumerate() {
                    if player.show {
                        let mut current = show_player_stats.get();
                        if current.insert(i) {
                            set_show_player_stats.set(current.clone());
                        }

                        let set_show_player_stats = set_show_player_stats.clone();
                        let timers = timers.clone();
                        set_timeout(move || {
                            timers.borrow_mut().remove(&i);
                            let mut current = show_player_stats.get();
                            if current.remove(&i) {
                                set_show_player_stats.set(current.clone());
                            }
                        }, Duration::from_secs(10));
                    }
                }
            }
        }
    });
    
    
    view! {
        <aside class="w-[42rem] bg-panel shadow-lg text-sm overflow-y-auto max-h-[720px]">
            <div class="border-gray-100 px-3 py-2 font-semibold text-base">Characters</div>
            <div class="flex gap-1 px-1">
                <div class="flex flex-col gap-1 w-1/2">
                    {move || {
                        let Some(gs) = gs.get() else { return vec![] };
                        gs.party
                            .iter()
                            .enumerate()
                            .filter(|(i, _)| i % 2 == 0)
                            .map(|(i, player)| {
                                let is_expanded = show_player_stats.get().contains(&i);
                                view! {
                                    <PlayerPanel player=player.clone() is_expanded=is_expanded />
                                }
                            })
                            .collect::<Vec<_>>()
                    }}
                </div>
                <div class="flex flex-col gap-1 w-1/2">
                    {move || {
                        let Some(gs) = gs.get() else { return vec![] };
                        gs.party
                            .iter()
                            .enumerate()
                            .filter(|(i, _)| i % 2 == 1)
                            .map(|(i, player)| {
                                let is_expanded = show_player_stats.get().contains(&i);
                                view! {
                                    <PlayerPanel player=player.clone() is_expanded=is_expanded />
                                }
                            })
                            .collect::<Vec<_>>()
                    }}
                </div>
            </div>
        </aside>
    }
}

#[component]
fn PlayerPanel(#[prop(into)] player: PlayerSnapshot, #[prop(into)] is_expanded: bool) -> impl IntoView {
    let (height_style, set_height_style) = signal(String::from("0px"));

    Effect::new(move |_| {
        if is_expanded {
            set_height_style.set("106px".to_string());
        } else {
            set_height_style.set("0px".to_string());
        }
    });

    view! {
        <div class="relative p-2 bg-stone-900 border border-amber-300 transition-opacity">
            <div class="flex items-center gap-2">
                <PlayerSpriteCanvas sprite=player_sprite((
                    &player.form,
                    &player.class,
                    player.level,
                )) />
                <div class="flex flex-col grow">
                    <div class="font-semibold text-lg text-amber-500">{player.name.clone()}</div>

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
            <div style=move || {
                format!(
                    "height: {}; overflow: hidden; transition: height 0.3s ease;",
                    height_style.get(),
                )
            }>
                <Show when=move || is_expanded fallback=move || view! {}.into_any()>
                    <ExtraStats player=player.clone() />
                </Show>
            </div>
        </div>
    }
}



#[component]
fn ExtraStats(#[prop(into)] player: PlayerSnapshot) -> impl IntoView {
    view! {
        <div class="px-2 pb-2">
            <p class="font-semibold text-sm text-amber-200">EXP: 12</p>
            <p class="font-semibold text-sm text-red-500">Strength: {player.stats.strength}</p>
            <p class="font-semibold text-sm text-green-500">Agility: {player.stats.agility}</p>
            <p class="font-semibold text-sm text-blue-500">
                Intelligence: {player.stats.intelligence}
            </p>
        </div>
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
