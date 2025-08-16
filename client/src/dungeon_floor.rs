use crate::components::draw_sprite;
use crate::components::game_canvas::RenderEntity;
use crate::sprites::SPRITE_DIMENSION;
use crate::sprites::monsters_sprites::{enemy_sprites, player_sprite};
use crate::sprites::terrain_sprites::{TileSet, get_dead_sprite, get_terrain};
use common::{EntityPosition, Form, Health, PlayerClass};
use std::collections::HashMap;
use std::str::FromStr;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

pub fn draw_dungeon_floor(
    ctx: &CanvasRenderingContext2d,
    terrain_image: &HtmlImageElement,
    monster_image: &HtmlImageElement,
    floor: &[Vec<u8>],
    canvas_width: f64,
    canvas_height: f64,
    camera_position: (f64, f64),
    difficulty: u32,
    entity_render_map: &HashMap<u32, RenderEntity>,
) {
    const DEFAULT_TERRAIN_SCALE: f64 = 1.05;
    let map_height = floor.len();
    let map_width = floor[0].len();

    // camera tile in map coordinates
    let (camera_row, camera_col) = camera_position;

    // compute where the camera tile would be drawn (its screen x/y)
    let camera_screen_x = (camera_col as f64 - camera_row as f64) * (SPRITE_DIMENSION as f64 / 2.0)
        + canvas_width / 2.0
        - SPRITE_DIMENSION as f64 / 2.0;
    let camera_screen_y = (camera_col as f64 + camera_row as f64) * (SPRITE_DIMENSION as f64 / 4.0);

    // center of the canvas
    let center_x = canvas_width / 2.0;
    let center_y = canvas_height / 2.0;

    // offset needed to put the camera tile in the center
    let offset_x = center_x - camera_screen_x;
    let offset_y = center_y - camera_screen_y;

    let [
        floor_tile,
        wall_tile,
        item_tile,
        opened_tile,
        ud_door,
        lr_door,
        stair_tile,
        dd1,
        dd2,
    ] = match difficulty {
        1 => get_terrain(&TileSet::Woods),
        2 => get_terrain(&TileSet::Mountain),
        3 => get_terrain(&TileSet::Desert),
        4 => get_terrain(&TileSet::Tundra),
        _ => unimplemented!(),
    };

    for row in 0..map_height {
        for col in 0..map_width {
            let val = floor[row][col];

            let x = (col as f64 - row as f64) * (SPRITE_DIMENSION as f64 / 2.0)
                + canvas_width / 2.0
                - SPRITE_DIMENSION as f64 / 2.0
                + offset_x;
            let y = (col as f64 + row as f64) * (SPRITE_DIMENSION as f64 / 4.0) + offset_y;

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
                // ud_door
                3 => {
                    //TODO: corridor direction
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
                        &ud_door,
                        x,
                        y,
                        DEFAULT_TERRAIN_SCALE,
                        None,
                    );
                }
                4 => {
                    draw_sprite(
                        ctx,
                        terrain_image,
                        &floor_tile,
                        x,
                        y,
                        DEFAULT_TERRAIN_SCALE,
                        None,
                    );
                    draw_sprite(ctx, terrain_image, &dd1, x, y, DEFAULT_TERRAIN_SCALE, None);
                }
                5 => {
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
                        &stair_tile,
                        x,
                        y,
                        DEFAULT_TERRAIN_SCALE,
                        None,
                    );
                }
                _ => {}
            }
        }
    }

    for re in entity_render_map.values() {
        let (tile_row, tile_col) = re.render_pos;

        // Skip if out of bounds
        if tile_row < 0.0
            || tile_col < 0.0
            || tile_row >= map_height as f64
            || tile_col >= map_width as f64
        {
            continue;
        }

        let x = (tile_col - tile_row) * (SPRITE_DIMENSION as f64 / 2.0) + canvas_width / 2.0
            - SPRITE_DIMENSION as f64 / 2.0
            + offset_x;
        let y = (tile_col + tile_row) * (SPRITE_DIMENSION as f64 / 4.0) + offset_y;

        if matches!(re.health, Some(Health::Dead)) {
            draw_sprite(ctx, terrain_image, &get_dead_sprite(), x, y, 1.0, None);
        } else {
            match re.entity_type.as_str() {
                "E1" => draw_sprite(
                    ctx,
                    monster_image,
                    enemy_sprites(difficulty)[0],
                    x,
                    y,
                    1.0,
                    None,
                ),
                "E2" => draw_sprite(
                    ctx,
                    monster_image,
                    enemy_sprites(difficulty)[1],
                    x,
                    y,
                    1.0,
                    None,
                ),
                "E3" => draw_sprite(
                    ctx,
                    monster_image,
                    enemy_sprites(difficulty)[2],
                    x,
                    y,
                    1.0,
                    None,
                ),
                "Item" => draw_sprite(ctx, terrain_image, &item_tile, x, y, 1.0, None),
                "Opened" => draw_sprite(ctx, terrain_image, &opened_tile, x, y, 1.0, None),
                cl => {
                    if let Ok(player_class) = PlayerClass::from_str(cl) {
                        let scale = if let Form::Scaled(x) = re.form {
                            x
                        } else {
                            1.0
                        };

                        let adjusted_x = x - (SPRITE_DIMENSION as f64 * (scale - 1.0) / 2.0);
                        let adjusted_y = y - (SPRITE_DIMENSION as f64 * (scale - 1.0));

                        draw_sprite(
                            ctx,
                            monster_image,
                            player_sprite((&re.form, &player_class, re.level)),
                            adjusted_x,
                            adjusted_y,
                            scale,
                            if matches!(re.form, Form::Invisible) {
                                Some(0.50)
                            } else {
                                None
                            },
                        )
                    }
                }
            }
        }
    }
}
