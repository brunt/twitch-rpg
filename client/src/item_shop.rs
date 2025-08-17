use crate::SpriteSheets;
use crate::components::draw_item_sprite;
use crate::sprites::ITEM_SPRITE_DIMENSION;
use crate::sprites::items_sprites::ITEMS_SPRITES;
use common::{ItemQuality, MenuItem, ShopItem};
use web_sys::CanvasRenderingContext2d;
// /////////////////////////
// //  ITEM NAME         #//
// // ITEM IMAGE  SLOT    //
// // /////////   COST    //
// // STATS               //
// // DESCRIPTION         //
// //                     //
// /////////////////////////

pub(crate) fn draw_shop_interface(
    ctx: &CanvasRenderingContext2d,
    sprites: &SpriteSheets,
    items: &Vec<(MenuItem, ShopItem)>,
    start_x: f64,
    start_y: f64,
    slots_per_row: usize,
) {
    const ITEM_SLOT_HEIGHT: f64 = 250.0;
    const ITEM_SLOT_WIDTH: f64 = 300.0;
    const MARGIN: f64 = 16.0;
    const TITLE_TEXT_FONT: &str = "bold 18px sans-serif";
    const BOLD_TEXT_FONT: &str = "bold 16px sans-serif";
    const BOLD_SM_TEXT_FONT: &str = "bold 16px sans-serif";
    const SM_TEXT_FONT: &str = "14px sans-serif";

    ctx.draw_image_with_html_image_element(&sprites.background, 0.0, 0.0)
        .unwrap();

    for (menu_item, item) in items.iter() {
        let row = menu_item.0 / slots_per_row;
        let col = menu_item.0 % slots_per_row;
        let x = start_x + col as f64 * (ITEM_SLOT_WIDTH + MARGIN);
        let y = start_y + row as f64 * (ITEM_SLOT_HEIGHT + MARGIN);
        let quality = match item.quality.clone() {
            ItemQuality::Common => "#ddd",
            ItemQuality::Uncommon => "#af0",
            ItemQuality::Rare => "#0af",
            ItemQuality::Epic => "#c3f",
            ItemQuality::Legendary => "#f90",
        };

        // Draw slot rectangle
        ctx.set_fill_style_str("#222");
        ctx.fill_rect(x, y, ITEM_SLOT_WIDTH, ITEM_SLOT_HEIGHT);

        // draw frame around slot rectangle
        ctx.set_fill_style_str(quality);
        ctx.stroke_rect(x, y, ITEM_SLOT_WIDTH, ITEM_SLOT_HEIGHT);

        // Draw Menu Item #
        ctx.set_fill_style_str("fff");
        ctx.set_font(BOLD_TEXT_FONT);
        ctx.fill_text(
            &format!("{}", menu_item.0),
            x + ITEM_SLOT_WIDTH - MARGIN,
            y + 20.0,
        )
        .unwrap();

        // Draw item sprite centered in slot
        let sprite_x = x + MARGIN * 1.5;
        let sprite_y = y + 60.0;
        draw_item_sprite(
            ctx,
            &sprites.items,
            ITEMS_SPRITES.get(&item.id),
            sprite_x,
            sprite_y,
        );

        // draw frame around sprite
        ctx.set_stroke_style_str(quality);
        ctx.set_line_width(1.0);
        let frame_x = sprite_x - 1.0;
        let frame_y = sprite_y - 1.0;
        let frame_w = ITEM_SPRITE_DIMENSION as f64 + 30.0;
        let frame_h = ITEM_SPRITE_DIMENSION as f64 + 30.0;
        ctx.stroke_rect(frame_x - 8.0, frame_y - 8.0, frame_w - 10.0, frame_h - 10.0);

        // Draw item name
        ctx.set_fill_style_str(quality);
        ctx.set_font(TITLE_TEXT_FONT);
        ctx.fill_text(&item.name, x + MARGIN, y + MARGIN * 2.0)
            .unwrap();

        // Draw equipment slot value
        ctx.set_font(BOLD_TEXT_FONT);
        ctx.fill_text(&item.equip_slot.to_string(), x + 80.0, y + 65.0)
            .unwrap();

        // Draw price
        ctx.set_fill_style_str("#ffd700");
        ctx.set_font(SM_TEXT_FONT);
        ctx.fill_text(&format!("{}", item.price), x + 80.0, y + 85.0)
            .unwrap();

        // Draw gold sprite
        draw_item_sprite(
            ctx,
            &sprites.items,
            Some(ITEMS_SPRITES.get(&138).unwrap()),
            x + 80.0 + MARGIN,
            y + 65.0,
        );

        // Draw stats
        if let Some(item_stats) = &item.stats {
            ctx.set_fill_style_str("#fff");
            ctx.set_font(BOLD_SM_TEXT_FONT);
            let mut col_x = x + MARGIN; // start first column
            let col_y_start = y + 135.0; // top of slot

            // ---- Attack modifiers ----
            if let Some(atk) = &item_stats.attack_modifiers {
                let mut col_y = col_y_start;
                if let Some(dmg) = atk.damage_bonus {
                    ctx.fill_text(&format!("ATK: {}", dmg), col_x, col_y)
                        .unwrap();
                    col_y += 15.0;
                }
                if let Some(hit) = atk.hit_rating_bonus {
                    ctx.fill_text(&format!("HIT: {}", hit), col_x, col_y)
                        .unwrap();
                    col_y += 15.0;
                }
                if let Some(range) = atk.range_bonus {
                    ctx.fill_text(&format!("RANGE: {}", range), col_x, col_y)
                        .unwrap();
                    col_y += 15.0;
                }
                if let Some(crit) = atk.crit_damage_multiplier {
                    ctx.fill_text(&format!("CRIT: {}", crit), col_x, col_y)
                        .unwrap();
                    col_y += 15.0;
                }
                if let Some(cd) = atk.cooldown_reduction_ms {
                    ctx.fill_text(&format!("CDWN: {}", cd / 1000), col_x, col_y)
                        .unwrap();
                }
                col_x += 100.0; // move to next column for defense
            }

            // ---- Defense modifiers ----
            if let Some(def) = &item_stats.defense_modifiers {
                let mut col_y = col_y_start;
                if let Some(dmg) = def.damage_reduction {
                    ctx.fill_text(&format!("DEF: {}", dmg), col_x, col_y)
                        .unwrap();
                    col_y += 15.0;
                }
                if let Some(ev) = def.evasion_rating {
                    ctx.fill_text(&format!("EV: {}", ev), col_x, col_y).unwrap();
                    // col_y += 15.0;
                }
                col_x += 100.0; // move to next column for attributes
            }

            // ---- Attributes ----
            let mut col_y = col_y_start;
            if let Some(str) = item_stats.strength {
                ctx.fill_text(&format!("STR: {}", str), col_x, col_y)
                    .unwrap();
                col_y += 15.0;
            }
            if let Some(agility) = item_stats.agility {
                ctx.fill_text(&format!("AGI: {}", agility), col_x, col_y)
                    .unwrap();
                col_y += 15.0;
            }
            if let Some(int) = item_stats.intelligence {
                ctx.fill_text(&format!("INT: {}", int), col_x, col_y)
                    .unwrap();
            }
        }

        // Draw description (shortened for slot)
        ctx.set_fill_style_str("#aaa");
        ctx.set_font(SM_TEXT_FONT);
        draw_wrapped_text(
            ctx,
            &item.description,
            x + 10.0,
            y + ITEM_SLOT_HEIGHT - 35.0,
            ITEM_SLOT_WIDTH - MARGIN,
            14.0,
        );
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
