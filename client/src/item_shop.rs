use crate::components::draw_item_sprite;
use crate::sprites::items_sprites::ITEMS_SPRITES;
use crate::sprites::{ITEM_SPRITE_DIMENSION, SpriteRect};
use common::{ItemQuality, MenuItem, ShopItem};
use std::collections::HashMap;
use leptos::context::use_context;
use leptos::prelude::{Get, LocalStorage, ReadSignal};
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};
use crate::SpriteSheets;
// TODO: reshape shop item
// /////////////////////////
// //  ITEM NAME         #//
// // ITEM IMAGE  SLOT    //
// // /////////   COST    //
// // DESCRIPTION         //
// //                     //
// /////////////////////////

pub(crate) fn draw_shop_interface(
    ctx: &CanvasRenderingContext2d,
    sprites: &SpriteSheets,
    items: &HashMap<MenuItem, ShopItem>,
    start_x: f64,
    start_y: f64,
    slots_per_row: usize,
) {
    const ITEM_SLOT_SIZE: f64 = 150.0;
    const ITEM_SLOT_WIDTH: f64 = ITEM_SLOT_SIZE * 1.5;

    let padding = 12.0;
            
   ctx.draw_image_with_html_image_element(&sprites.background, 0.0, 0.0).unwrap();

    for (menu_item, item) in items.iter() {
        let row = menu_item.0 / slots_per_row;
        let col = menu_item.0 % slots_per_row;
        let x = start_x + col as f64 * (ITEM_SLOT_WIDTH + padding);
        let y = start_y + row as f64 * (ITEM_SLOT_SIZE + padding);

        // Draw slot rectangle
        ctx.set_fill_style_str("#222");
        ctx.fill_rect(x, y, ITEM_SLOT_WIDTH, ITEM_SLOT_SIZE);
        // Draw item sprite centered in slot
        let sprite_x = x + (ITEM_SLOT_SIZE - ITEM_SPRITE_DIMENSION as f64) * 0.20;
        let sprite_y = y + 20.0;
        draw_item_sprite(
            ctx,
            &sprites.items,
            ITEMS_SPRITES.get(&item.id),
            sprite_x,
            sprite_y,
        );
        ctx.set_fill_style_str("#fff");
        ctx.line_to(100.0, 100.0);

        // draw frame around sprite
        ctx.set_stroke_style_str(match item.quality.clone() {
            ItemQuality::Common => "#ddd",
            ItemQuality::Uncommon => "#af0",
            ItemQuality::Rare => "#0af",
            ItemQuality::Epic => "#c3f",
            ItemQuality::Legendary => "#f90",
        });
        ctx.set_line_width(1.0);
        let frame_x = sprite_x - 1.0;
        let frame_y = sprite_y - 1.0;
        let frame_w = ITEM_SPRITE_DIMENSION as f64 + 30.0;
        let frame_h = ITEM_SPRITE_DIMENSION as f64 + 30.0;
        ctx.stroke_rect(frame_x - 8.0, frame_y - 8.0, frame_w - 10.0, frame_h - 10.0);

        // Draw item name
        ctx.set_fill_style_str(match item.quality.clone() {
            ItemQuality::Common => "#ddd",
            ItemQuality::Uncommon => "#af0",
            ItemQuality::Rare => "#0af",
            ItemQuality::Epic => "#c3f",
            ItemQuality::Legendary => "#f90",
        });
        ctx.set_font("bold 16px sans-serif");
        ctx.fill_text(&item.name, x + 8.0, y + ITEM_SLOT_SIZE - 60.0)
            .unwrap();

        // Draw price
        ctx.set_fill_style_str("#ffd700");
        ctx.set_font("bold 14px sans-serif");
        ctx.fill_text(
            &format!("{}", item.price), //TODO: redundant price info?
            x + (ITEM_SLOT_SIZE - ITEM_SPRITE_DIMENSION as f64) * 0.85,
            y + ITEM_SLOT_SIZE - 103.0,
        )
        .unwrap();

        // Draw gold sprite
        draw_item_sprite(
            ctx,
            &sprites.items,
            Some(ITEMS_SPRITES.get(&138).unwrap()),
            x + 70.0,
            y + 25.0,
        );

        // Draw description (shortened for slot)
        ctx.set_fill_style_str("#aaa");
        ctx.set_font("12px sans-serif");
        draw_wrapped_text(
            ctx,
            &item.description,
            x + 8.0,
            y + ITEM_SLOT_SIZE - 45.0,
            ITEM_SLOT_WIDTH - 16.0,
            14.0,
        );

        // Draw equipment slot value
        ctx.set_font("bold 12px sans-serif");
        ctx.fill_text(&item.equip_slot.to_string(), x + 80.0, y + 25.0)
            .unwrap();

        // Draw Menu Item #
        ctx.set_fill_style_str("#fff");
        ctx.set_font("bold 16px sans-serif");
        ctx.fill_text(
            &format!("{}", menu_item.0),
            x + ITEM_SLOT_WIDTH - 16.0,
            y + 20.0,
        )
        .unwrap();
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
    let mut line = String::new();

    for word in text.split_whitespace() {
        let test_line = if line.is_empty() {
            word.to_string()
        } else {
            format!("{line} {word}")
        };

        if ctx.measure_text(&test_line).unwrap().width() > max_width && !line.is_empty() {
            ctx.fill_text(&line, x, y).unwrap();
            line.clear();
            line.push_str(word);
            y += line_height;
        } else {
            line = test_line;
        }
    }
    if !line.is_empty() {
        ctx.fill_text(&line, x, y).unwrap();
    }
}
