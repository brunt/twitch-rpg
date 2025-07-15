use crate::components::draw_sprite;
use crate::sprites::SPRITE_DIMENSION;
use crate::sprites::monsters_sprites::MONSTERS_SPRITE_339;
use crate::sprites::terrain_sprites::{TERRAIN_SPRITE_158, TERRAIN_SPRITE_218, TERRAIN_SPRITE_219, TERRAIN_SPRITE_403, TERRAIN_SPRITE_432, TERRAIN_SPRITE_467, TERRAIN_SPRITE_653};
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

pub fn draw_dungeon_floor(
    ctx: &CanvasRenderingContext2d,
    terrain_image: &HtmlImageElement,
    _monster_image: &HtmlImageElement,
    floor: &Vec<Vec<u8>>,
    canvas_width: f64,
    canvas_height: f64,
    camera_position: (u32, u32),
) {
    let map_height = floor.len();
    let map_width = floor[0].len();
    
    // camera tile in map coordinates
    let (camera_row, camera_col) = camera_position;

    // compute where the camera tile would be drawn (its screen x/y)
    let camera_screen_x = (camera_col as f64 - camera_row as f64) * (SPRITE_DIMENSION / 2.0)
        + canvas_width / 2.0
        - SPRITE_DIMENSION / 2.0;
    let camera_screen_y = (camera_col as f64 + camera_row as f64) * (SPRITE_DIMENSION / 4.0);

    // center of the canvas
    let center_x = canvas_width / 2.0;
    let center_y = canvas_height / 2.0;

    // offset needed to put the camera tile in the center
    let offset_x = center_x - camera_screen_x;
    let offset_y = center_y - camera_screen_y;
    

    for row in 0..map_width {
        for col in 0..map_height {
            let val = floor[row][col];

            let x = (col as f64 - row as f64) * (SPRITE_DIMENSION / 2.0) + canvas_width / 2.0
                - SPRITE_DIMENSION / 2.0 + offset_x;
            let y = (col as f64 + row as f64) * (SPRITE_DIMENSION / 4.0) + offset_y;

            match val {
                0 => draw_sprite(ctx, terrain_image, &TERRAIN_SPRITE_653, x, y, 1.0, None),
                1 => {
                    //draw both the floor and the wall so there are no gaps
                    draw_sprite(ctx, terrain_image, &TERRAIN_SPRITE_653, x, y, 1.0, None);
                    draw_sprite(ctx, terrain_image, &TERRAIN_SPRITE_467, x, y, 1.0, None)
                },
                _ => unimplemented!(),
            }
            // let screen_x =
            //     (row as f64 - col as f64) * (SPRITE_DIMENSION / 2.0) + canvas_width / 2.0;
            // let screen_y = (row as f64 + col as f64) * (SPRITE_DIMENSION / 2.0);

            // let is_top_left_corner = row == 0 && col == 0;
            // let is_top_right_corner = row == 0 && col == map_height - 1;
            // let is_bottom_left_corner = row == map_width - 1 && col == 0;
            // let is_top_corner = row == map_width - 1 && col == map_height - 1;
            // let is_ne_sw = row == 0 || row == map_width - 1;
            // let is_nw_se = col == 0 || col == map_height - 1;
            // draw_sprite(ctx, terrain_image, &TERRAIN_SPRITE_653, x, y, 1.0, None);
            //
            // if is_ne_sw {
            //     draw_sprite(ctx, terrain_image, &TERRAIN_SPRITE_467, x, y, 1.0, None);
            // }
            // if is_nw_se {
            //     draw_sprite(ctx, terrain_image, &TERRAIN_SPRITE_432, x, y, 1.0, None);
            // }
            // if is_top_left_corner {
            //     draw_sprite(ctx, terrain_image, &TERRAIN_SPRITE_432, x, y, 1.0, None);
            // }
        }
    }
}
