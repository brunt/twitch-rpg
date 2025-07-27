use leptos::either::EitherOf16::P;

pub const SPRITE_DIMENSION: u32 = 56;
pub const ITEM_SPRITE_DIMENSION: u32 = 34;

pub const PROJECTILE_SPRITE_DIMENSION: u32 = 31;

#[derive(Copy, Clone, Debug)]
pub struct SpriteRect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl SpriteRect {
    pub const fn at(ix: u32, iy: u32) -> SpriteRect {
        Self {
            x: (ix * SPRITE_DIMENSION) as f64,
            y: (iy * SPRITE_DIMENSION) as f64,
            w: SPRITE_DIMENSION as f64,
            h: SPRITE_DIMENSION as f64,
        }
    }

    pub const fn item_at(ix: u32, iy: u32) -> SpriteRect {
        Self {
            x: (ix * ITEM_SPRITE_DIMENSION) as f64,
            y: (iy * ITEM_SPRITE_DIMENSION) as f64,
            w: ITEM_SPRITE_DIMENSION as f64,
            h: ITEM_SPRITE_DIMENSION as f64,
        }
    }

    pub const fn projectile_at(ix: u32, iy: u32) -> SpriteRect {
        Self {
            x: (ix * PROJECTILE_SPRITE_DIMENSION) as f64,
            y: (iy * PROJECTILE_SPRITE_DIMENSION) as f64,
            w: PROJECTILE_SPRITE_DIMENSION as f64,
            h: PROJECTILE_SPRITE_DIMENSION as f64,
        }
    }
}

pub mod damage_fx_sprites;
pub mod items_sprites;
pub mod monsters_sprites;
pub mod spellfx_anim_1_sprites;
pub mod spellfx_anim_2_sprites;
pub mod spellfx_anim_3_sprites;
pub mod spellfx_anim_4_sprites;
pub mod spellfx_anim_5_sprites;
pub mod spellfx_missiles_sprites;
pub mod terrain_sprites;
