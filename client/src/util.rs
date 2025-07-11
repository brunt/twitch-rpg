use crate::sprites::{ITEM_SPRITE_DIMENSION, SPRITE_DIMENSION, SpriteRect};
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

pub fn load_images() -> [HtmlImageElement; 10] {
    let terrain_image = HtmlImageElement::new().unwrap();
    let monster_image = HtmlImageElement::new().unwrap();
    let item_image = HtmlImageElement::new().unwrap();
    let spellfx_anim_1_image = HtmlImageElement::new().unwrap();
    let spellfx_anim_2_image = HtmlImageElement::new().unwrap();
    let spellfx_anim_3_image = HtmlImageElement::new().unwrap();
    let spellfx_anim_4_image = HtmlImageElement::new().unwrap();
    let spellfx_anim_5_image = HtmlImageElement::new().unwrap();
    let damage_fx_sprite_image = HtmlImageElement::new().unwrap();
    let missiles_image = HtmlImageElement::new().unwrap();

    terrain_image.set_src("public/sprites/terrain.png");
    item_image.set_src("public/sprites/items.png");
    monster_image.set_src("public/sprites/monsters.png");
    spellfx_anim_1_image.set_src("public/sprites/SpellFXAnim1.png");
    spellfx_anim_2_image.set_src("public/sprites/SpellFXAnim2.png");
    spellfx_anim_3_image.set_src("public/sprites/SpellFXAnim3.png");
    spellfx_anim_4_image.set_src("public/sprites/SpellFXAnim4.png");
    spellfx_anim_5_image.set_src("public/sprites/SpellFXAnim5.png");
    damage_fx_sprite_image.set_src("public/sprites/DamageFX.png");
    missiles_image.set_src("public/sprites/SpellFXMissiles.png");

    [
        terrain_image,
        monster_image,
        item_image,
        spellfx_anim_1_image,
        spellfx_anim_2_image,
        spellfx_anim_3_image,
        spellfx_anim_4_image,
        spellfx_anim_5_image,
        damage_fx_sprite_image,
        missiles_image,
    ]
}

pub fn draw_sprite(
    ctx: &CanvasRenderingContext2d,
    image: &HtmlImageElement,
    sprite: &SpriteRect,
    x: f64,
    y: f64,
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
        SPRITE_DIMENSION,
        SPRITE_DIMENSION,
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
        ITEM_SPRITE_DIMENSION,
        ITEM_SPRITE_DIMENSION,
    )
    .expect_throw("Failed to draw sprite");
}

pub struct AnimationState {
    pub frame_count: u64,
    pub last_time: f64,
}
