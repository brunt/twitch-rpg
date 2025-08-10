use crate::sprites::{
    ITEM_SPRITE_DIMENSION, PROJECTILE_SPRITE_DIMENSION, SPRITE_DIMENSION, SpriteRect,
};
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

pub mod bottom_panel;
pub mod game_canvas;
pub mod side_panel;

pub fn draw_sprite(
    ctx: &CanvasRenderingContext2d,
    image: &HtmlImageElement,
    sprite: &SpriteRect,
    x: f64,
    y: f64,
    scale: f64,
    opacity: Option<f64>,
) {
    if let Some(o) = opacity {
        ctx.set_global_alpha(o);
    }
    ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        image,
        sprite.x,
        sprite.y,
        sprite.w,
        sprite.h,
        x,
        y,
        SPRITE_DIMENSION as f64 * scale,
        SPRITE_DIMENSION as f64 * scale,
    )
    .expect_throw("Failed to draw sprite");
    if opacity.is_some() {
        ctx.set_global_alpha(1.0);
    }
}

pub fn draw_item_sprite(
    ctx: &CanvasRenderingContext2d,
    image: &HtmlImageElement,
    sprite: Option<&SpriteRect>,
    x: f64,
    y: f64,
) {
    let Some(sprite) = sprite else { return };
    ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        image,
        sprite.x,
        sprite.y,
        sprite.w,
        sprite.h,
        x,
        y,
        ITEM_SPRITE_DIMENSION as f64,
        ITEM_SPRITE_DIMENSION as f64,
    )
    .expect_throw("Failed to draw sprite");
}

pub fn draw_proj_sprite(
    ctx: &CanvasRenderingContext2d,
    image: &HtmlImageElement,
    sprite: &SpriteRect,
    x: f64,
    y: f64,
) {
    ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        image,
        sprite.x,
        sprite.y,
        sprite.w,
        sprite.h,
        x,
        y,
        PROJECTILE_SPRITE_DIMENSION as f64,
        PROJECTILE_SPRITE_DIMENSION as f64,
    )
    .expect_throw("Failed to draw sprite");
}

pub struct AnimationState {
    pub frame_count: u64,
    pub last_time: f64,
}
