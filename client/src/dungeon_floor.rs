use crate::components::draw_sprite;
use crate::sprites::SPRITE_DIMENSION;
use crate::sprites::monsters_sprites::{enemy_sprites, player_sprite};
use crate::sprites::terrain_sprites::{TileSet, get_dead_sprite, get_terrain};
use common::{EntityPosition, Form, Health, PlayerClass};
use std::str::FromStr;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

pub fn draw_dungeon_floor(
    ctx: &CanvasRenderingContext2d,
    terrain_image: &HtmlImageElement,
    monster_image: &HtmlImageElement,
    floor: &[Vec<u8>],
    canvas_width: f64,
    canvas_height: f64,
    camera_position: (u32, u32),
    difficulty: u32,
    floor_pos: &[EntityPosition],
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

    // Draw entities at positions
    for EntityPosition {
        entity_type: class,
        position,
        level,
        health,
        form,
        ..
    } in floor_pos.iter()
    {
        // Skip if out of map bounds to be robust
        let row = position.x as usize;
        let col = position.y as usize;
        if row >= map_height || col >= map_width {
            continue;
        }

        let x = (col as f64 - row as f64) * (SPRITE_DIMENSION as f64 / 2.0) + canvas_width / 2.0
            - SPRITE_DIMENSION as f64 / 2.0
            + offset_x;
        let y = (col as f64 + row as f64) * (SPRITE_DIMENSION as f64 / 4.0) + offset_y;

        // Draw the sprite at (x, y)
        if let Some(Health::Dead) = health {
            draw_sprite(ctx, terrain_image, &get_dead_sprite(), x, y, 1.0, None)
        } else {
            match class.as_str() {
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
                        let scale = if let Form::Scaled(x) = form { *x } else { 1.0 };
                        
                        let adjusted_x = x - (SPRITE_DIMENSION as f64 * (scale - 1.0) / 2.0);
                        let adjusted_y = y - (SPRITE_DIMENSION as f64 * (scale - 1.0));
                        
                        draw_sprite(
                            ctx,
                            monster_image,
                            player_sprite((&form, &player_class, *level)),
                            adjusted_x,
                            adjusted_y,
                            scale,
                            None,
                        )
                    }
                }
            }
        }
    }
}
