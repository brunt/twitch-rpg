use crate::SpriteSheets;
use crate::components::{AnimationState, draw_sprite};
use crate::dungeon_floor::draw_dungeon_floor;
use crate::item_shop::draw_shop_interface;
use crate::sprites::SPRITE_DIMENSION;
use crate::sprites::spellfx_missiles_sprites::ActiveProjectile;
use common::{Form, GameSnapShot, Health};
use leptos::context::use_context;
use leptos::html::Canvas;
use leptos::prelude::{Effect, Get, LocalStorage, NodeRef, NodeRefAttribute, ReadSignal, Signal};
use leptos::{IntoView, component, view};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{CanvasRenderingContext2d, window};

#[derive(Clone)]
pub struct RenderEntity {
    pub render_pos: (f64, f64), // Smooth interpolated position
    pub target_pos: (f64, f64), // Latest target position from server
    pub entity_type: String,
    pub level: u32,
    pub form: Form,
    pub health: Option<Health>,
}

#[derive(Clone)]
pub struct CameraState {
    pub render_pos: (f64, f64), // Smooth interpolated camera position
    pub target_pos: (f64, f64), // Latest target camera position from server
}

impl CameraState {
    fn new() -> Self {
        Self {
            render_pos: (0.0, 0.0),
            target_pos: (0.0, 0.0),
        }
    }
}

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
    let entity_render_map: Rc<RefCell<HashMap<u32, RenderEntity>>> =
        Rc::new(RefCell::new(HashMap::new()));
    let active_projectiles: Rc<RefCell<Vec<ActiveProjectile>>> = Rc::new(RefCell::new(vec![]));

    // Camera state for smooth interpolation
    let camera_state: Rc<RefCell<CameraState>> = Rc::new(RefCell::new(CameraState::new()));

    // Track if the player is in the dungeon to clear entities on entry.
    let is_in_dungeon = Rc::new(RefCell::new(false));

    // Rc<RefCell<Option<Closure>>> is used to store the animation frame closure,
    // allowing it to be called recursively by requestAnimationFrame.
    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();

    // --- Effect 1: Handle GameSnapShot updates and add new projectiles ---
    // Clones of shared state for this specific Effect. These will be moved into the closure.
    let latest_snapshot_for_gs_effect = latest_snapshot.clone();
    let active_projectiles_for_gs_effect = active_projectiles.clone();
    let entity_render_map_for_gs_effect = entity_render_map.clone();
    let camera_state_for_gs_effect = camera_state.clone();
    let is_in_dungeon_for_gs_effect = is_in_dungeon.clone();
    let win_for_gs_effect = window().unwrap();

    Effect::new(move |_| {
        // This effect runs whenever 'gs' (GameSnapShot signal) changes.
        if let Some(snapshot) = gs.get() {
            // Update the latest snapshot.
            *latest_snapshot_for_gs_effect.borrow_mut() = Some(snapshot.clone());

            // --- Dungeon Transition Logic ---
            let currently_in_dungeon = snapshot.floor.is_some();
            let mut previously_in_dungeon = is_in_dungeon_for_gs_effect.borrow_mut();

            // If we just entered the dungeon, clear the entity map to prevent
            // entities from "flying in" from their old positions.
            if currently_in_dungeon && !*previously_in_dungeon {
                entity_render_map_for_gs_effect.borrow_mut().clear();
            }

            *previously_in_dungeon = currently_in_dungeon;
            // --- End Dungeon Transition Logic ---

            // Update camera target position
            if let Some(camera_pos) = snapshot.camera_position.as_ref() {
                let mut camera = camera_state_for_gs_effect.borrow_mut();
                let new_target = (camera_pos.x as f64, camera_pos.y as f64);

                // If this is the first time setting camera position, snap to target
                if camera.render_pos == (0.0, 0.0) && camera.target_pos == (0.0, 0.0) {
                    camera.render_pos = new_target;
                }

                camera.target_pos = new_target;
            }

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

            // Update entity targets
            if let Some(positions) = snapshot.floor_positions.as_ref() {
                let mut render_map = entity_render_map_for_gs_effect.borrow_mut();
                for ep in positions {
                    let target = (ep.position.x as f64, ep.position.y as f64);
                    render_map
                        .entry(ep.id)
                        .and_modify(|re| {
                            re.target_pos = target;
                            re.entity_type = ep.entity_type.clone();
                            re.health = ep.health.clone();
                            re.form = ep.form.clone();
                        })
                        .or_insert_with(|| RenderEntity {
                            render_pos: target,
                            target_pos: target,
                            entity_type: ep.entity_type.clone(),
                            level: ep.level,
                            form: ep.form.clone(),
                            health: ep.health.clone(),
                        });
                }
            }
        }
    });

    // --- Effect 2: Main Animation Loop ---
    // These are the variables that the *outer* Effect closure will capture by moving them.
    // They are cloned here once.
    let animation_state_outer = animation_state.clone();
    let latest_snapshot_outer = latest_snapshot.clone();
    let active_projectiles_outer = active_projectiles.clone();
    let entity_render_map_outer = entity_render_map.clone();
    let camera_state_outer = camera_state.clone();
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
        let entity_render_map_for_closure = entity_render_map_outer.clone();
        let active_projectiles_for_closure = active_projectiles_outer.clone();
        let camera_state_for_closure = camera_state_outer.clone();
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
            let dt = (now - anim.last_time) / 1000.0; // Delta time in seconds

            // Update last_time for the next frame's delta time calculation.
            anim.last_time = now;

            // Smooth camera interpolation
            let (camera_x, camera_y) = {
                let mut camera = camera_state_for_closure.borrow_mut();

                // Camera movement speed (tiles per second)
                const CAMERA_SPEED: f64 = 2.5;

                let dx = camera.target_pos.0 - camera.render_pos.0;
                let dy = camera.target_pos.1 - camera.render_pos.1;
                let distance_squared = dx * dx + dy * dy;

                if distance_squared > 0.0001 {
                    let distance = distance_squared.sqrt();
                    let max_move_dist = CAMERA_SPEED * dt;
                    let move_dist = distance.min(max_move_dist);

                    // Move towards target
                    camera.render_pos.0 += (dx / distance) * move_dist;
                    camera.render_pos.1 += (dy / distance) * move_dist;
                } else {
                    // Snap to target if very close
                    camera.render_pos = camera.target_pos;
                }

                (camera.render_pos.0, camera.render_pos.1)
            };

            // Clear the canvas with black.
            ctx.set_fill_style_str("black");
            ctx.fill_rect(0.0, 0.0, CANVAS_WIDTH, CANVAS_HEIGHT);

            // Draw game elements based on the latest snapshot.
            if let Some(snapshot) = latest_snapshot_for_closure.borrow().as_ref() {
                if let (Some(floor), Some(_cam_pos), Some(difficulty)) = (
                    snapshot.floor.as_ref(),
                    snapshot.camera_position.as_ref(),
                    snapshot.difficulty,
                ) {
                    // Smooth entity movement
                    {
                        let mut render_map = entity_render_map_for_closure.borrow_mut();
                        for re in render_map.values_mut() {
                            // Move at a constant speed for a more direct, less "floaty" feel.
                            // Speed is in world units (tiles) per second. This can be tuned.
                            const MOVEMENT_SPEED: f64 = 2.5;

                            let dx = re.target_pos.0 - re.render_pos.0;
                            let dy = re.target_pos.1 - re.render_pos.1;
                            let distance_squared = dx * dx + dy * dy;

                            // Avoid sqrt and division by zero if already at the target.
                            if distance_squared > 0.0001 {
                                let distance = distance_squared.sqrt();
                                let max_move_dist = MOVEMENT_SPEED * dt;
                                let move_dist = distance.min(max_move_dist);

                                // Move towards target.
                                re.render_pos.0 += (dx / distance) * move_dist;
                                re.render_pos.1 += (dy / distance) * move_dist;
                            } else {
                                // Snap to target if very close.
                                re.render_pos = re.target_pos;
                            }
                        }
                    }

                    draw_dungeon_floor(
                        &ctx,
                        &sprites_for_closure.terrain,
                        &sprites_for_closure.monsters,
                        floor,
                        CANVAS_WIDTH,
                        CANVAS_HEIGHT,
                        (camera_x, camera_y), // Use interpolated camera position
                        difficulty,
                        &entity_render_map_for_closure.borrow(),
                    );

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
                            (camera_y - camera_x) * (SPRITE_DIMENSION as f64 / 2.0);
                        let camera_screen_y_relative_to_origin =
                            (camera_y + camera_x) * (SPRITE_DIMENSION as f64 / 4.0);

                        // --- Calculate final screen coordinates for the projectile ---
                        // Projectile's screen position - Camera's screen position + Canvas Center adjustments
                        let x = proj_screen_x_relative_to_origin
                            - camera_screen_x_relative_to_origin
                            + CANVAS_WIDTH / 2.0
                            - SPRITE_DIMENSION as f64 / 2.0; // Adjust for sprite origin (usually top-left or center)
                        let y = proj_screen_y_relative_to_origin
                            - camera_screen_y_relative_to_origin
                            + CANVAS_HEIGHT / 2.0
                            - SPRITE_DIMENSION as f64 / 2.0; // Adjust for sprite origin (usually top-left or center)

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
                } else {
                    if let Some(shop_items) = snapshot.shop_items.as_ref() {
                        draw_shop_interface(&ctx, &sprites_for_closure, shop_items, 15.0, 15.0, 4)
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
