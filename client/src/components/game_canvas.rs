use crate::components::{AnimationState, load_images};
use crate::dungeon_floor::draw_dungeon_floor;
use crate::item_shop::draw_shop_interface;
use common::GameSnapShot;
use leptos::html::Canvas;
use leptos::prelude::{Effect, Get, NodeRef, NodeRefAttribute, Signal};
use leptos::{IntoView, component, view};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{CanvasRenderingContext2d, window};

#[component]
pub fn GameCanvas(#[prop(into)] gs: Signal<Option<GameSnapShot>>) -> impl IntoView {
    let canvas_ref: NodeRef<Canvas> = NodeRef::new();
    const CANVAS_WIDTH: f64 = 1280.0;
    const CANVAS_HEIGHT: f64 = 720.0;

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
            leptos::logging::log!("{:?}", &latest_snapshot);
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

            let now = window().unwrap().performance().unwrap().now();
            let dt = now - anim.last_time;

            // backmost layer: black
            ctx.set_fill_style_str("black");
            ctx.fill_rect(0.0, 0.0, CANVAS_WIDTH, CANVAS_HEIGHT);

            if let Some(snapshot) = latest_snapshot.borrow().as_ref() {
                if let (Some(floor), Some(position), Some(difficulty), Some(positions)) = (
                    snapshot.floor.as_ref(),
                    snapshot.party_position.as_ref(),
                    snapshot.difficulty,
                    snapshot.floor_positions.as_ref(),
                ) {
                    draw_dungeon_floor(
                        &ctx,
                        &closure_terrain_image,
                        &closure_monster_image,
                        &floor,
                        CANVAS_WIDTH,
                        CANVAS_HEIGHT,
                        (position.y, position.x),
                        difficulty,
                        positions,
                    );
                } else {
                    if let Some(ready_timer) = &snapshot.ready_timer {
                        ctx.set_font("bold 16px sans-serif");
                        ctx.set_fill_style_str("white");
                        ctx.fill_text(
                            format!("Entering Dungeon in {} seconds", ready_timer.remaining)
                                .to_string()
                                .as_str(),
                            CANVAS_WIDTH * 0.45,
                            20.0,
                        )
                        .expect("failed to count down");
                    }
                    if let Some(shop_items) = snapshot.shop_items.as_ref() {
                        draw_shop_interface(&ctx, &closure_item_image, &shop_items, 30.0, 30.0, 4)
                    };
                }
            }
            let window = window().unwrap();
            let cb = g.borrow();
            let cb_ref = cb.as_ref().unwrap();
            window
                .request_animation_frame(cb_ref.as_ref().unchecked_ref())
                .unwrap();
        });

        *f.borrow_mut() = Some(closure);

        window()
            .unwrap()
            .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .unwrap();
    });

    view! { <canvas node_ref=canvas_ref width=CANVAS_WIDTH height=CANVAS_HEIGHT /> }
}
