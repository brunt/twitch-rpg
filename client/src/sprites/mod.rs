pub const TERRAIN_SPRITE_DIMENSION: f64 = 56.0;
pub struct SpriteRect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl SpriteRect {
    pub const fn at(ix: f64, iy: f64) -> SpriteRect {
        Self {
            x: ix * TERRAIN_SPRITE_DIMENSION,
            y: iy * TERRAIN_SPRITE_DIMENSION,
            w: TERRAIN_SPRITE_DIMENSION,
            h: TERRAIN_SPRITE_DIMENSION,
        }
    }
}

pub mod monster_sprites;
pub mod terrain_sprites;
