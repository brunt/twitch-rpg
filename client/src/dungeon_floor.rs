use crate::components::draw_sprite;
use crate::sprites::SPRITE_DIMENSION;
use crate::sprites::monsters_sprites::{enemy_sprite, player_sprite};
use crate::sprites::terrain_sprites::{TileSet, get_terrain};
use common::{EntityPosition, Form, PlayerClass};
use std::str::FromStr;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

pub fn draw_dungeon_floor(
    ctx: &CanvasRenderingContext2d,
    terrain_image: &HtmlImageElement,
    monster_image: &HtmlImageElement,
    floor: &Vec<Vec<u8>>,
    canvas_width: f64,
    canvas_height: f64,
    camera_position: (u32, u32),
    difficulty: u32,
    floor_pos: &Vec<EntityPosition>,
) {
    const DEFAULT_TERRAIN_SCALE: f64 = 1.05;
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

    let [floor_tile, wall_tile, item_tile] = match difficulty {
        1 => get_terrain(&TileSet::Woods),
        2 => get_terrain(&TileSet::Mountain),
        3 => get_terrain(&TileSet::Desert),
        4 => get_terrain(&TileSet::Tundra),
        _ => unimplemented!(),
    };

    let enemy_sprite = enemy_sprite();

    for row in 0..map_height {
        for col in 0..map_width {
            let val = floor[row][col];

            let x = (col as f64 - row as f64) * (SPRITE_DIMENSION / 2.0) + canvas_width / 2.0
                - SPRITE_DIMENSION / 2.0
                + offset_x;
            let y = (col as f64 + row as f64) * (SPRITE_DIMENSION / 4.0) + offset_y;

            match val {
                1 => draw_sprite(
                    ctx,
                    terrain_image,
                    &floor_tile,
                    x,
                    y,
                    DEFAULT_TERRAIN_SCALE,
                    None,
                ),
                2 => {
                    //draw both the floor and the wall so there are no gaps
                    draw_sprite(
                        ctx,
                        terrain_image,
                        &floor_tile,
                        x,
                        y,
                        DEFAULT_TERRAIN_SCALE,
                        None,
                    );
                    draw_sprite(
                        ctx,
                        terrain_image,
                        &wall_tile,
                        x,
                        y,
                        DEFAULT_TERRAIN_SCALE,
                        None,
                    )
                }
                _ => {}
            }
        }
    }

    // Draw entities at positions
    for EntityPosition {
        class,
        position,
        level,
        ..
    } in floor_pos.iter()
    {
        // Skip if out of map bounds to be robust
        let row = position.x as usize;
        let col = position.y as usize;
        if row >= map_height || col >= map_width {
            continue;
        }

        let x = (col as f64 - row as f64) * (SPRITE_DIMENSION / 2.0) + canvas_width / 2.0
            - SPRITE_DIMENSION / 2.0
            + offset_x;
        let y = (col as f64 + row as f64) * (SPRITE_DIMENSION / 4.0) + offset_y;

        // Draw the sprite at (x, y)
        match class.as_str() {
            "Enemy" => draw_sprite(ctx, monster_image, &enemy_sprite, x, y, 1.0, None),
            "Item" => draw_sprite(ctx, terrain_image, &item_tile, x, y, 1.0, None),
            cl => {
                if let Ok(player_class) = PlayerClass::from_str(cl) {
                    draw_sprite(
                        ctx,
                        monster_image,
                        player_sprite((&Form::Normal, &player_class, *level)),
                        x,
                        y,
                        1.0,
                        None,
                    )
                }
            }
        }
    }
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
