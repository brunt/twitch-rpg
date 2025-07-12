use crate::components::draw_item_sprite;
use crate::sprites::items_sprites::ITEMS_SPRITES;
use crate::sprites::{ITEM_SPRITE_DIMENSION, SpriteRect};
use common::ShopItem;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

pub(crate) fn draw_shop_interface(
    ctx: &CanvasRenderingContext2d,
    item_image: &HtmlImageElement,
    items: &[ShopItem],
    start_x: f64,
    start_y: f64,
    slot_width: f64,
    slot_height: f64,
    slots_per_row: usize,
) {
    let padding = 12.0;
    for (i, item) in items.iter().enumerate() {
        let row = i / slots_per_row;
        let col = i % slots_per_row;
        let x = start_x + col as f64 * (slot_width + padding);
        let y = start_y + row as f64 * (slot_height + padding);

        // Draw slot rectangle
        ctx.set_fill_style_str("#222");
        ctx.fill_rect(x, y, slot_width, slot_height);

        // Draw item sprite centered in slot
        let sprite_x = x + (slot_width - ITEM_SPRITE_DIMENSION) / 2.0;
        let sprite_y = y + 8.0;
        draw_item_sprite(
            ctx,
            item_image,
            ITEMS_SPRITES.get(&item.sprite),
            sprite_x,
            sprite_y,
        );
        ctx.set_fill_style_str("#fff");
        ctx.line_to(100.0, 100.0);

        // Draw item name
        ctx.set_fill_style_str("#fff");
        ctx.set_font("bold 16px sans-serif");
        ctx.fill_text(&item.name, x + 8.0, y + slot_height - 32.0)
            .unwrap();

        // Draw price
        ctx.set_fill_style_str("#ffd700");
        ctx.set_font("14px sans-serif");
        ctx.fill_text(
            &format!("{} gold", item.price),
            x + 8.0,
            y + slot_height - 14.0,
        )
        .unwrap();

        // Draw description (shortened for slot)
        ctx.set_fill_style_str("#aaa");
        ctx.set_font("12px sans-serif");
        let desc = if item.description.len() > 32 {
            format!("{}...", &item.description[..29])
        } else {
            item.description.clone()
        };
        ctx.fill_text(&desc, x + 8.0, y + slot_height - 2.0)
            .unwrap();
    }
}
