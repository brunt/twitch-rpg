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
    slots_per_row: usize,
) {
    const ITEM_SLOT_SIZE: f64 = 150.0;

    let padding = 12.0;
    for (i, item) in items.iter().enumerate() {
        let row = i / slots_per_row;
        let col = i % slots_per_row;
        let x = start_x + col as f64 * (ITEM_SLOT_SIZE + padding);
        let y = start_y + row as f64 * (ITEM_SLOT_SIZE + padding);

        // Draw slot rectangle
        ctx.set_fill_style_str("#222");
        ctx.fill_rect(x, y, ITEM_SLOT_SIZE, ITEM_SLOT_SIZE);
        // Draw item sprite centered in slot
        let sprite_x = x + (ITEM_SLOT_SIZE - ITEM_SPRITE_DIMENSION) * 0.20;
        let sprite_y = y + 20.0;
        draw_item_sprite(
            ctx,
            item_image,
            ITEMS_SPRITES.get(&item.sprite),
            sprite_x,
            sprite_y,
        );
        ctx.set_fill_style_str("#fff");
        ctx.line_to(100.0, 100.0);

        // draw frame around sprite
        ctx.set_stroke_style_str("#af0");
        ctx.set_line_width(1.0);
        let frame_x = sprite_x - 1.0;
        let frame_y = sprite_y - 1.0;
        let frame_w = ITEM_SPRITE_DIMENSION + 30.0;
        let frame_h = ITEM_SPRITE_DIMENSION + 30.0;
        ctx.stroke_rect(frame_x - 8.0, frame_y - 8.0, frame_w - 10.0, frame_h - 10.0);

        // Draw item name
        ctx.set_fill_style_str("#fff");
        ctx.set_font("bold 16px sans-serif");
        ctx.fill_text(&item.name, x + 8.0, y + ITEM_SLOT_SIZE - 60.0)
            .unwrap();

        // Draw price
        ctx.set_fill_style_str("#ffd700");
        ctx.set_font("14px sans-serif");
        ctx.fill_text(
            &format!("{}", item.price),
            x + (ITEM_SLOT_SIZE - ITEM_SPRITE_DIMENSION) * 0.85,
            y + ITEM_SLOT_SIZE - 103.0,
        )
        .unwrap();

        // Draw gold sprite
        draw_item_sprite(ctx, item_image, Some(ITEMS_SPRITES.get("small_gold_pile").unwrap()), x + 70.0, y + 25.0);


        // Draw description (shortened for slot)
        ctx.set_fill_style_str("#aaa");
        ctx.set_font("12px sans-serif");
        draw_wrapped_text(ctx, &item.description, x + 8.0, y + ITEM_SLOT_SIZE - 45.0, ITEM_SLOT_SIZE - 16.0, 14.0);
    }
}

fn draw_wrapped_text(
    ctx: &CanvasRenderingContext2d,
    text: &str,
    x: f64,
    mut y: f64,
    max_width: f64,
    line_height: f64,
) {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut line = String::new();

    for word in words {
        let test_line = if line.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", line, word)
        };
        let metrics = ctx.measure_text(&test_line).unwrap();
        if metrics.width() > max_width && !line.is_empty() {
            ctx.fill_text(&line, x, y).unwrap();
            line = word.to_string();
            y += line_height;
        } else {
            line = test_line;
        }
    }
    if !line.is_empty() {
        ctx.fill_text(&line, x, y).unwrap();
    }
}
