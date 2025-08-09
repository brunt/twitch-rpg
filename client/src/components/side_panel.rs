use crate::components::{draw_item_sprite, draw_sprite};
use crate::sprites::items_sprites::ITEMS_SPRITES;
use crate::sprites::monsters_sprites::player_sprite;
use crate::sprites::{ITEM_SPRITE_DIMENSION, SPRITE_DIMENSION, SpriteRect};
use common::{EquipmentSlot, GameSnapShot, Item, ItemQuality, PlayerSnapshot};
use leptos::control_flow::Show;
use leptos::html::Canvas;
use leptos::prelude::{
    ClassAttribute, Get, IntoAny, Set, Signal, StyleAttribute, set_timeout, signal,
};
use leptos::prelude::{Effect, ElementChild, NodeRef, NodeRefAttribute};
use leptos::{IntoView, component, view};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::time::Duration;
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
                        set_timeout(
                            move || {
                                timers.borrow_mut().remove(&i);
                                let mut current = show_player_stats.get();
                                if current.remove(&i) {
                                    set_show_player_stats.set(current.clone());
                                }
                            },
                            Duration::from_secs(10),
                        );
                    }
                }
            }
        }
    });

    view! {
        <aside class="w-[42rem] bg-panel shadow-lg text-sm overflow-y-auto">
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
                                let is_expanded = show_player_stats.get().contains(&i) || gs.party.len() < 5;
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
                                let is_expanded = show_player_stats.get().contains(&i) || gs.party.len() < 5;
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
fn PlayerPanel(
    #[prop(into)] player: PlayerSnapshot,
    #[prop(into)] is_expanded: bool,
) -> impl IntoView {
    let (height_style, set_height_style) = signal(String::from("0px"));

    Effect::new(move |_| {
        if is_expanded {
            set_height_style.set("175px".to_string());
        } else {
            set_height_style.set("0px".to_string());
        }
    });

    let utility_item = player
        .equipped_items
        .iter()
        .find(|(slot, _)| matches!(slot, EquipmentSlot::UtilitySlot))
        .map(|(_, item)| item);

    view! {
        <div class="relative p-2 bg-stone-900 border border-amber-300 transition-opacity">
            <div class="flex items-center gap-2">
                <div class="flex flex-col items-center">
                    <PlayerSpriteCanvas sprite=player_sprite((
                        &player.form,
                        &player.class,
                        player.level,
                    )) />
                    <div class="h-[38px] mt-1 flex items-center justify-center">
                        {utility_item
                            .map(|item| view! { <ItemSpriteCanvas item=item.clone() /> }.into_any())
                            .unwrap_or_else(|| {
                                view! {
                                    <div
                                        class="inline-block align-middle"
                                        style=format!(
                                            "width:{}px; height:{}px;",
                                            ITEM_SPRITE_DIMENSION + 2,
                                            ITEM_SPRITE_DIMENSION + 2,
                                        )
                                    ></div>
                                }
                                    .into_any()
                            })}
                    </div>
                </div>
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
            <div class="flex flex-row justify-between gap-4">
                <div class="grid grid-cols-2 gap-1">
                    {move || {
                        let mut equipped: Vec<_> = player
                            .equipped_items
                            .iter()
                            .filter(|(slot, _)| !matches!(slot, EquipmentSlot::UtilitySlot))
                            .collect();
                        equipped.sort_by_key(|(slot, _)| slot.slot_order());

                        equipped.into_iter()
                        .map(|(slot, item)| {
                                view! {
                                    <div class="flex items-center gap-2">
                                        <ItemSpriteCanvas item=item.clone() />
                                        // <p>{item.name.clone()}</p>
                                    </div>
                                }
                            })
                            .collect::<Vec<_>>()
                    }}
                </div>
                <div class="w-36">
                    <div class="flex flex-col gap-1">
                        <div class="flex justify-between font-semibold text-sm text-rose-400">
                            <span>Strength</span>
                            <span>{player.stats.strength}</span>
                        </div>
                        <div class="flex justify-between font-semibold text-sm text-green-500">
                            <span>Agility</span>
                            <span>{player.stats.agility}</span>
                        </div>
                        <div class="flex justify-between font-semibold text-sm text-blue-500">
                            <span>Intelligence</span>
                            <span>{player.stats.intelligence}</span>
                        </div>
                    </div>
                </div>
            </div>
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
            width=SPRITE_DIMENSION as f64 * SCALE
            height=SPRITE_DIMENSION as f64 * SCALE
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
            draw_item_sprite(&ctx, &closure_coin_image, ITEMS_SPRITES.get(&138), 0.0, 0.0)
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

#[component]
fn ItemSpriteCanvas(#[prop(into)] item: Item) -> impl IntoView {
    let canvas_ref: NodeRef<Canvas> = NodeRef::new();

    let item_rc_outer = Rc::new(item);
    Effect::new(move |_| {
        let Some(canvas) = canvas_ref.get() else {
            return;
        };
        let item_rc = Rc::clone(&item_rc_outer);

        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        let ctx_rc = Rc::new(RefCell::new(ctx));
        let item_image_rc = Rc::new(RefCell::new(HtmlImageElement::new().unwrap()));

        let closure_item_image_rc = Rc::clone(&item_image_rc);
        let closure_ctx_rc = Rc::clone(&ctx_rc);
        let closure_item_rc = item_rc.clone();

        let onload = Closure::<dyn FnMut()>::new(move || {
            let item = &*closure_item_rc;
            let item_image = closure_item_image_rc.borrow();
            let ctx = closure_ctx_rc.borrow();
            ctx.set_stroke_style_str(match item.quality.clone() {
                ItemQuality::Common => "#ddd",
                ItemQuality::Uncommon => "#af0",
                ItemQuality::Rare => "#0af",
                ItemQuality::Epic => "#c3f",
                ItemQuality::Legendary => "#f90",
            });
            ctx.set_line_width(1.0);
            let frame_x = 1.0;
            let frame_y = 1.0;
            let frame_w = ITEM_SPRITE_DIMENSION as f64;
            let frame_h = ITEM_SPRITE_DIMENSION as f64;
            ctx.stroke_rect(frame_x, frame_y, frame_w, frame_h);
            draw_item_sprite(
                &ctx,
                &item_image,
                ITEMS_SPRITES.get(&item.item_id),
                0.0,
                0.0,
            )
        });

        let item_image_mut = item_image_rc.borrow_mut();
        item_image_mut.set_onload(Some(onload.as_ref().unchecked_ref()));
        item_image_mut.set_src("public/sprites/items.png");
        onload.forget();
    });

    view! {
        <canvas
            node_ref=canvas_ref
            width=ITEM_SPRITE_DIMENSION as f64 + 2.0
            height=ITEM_SPRITE_DIMENSION as f64 + 2.0
            class="inline-block align-middle"
        />
    }
}
