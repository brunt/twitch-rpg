use crate::SpriteSheets;
use crate::components::{AnimationState, draw_sprite};
use crate::dungeon_floor::draw_dungeon_floor;
use crate::item_shop::draw_shop_interface;
use crate::sprites::SPRITE_DIMENSION;
use crate::sprites::spellfx_missiles_sprites::ActiveProjectile;
use common::GameSnapShot;
use leptos::context::use_context;
use leptos::html::Canvas;
use leptos::prelude::{Effect, Get, LocalStorage, NodeRef, NodeRefAttribute, ReadSignal, Signal};
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

    let sprites = use_context::<ReadSignal<SpriteSheets, LocalStorage>>()
        .expect("SpriteSheets context not found")
        .get();

    // Shared mutable state for the game logic. Rc provides multiple ownership,
    // and RefCell allows mutable borrowing at runtime.
    let latest_snapshot = Rc::new(RefCell::new(None));
    let animation_state = Rc::new(RefCell::new(AnimationState {
        frame_count: 0,
        last_time: 0.0,
    }));
    let active_projectiles: Rc<RefCell<Vec<ActiveProjectile>>> = Rc::new(RefCell::new(vec![]));

    // Rc<RefCell<Option<Closure>>> is used to store the animation frame closure,
    // allowing it to be called recursively by requestAnimationFrame.
    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let g = f.clone(); // 'g' is a clone of 'f', used to pass the closure reference into itself.

    // --- Effect 1: Handle GameSnapShot updates and add new projectiles ---
    // Clones of shared state for this specific Effect. These will be moved into the closure.
    let latest_snapshot_for_gs_effect = latest_snapshot.clone();
    let active_projectiles_for_gs_effect = active_projectiles.clone();
    let win_for_gs_effect = window().unwrap(); // Capture window once for this effect.

    Effect::new(move |_| {
        // This effect runs whenever 'gs' (GameSnapShot signal) changes.
        if let Some(snapshot) = gs.get() {
            // Update the latest snapshot.
            *latest_snapshot_for_gs_effect.borrow_mut() = Some(snapshot.clone());

            // Add new projectiles based on the snapshot.
            let Some(projectiles) = snapshot.projectiles.as_ref() else {
                return;
            };

            let now = win_for_gs_effect.performance().unwrap().now();

            for proj in projectiles {
                active_projectiles_for_gs_effect
                    .borrow_mut()
                    .push(ActiveProjectile::new(proj, now));
            }
        }
    });

    // --- Effect 2: Main Animation Loop ---
    // These are the variables that the *outer* Effect closure will capture by moving them.
    // They are cloned here once.
    let animation_state_outer = animation_state.clone();
    let latest_snapshot_outer = latest_snapshot.clone();
    let active_projectiles_outer = active_projectiles.clone();
    let f_outer = f.clone();
    let g_outer = g.clone();
    let win_outer = window().unwrap(); // Capture window once for this effect.
    let sprites_clone = sprites.clone();

    Effect::new(move |_| {
        // This effect runs when its dependencies change (e.g., canvas_ref becomes available).
        // It sets up and starts the animation loop.
        let Some(canvas) = canvas_ref.get() else {
            return; // Canvas not ready yet.
        };
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        // --- Clones for the *inner* animation frame closure ---
        // These clones are created *inside* the Effect closure.
        // This allows the outer Effect closure to implement FnMut, as it's not
        // moving its own captured variables into the inner closure, but new clones.
        let animation_state_for_closure = animation_state_outer.clone();
        let latest_snapshot_for_closure = latest_snapshot_outer.clone();
        let active_projectiles_for_closure = active_projectiles_outer.clone();
        let g_for_closure = g_outer.clone();
        let sprites_for_closure = sprites_clone.clone();

        let win_for_closure = win_outer.clone(); // window() is not Rc, so clone the value.

        // Create the animation frame closure. This closure will be called repeatedly by the browser.
        let closure = Closure::<dyn FnMut()>::new(move || {
            // Borrow mutable references to the shared state.
            let mut anim = animation_state_for_closure.borrow_mut();
            let mut projectiles = active_projectiles_for_closure.borrow_mut();

            anim.frame_count += 1;

            let now = win_for_closure.performance().unwrap().now();
            // Update last_time for the next frame's delta time calculation.
            anim.last_time = now;

            // Clear the canvas with black.
            ctx.set_fill_style_str("black");
            ctx.fill_rect(0.0, 0.0, CANVAS_WIDTH, CANVAS_HEIGHT);

            // Draw game elements based on the latest snapshot.
            if let Some(snapshot) = latest_snapshot_for_closure.borrow().as_ref() {
                // Get camera position from snapshot
                let camera_x = snapshot.camera_position.as_ref().map_or(0, |pos| pos.x);
                let camera_y = snapshot.camera_position.as_ref().map_or(0, |pos| pos.y);

                if let (Some(floor), Some(position), Some(difficulty), Some(positions)) = (
                    snapshot.floor.as_ref(),
                    snapshot.camera_position.as_ref(),
                    snapshot.difficulty,
                    snapshot.floor_positions.as_ref(),
                ) {
                    draw_dungeon_floor(
                        &ctx,
                        &sprites_for_closure.terrain,
                        &sprites_for_closure.monsters,
                        floor,
                        CANVAS_WIDTH,
                        CANVAS_HEIGHT,
                        (position.x, position.y),
                        difficulty,
                        positions,
                    );
                } else {
                    if let Some(shop_items) = snapshot.shop_items.as_ref() {
                        draw_shop_interface(&ctx, &sprites_for_closure, shop_items, 30.0, 30.0, 4)
                    };
                    // Draw ready timer or shop interface if not in dungeon.
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
                }

                // Animate and draw active projectiles.
                projectiles.retain(|p| {
                    let progress = (now - p.start_time) / 250.0; // 250ms duration
                    if progress >= 1.0 {
                        return false; // Remove projectile if animation is complete.
                    }

                    // Interpolate projectile position in world coordinates.
                    let interp = |a: u32, b: u32| a as f64 + (b as f64 - a as f64) * progress;
                    let proj_world_row = interp(p.from.0, p.to.0);
                    let proj_world_col = interp(p.from.1, p.to.1);

                    // --- Calculate projectile screen coordinates relative to world origin ---
                    let proj_screen_x_relative_to_origin =
                        (proj_world_col - proj_world_row) * (SPRITE_DIMENSION as f64 / 2.0);
                    let proj_screen_y_relative_to_origin =
                        (proj_world_col + proj_world_row) * (SPRITE_DIMENSION as f64 / 4.0);

                    // --- Calculate camera screen coordinates relative to world origin ---
                    // The camera's world position (camera_x, camera_y) should be used in the same isometric projection formula
                    // to find its screen offset from the world origin.
                    let camera_screen_x_relative_to_origin =
                        (camera_y as f64 - camera_x as f64) * (SPRITE_DIMENSION as f64 / 2.0);
                    let camera_screen_y_relative_to_origin =
                        (camera_y as f64 + camera_x as f64) * (SPRITE_DIMENSION as f64 / 4.0);

                    // --- Calculate final screen coordinates for the projectile ---
                    // Projectile's screen position - Camera's screen position + Canvas Center adjustments
                    let x = proj_screen_x_relative_to_origin - camera_screen_x_relative_to_origin
                        + CANVAS_WIDTH / 2.0
                        - SPRITE_DIMENSION as f64 / 2.0; // Adjust for sprite origin (usually top-left or center)
                    let y = proj_screen_y_relative_to_origin - camera_screen_y_relative_to_origin
                        + CANVAS_HEIGHT / 2.0
                        - SPRITE_DIMENSION as f64 / 2.0; // Adjust for sprite origin (usually top-left or center)

                    // --- IMPORTANT DEBUGGING OUTPUTS ---
                    leptos::logging::log!("Projectile draw X: {}", &x);
                    leptos::logging::log!("Projectile draw Y: {}", &y);
                    leptos::logging::log!("Projectile from: {:?}, to: {:?}", p.from, p.to);
                    leptos::logging::log!("Camera position: x={}, y={}", camera_x, camera_y);

                    // Draw the projectile sprite.
                    draw_sprite(
                        &ctx,
                        &sprites_for_closure.projectiles,
                        &p.sprite,
                        x,
                        y,
                        1.0,
                        None,
                    );
                    true // Keep projectile if animation is ongoing.
                });
            }

            // Request the next animation frame, recursively calling this closure.
            let cb = g_for_closure.borrow(); // Get a reference to the stored closure.
            let cb_ref = cb.as_ref().unwrap();
            win_for_closure
                .request_animation_frame(cb_ref.as_ref().unchecked_ref())
                .unwrap();
        });

        // Store the created closure in 'f' (which 'g' also points to).
        *f_outer.borrow_mut() = Some(closure);

        // Start the animation loop by requesting the first frame.
        win_outer
            .request_animation_frame(f_outer.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .unwrap();
    });

    view! { <canvas node_ref=canvas_ref width=CANVAS_WIDTH height=CANVAS_HEIGHT /> }
}
