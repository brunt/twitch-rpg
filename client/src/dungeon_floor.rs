use crate::components::draw_sprite;
use crate::sprites::SPRITE_DIMENSION;
use crate::sprites::monsters_sprites::MONSTERS_SPRITE_339;
use crate::sprites::terrain_sprites::{
    TERRAIN_SPRITE_158, TERRAIN_SPRITE_218, TERRAIN_SPRITE_219, TERRAIN_SPRITE_432,
    TERRAIN_SPRITE_467, TERRAIN_SPRITE_653,
};
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

pub fn draw_dungeon_floor(
    ctx: &CanvasRenderingContext2d,
    terrain_image: &HtmlImageElement,
    monster_image: &HtmlImageElement,
    map_width: usize,
    map_height: usize,
    canvas_width: f64,
    canvas_height: f64,
) {
    for row in 0..map_width {
        for col in 0..map_height {
            let screen_x =
                (row as f64 - col as f64) * (SPRITE_DIMENSION / 2.0) + canvas_width / 2.0;
            let screen_y = (row as f64 + col as f64) * (SPRITE_DIMENSION / 2.0);

            let is_top_left_corner = row == 0 && col == 0;
            let is_top_right_corner = row == 0 && col == map_height - 1;
            let is_bottom_left_corner = row == map_width - 1 && col == 0;
            let is_top_corner = row == map_width - 1 && col == map_height - 1;
            let is_ne_sw = row == 0 || row == map_width - 1;
            let is_nw_se = col == 0 || col == map_height - 1;

            let x = (col as f64 - row as f64) * (SPRITE_DIMENSION / 2.0) + canvas_width / 2.0
                - SPRITE_DIMENSION / 2.0;
            let y = (col as f64 + row as f64) * (SPRITE_DIMENSION / 4.0);

            draw_sprite(ctx, terrain_image, &TERRAIN_SPRITE_653, x, y, None);

            if is_ne_sw {
                draw_sprite(ctx, terrain_image, &TERRAIN_SPRITE_467, x, y, None);
            }
            if is_nw_se {
                draw_sprite(ctx, terrain_image, &TERRAIN_SPRITE_432, x, y, None);
            }
            if is_top_left_corner {
                draw_sprite(ctx, terrain_image, &TERRAIN_SPRITE_432, x, y, None);
            }

            // Example of drawing additional sprites
            draw_sprite(
                ctx,
                terrain_image,
                &TERRAIN_SPRITE_158,
                SPRITE_DIMENSION * 6.0,
                SPRITE_DIMENSION * 5.0,
                None,
            );
            draw_sprite(
                ctx,
                terrain_image,
                &TERRAIN_SPRITE_218,
                SPRITE_DIMENSION * 5.0,
                SPRITE_DIMENSION * 5.0,
                None,
            );
            draw_sprite(
                ctx,
                terrain_image,
                &TERRAIN_SPRITE_219,
                SPRITE_DIMENSION * 5.5,
                SPRITE_DIMENSION * 5.25,
                None,
            );
            draw_sprite(
                ctx,
                monster_image,
                &MONSTERS_SPRITE_339,
                SPRITE_DIMENSION * 7.5,
                SPRITE_DIMENSION * 5.0,
                None,
            );
        }
    }
}
