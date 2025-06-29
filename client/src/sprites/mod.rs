pub const SPRITE_DIMENSION: f64 = 56.0;
pub struct SpriteRect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl SpriteRect {
    pub const fn at(ix: f64, iy: f64) -> SpriteRect {
        Self {
            x: ix * SPRITE_DIMENSION,
            y: iy * SPRITE_DIMENSION,
            w: SPRITE_DIMENSION,
            h: SPRITE_DIMENSION,
        }
    }
}

pub mod monsters_sprites;
pub mod terrain_sprites;
pub mod damage_fx_sprites;
pub mod items_sprites;
pub mod spellfx_anim_1_sprites;
pub mod spellfx_anim_2_sprites;
pub mod spellfx_anim_3_sprites;
pub mod spellfx_anim_4_sprites;
pub mod spellfx_anim_5_sprites;
pub mod spellfx_missiles_sprites;
