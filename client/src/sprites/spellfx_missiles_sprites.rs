use crate::sprites::SpriteRect;
use common::{DamageType, Projectile};
use std::f32::consts::PI;

pub struct ActiveProjectile {
    pub from: (u32, u32),
    pub to: (u32, u32),
    pub sprite: SpriteRect,
    pub start_time: f64,
}

fn direction_index(from: (u32, u32), to: (u32, u32)) -> usize {
    let dx = to.0 as f32 - from.0 as f32;
    let dy = to.1 as f32 - from.1 as f32;

    if dx == 0.0 && dy == 0.0 {
        return 0;
    }

    let angle = dy.atan2(dx); // standard atan2: angle from +X, counter-clockwise
    let adjusted = (-angle + PI / 2.0) % (2.0 * PI); // rotate so 0 is up, clockwise rotation
    let index = (adjusted / (PI / 4.0)).round() as usize % 8;

    index
}
impl ActiveProjectile {
    pub fn new(p: &Projectile, start: f64) -> Self {
        let from = (p.position.x, p.position.y);
        let to = (p.target_position.x, p.target_position.y);
        let index = direction_index(from, to);
        let sprite = SpriteRect::projectile_at(
            index as f64,
            match p.damage_type {
                DamageType::Physical => 0.0,
                DamageType::Purple => 3.0,
                _ => 1.0,
            },
        ); // assuming basic arrow for now
        Self {
            from,
            to,
            sprite,
            start_time: start,
        }
    }
}

#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_0: SpriteRect = SpriteRect::projectile_at(0.0, 0.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_8: SpriteRect = SpriteRect::projectile_at(0.0, 1.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_16: SpriteRect = SpriteRect::projectile_at(0.0, 2.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_24: SpriteRect = SpriteRect::projectile_at(0.0, 3.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_32: SpriteRect = SpriteRect::projectile_at(0.0, 4.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_40: SpriteRect = SpriteRect::projectile_at(0.0, 5.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_48: SpriteRect = SpriteRect::projectile_at(0.0, 6.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_56: SpriteRect = SpriteRect::projectile_at(0.0, 7.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_64: SpriteRect = SpriteRect::projectile_at(0.0, 8.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_72: SpriteRect = SpriteRect::projectile_at(0.0, 9.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_80: SpriteRect = SpriteRect::projectile_at(0.0, 10.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_88: SpriteRect = SpriteRect::projectile_at(0.0, 11.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_96: SpriteRect = SpriteRect::projectile_at(0.0, 12.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_104: SpriteRect = SpriteRect::projectile_at(0.0, 13.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_112: SpriteRect = SpriteRect::projectile_at(0.0, 14.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_120: SpriteRect = SpriteRect::projectile_at(0.0, 15.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_128: SpriteRect = SpriteRect::projectile_at(0.0, 16.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_136: SpriteRect = SpriteRect::projectile_at(0.0, 17.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_144: SpriteRect = SpriteRect::projectile_at(0.0, 18.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_1: SpriteRect = SpriteRect::projectile_at(1.0, 0.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_9: SpriteRect = SpriteRect::projectile_at(1.0, 1.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_17: SpriteRect = SpriteRect::projectile_at(1.0, 2.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_25: SpriteRect = SpriteRect::projectile_at(1.0, 3.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_33: SpriteRect = SpriteRect::projectile_at(1.0, 4.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_41: SpriteRect = SpriteRect::projectile_at(1.0, 5.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_49: SpriteRect = SpriteRect::projectile_at(1.0, 6.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_57: SpriteRect = SpriteRect::projectile_at(1.0, 7.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_65: SpriteRect = SpriteRect::projectile_at(1.0, 8.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_73: SpriteRect = SpriteRect::projectile_at(1.0, 9.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_81: SpriteRect = SpriteRect::projectile_at(1.0, 10.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_89: SpriteRect = SpriteRect::projectile_at(1.0, 11.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_97: SpriteRect = SpriteRect::projectile_at(1.0, 12.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_105: SpriteRect = SpriteRect::projectile_at(1.0, 13.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_113: SpriteRect = SpriteRect::projectile_at(1.0, 14.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_121: SpriteRect = SpriteRect::projectile_at(1.0, 15.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_129: SpriteRect = SpriteRect::projectile_at(1.0, 16.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_137: SpriteRect = SpriteRect::projectile_at(1.0, 17.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_145: SpriteRect = SpriteRect::projectile_at(1.0, 18.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_2: SpriteRect = SpriteRect::projectile_at(2.0, 0.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_10: SpriteRect = SpriteRect::projectile_at(2.0, 1.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_18: SpriteRect = SpriteRect::projectile_at(2.0, 2.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_26: SpriteRect = SpriteRect::projectile_at(2.0, 3.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_34: SpriteRect = SpriteRect::projectile_at(2.0, 4.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_42: SpriteRect = SpriteRect::projectile_at(2.0, 5.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_50: SpriteRect = SpriteRect::projectile_at(2.0, 6.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_58: SpriteRect = SpriteRect::projectile_at(2.0, 7.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_66: SpriteRect = SpriteRect::projectile_at(2.0, 8.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_74: SpriteRect = SpriteRect::projectile_at(2.0, 9.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_82: SpriteRect = SpriteRect::projectile_at(2.0, 10.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_90: SpriteRect = SpriteRect::projectile_at(2.0, 11.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_98: SpriteRect = SpriteRect::projectile_at(2.0, 12.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_106: SpriteRect = SpriteRect::projectile_at(2.0, 13.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_114: SpriteRect = SpriteRect::projectile_at(2.0, 14.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_122: SpriteRect = SpriteRect::projectile_at(2.0, 15.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_130: SpriteRect = SpriteRect::projectile_at(2.0, 16.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_138: SpriteRect = SpriteRect::projectile_at(2.0, 17.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_146: SpriteRect = SpriteRect::projectile_at(2.0, 18.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_3: SpriteRect = SpriteRect::projectile_at(3.0, 0.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_11: SpriteRect = SpriteRect::projectile_at(3.0, 1.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_19: SpriteRect = SpriteRect::projectile_at(3.0, 2.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_27: SpriteRect = SpriteRect::projectile_at(3.0, 3.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_35: SpriteRect = SpriteRect::projectile_at(3.0, 4.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_43: SpriteRect = SpriteRect::projectile_at(3.0, 5.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_51: SpriteRect = SpriteRect::projectile_at(3.0, 6.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_59: SpriteRect = SpriteRect::projectile_at(3.0, 7.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_67: SpriteRect = SpriteRect::projectile_at(3.0, 8.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_75: SpriteRect = SpriteRect::projectile_at(3.0, 9.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_83: SpriteRect = SpriteRect::projectile_at(3.0, 10.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_91: SpriteRect = SpriteRect::projectile_at(3.0, 11.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_99: SpriteRect = SpriteRect::projectile_at(3.0, 12.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_107: SpriteRect = SpriteRect::projectile_at(3.0, 13.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_115: SpriteRect = SpriteRect::projectile_at(3.0, 14.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_123: SpriteRect = SpriteRect::projectile_at(3.0, 15.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_131: SpriteRect = SpriteRect::projectile_at(3.0, 16.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_139: SpriteRect = SpriteRect::projectile_at(3.0, 17.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_147: SpriteRect = SpriteRect::projectile_at(3.0, 18.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_4: SpriteRect = SpriteRect::projectile_at(4.0, 0.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_12: SpriteRect = SpriteRect::projectile_at(4.0, 1.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_20: SpriteRect = SpriteRect::projectile_at(4.0, 2.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_28: SpriteRect = SpriteRect::projectile_at(4.0, 3.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_36: SpriteRect = SpriteRect::projectile_at(4.0, 4.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_44: SpriteRect = SpriteRect::projectile_at(4.0, 5.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_52: SpriteRect = SpriteRect::projectile_at(4.0, 6.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_60: SpriteRect = SpriteRect::projectile_at(4.0, 7.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_68: SpriteRect = SpriteRect::projectile_at(4.0, 8.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_76: SpriteRect = SpriteRect::projectile_at(4.0, 9.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_84: SpriteRect = SpriteRect::projectile_at(4.0, 10.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_92: SpriteRect = SpriteRect::projectile_at(4.0, 11.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_100: SpriteRect = SpriteRect::projectile_at(4.0, 12.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_108: SpriteRect = SpriteRect::projectile_at(4.0, 13.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_116: SpriteRect = SpriteRect::projectile_at(4.0, 14.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_124: SpriteRect = SpriteRect::projectile_at(4.0, 15.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_132: SpriteRect = SpriteRect::projectile_at(4.0, 16.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_140: SpriteRect = SpriteRect::projectile_at(4.0, 17.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_148: SpriteRect = SpriteRect::projectile_at(4.0, 18.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_5: SpriteRect = SpriteRect::projectile_at(5.0, 0.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_13: SpriteRect = SpriteRect::projectile_at(5.0, 1.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_21: SpriteRect = SpriteRect::projectile_at(5.0, 2.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_29: SpriteRect = SpriteRect::projectile_at(5.0, 3.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_37: SpriteRect = SpriteRect::projectile_at(5.0, 4.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_45: SpriteRect = SpriteRect::projectile_at(5.0, 5.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_53: SpriteRect = SpriteRect::projectile_at(5.0, 6.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_61: SpriteRect = SpriteRect::projectile_at(5.0, 7.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_69: SpriteRect = SpriteRect::projectile_at(5.0, 8.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_77: SpriteRect = SpriteRect::projectile_at(5.0, 9.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_85: SpriteRect = SpriteRect::projectile_at(5.0, 10.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_93: SpriteRect = SpriteRect::projectile_at(5.0, 11.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_101: SpriteRect = SpriteRect::projectile_at(5.0, 12.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_109: SpriteRect = SpriteRect::projectile_at(5.0, 13.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_117: SpriteRect = SpriteRect::projectile_at(5.0, 14.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_125: SpriteRect = SpriteRect::projectile_at(5.0, 15.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_133: SpriteRect = SpriteRect::projectile_at(5.0, 16.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_141: SpriteRect = SpriteRect::projectile_at(5.0, 17.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_149: SpriteRect = SpriteRect::projectile_at(5.0, 18.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_6: SpriteRect = SpriteRect::projectile_at(6.0, 0.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_14: SpriteRect = SpriteRect::projectile_at(6.0, 1.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_22: SpriteRect = SpriteRect::projectile_at(6.0, 2.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_30: SpriteRect = SpriteRect::projectile_at(6.0, 3.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_38: SpriteRect = SpriteRect::projectile_at(6.0, 4.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_46: SpriteRect = SpriteRect::projectile_at(6.0, 5.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_54: SpriteRect = SpriteRect::projectile_at(6.0, 6.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_62: SpriteRect = SpriteRect::projectile_at(6.0, 7.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_70: SpriteRect = SpriteRect::projectile_at(6.0, 8.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_78: SpriteRect = SpriteRect::projectile_at(6.0, 9.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_86: SpriteRect = SpriteRect::projectile_at(6.0, 10.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_94: SpriteRect = SpriteRect::projectile_at(6.0, 11.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_102: SpriteRect = SpriteRect::projectile_at(6.0, 12.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_110: SpriteRect = SpriteRect::projectile_at(6.0, 13.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_118: SpriteRect = SpriteRect::projectile_at(6.0, 14.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_126: SpriteRect = SpriteRect::projectile_at(6.0, 15.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_134: SpriteRect = SpriteRect::projectile_at(6.0, 16.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_142: SpriteRect = SpriteRect::projectile_at(6.0, 17.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_150: SpriteRect = SpriteRect::projectile_at(6.0, 18.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_7: SpriteRect = SpriteRect::projectile_at(7.0, 0.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_15: SpriteRect = SpriteRect::projectile_at(7.0, 1.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_23: SpriteRect = SpriteRect::projectile_at(7.0, 2.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_31: SpriteRect = SpriteRect::projectile_at(7.0, 3.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_39: SpriteRect = SpriteRect::projectile_at(7.0, 4.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_47: SpriteRect = SpriteRect::projectile_at(7.0, 5.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_55: SpriteRect = SpriteRect::projectile_at(7.0, 6.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_63: SpriteRect = SpriteRect::projectile_at(7.0, 7.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_71: SpriteRect = SpriteRect::projectile_at(7.0, 8.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_79: SpriteRect = SpriteRect::projectile_at(7.0, 9.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_87: SpriteRect = SpriteRect::projectile_at(7.0, 10.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_95: SpriteRect = SpriteRect::projectile_at(7.0, 11.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_103: SpriteRect = SpriteRect::projectile_at(7.0, 12.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_111: SpriteRect = SpriteRect::projectile_at(7.0, 13.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_119: SpriteRect = SpriteRect::projectile_at(7.0, 14.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_127: SpriteRect = SpriteRect::projectile_at(7.0, 15.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_135: SpriteRect = SpriteRect::projectile_at(7.0, 16.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_143: SpriteRect = SpriteRect::projectile_at(7.0, 17.0);
#[allow(dead_code)]
pub const SPELLFX_MISSILES_SPRITE_151: SpriteRect = SpriteRect::projectile_at(7.0, 18.0);
