use crate::sprites::SpriteRect;
use common::{Form, PlayerClass, PlayerSnapshot};

//TODO: may need this for shapeshifting, etc
pub enum VisualState {
    Normal { class: PlayerClass, level: u32 },
    PolyMorphed(SpriteRect),
    Invisible,
}

pub fn player_sprite((form, class, level): (&Form, &PlayerClass, u32)) -> &'static SpriteRect {
    match form {
        Form::Normal | Form::Scaled(_) | Form::Invisible => match (class, level) {
            (PlayerClass::Rogue, level) => {
                if level <= 5 {
                    &MONSTERS_SPRITE_878
                } else {
                    &MONSTERS_SPRITE_914
                }
            }
            (PlayerClass::Wizard, level) => {
                if level <= 5 {
                    &MONSTERS_SPRITE_363
                } else {
                    &MONSTERS_SPRITE_339
                }
            }
            (PlayerClass::Druid, level) => {
                if level <= 5 {
                    &MONSTERS_SPRITE_412
                } else {
                    &MONSTERS_SPRITE_448
                }
            }
            (PlayerClass::Ranger, level) => {
                if level <= 5 {
                    &MONSTERS_SPRITE_485
                } else {
                    &MONSTERS_SPRITE_413
                }
            }
            (PlayerClass::Warlock, level) => {
                if level <= 5 {
                    &MONSTERS_SPRITE_245
                } else {
                    &MONSTERS_SPRITE_872
                }
            }
            (PlayerClass::Cleric, level) => {
                if level <= 5 {
                    &MONSTERS_SPRITE_163
                } else {
                    &MONSTERS_SPRITE_164
                }
            }
            (PlayerClass::Sorcerer, level) => {
                if level <= 5 {
                    &MONSTERS_SPRITE_442
                } else {
                    &MONSTERS_SPRITE_406
                }
            }
            (PlayerClass::Fighter, level) => {
                if level <= 5 {
                    &MONSTERS_SPRITE_33
                } else {
                    &MONSTERS_SPRITE_35
                }
            }
            (PlayerClass::Paladin, level) => {
                if level <= 5 {
                    &MONSTERS_SPRITE_198
                } else {
                    &MONSTERS_SPRITE_219
                }
            }
        },
        _ => &MONSTERS_SPRITE_914,
        // Form::Polymorphed(s) => match &s {
        //     "dragon" => VisualState::PolyMorphed(sprite) => sprite
        //
        // }
    }

    // match class {
    //     PlayerClass::Rogue => {
    //         if level <= 5 {
    //             MONSTERS_SPRITE_878
    //         } else {
    //             MONSTERS_SPRITE_914
    //         }
    //     },
    //     PlayerClass::Wizard => {
    //         if level <= 5 {
    //             MONSTERS_SPRITE_39
    //         } else {
    //             MONSTERS_SPRITE_339
    //         }
    //     },
    //     _ => {
    //         MONSTERS_SPRITE_914
    //     }
    // }
}

pub fn enemy_sprites(difficulty: u32) -> [&'static SpriteRect; 3] {
    match difficulty {
        // TODO: more enemy sprites
        1 => [
            &MONSTERS_SPRITE_689,
            &MONSTERS_SPRITE_761,
            &MONSTERS_SPRITE_617,
        ], // spiders
        _ => [&MONSTERS_SPRITE_8, &MONSTERS_SPRITE_8, &MONSTERS_SPRITE_8], // mr skeltal
    }
}

#[allow(dead_code)]
pub const MONSTERS_SPRITE_0: SpriteRect = SpriteRect::at(0, 0);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_36: SpriteRect = SpriteRect::at(0, 1);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_72: SpriteRect = SpriteRect::at(0, 2);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_108: SpriteRect = SpriteRect::at(0, 3);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_144: SpriteRect = SpriteRect::at(0, 4);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_180: SpriteRect = SpriteRect::at(0, 5);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_216: SpriteRect = SpriteRect::at(0, 6);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_252: SpriteRect = SpriteRect::at(0, 7);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_288: SpriteRect = SpriteRect::at(0, 8);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_324: SpriteRect = SpriteRect::at(0, 9);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_360: SpriteRect = SpriteRect::at(0, 10);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_396: SpriteRect = SpriteRect::at(0, 11);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_432: SpriteRect = SpriteRect::at(0, 12);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_468: SpriteRect = SpriteRect::at(0, 13);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_504: SpriteRect = SpriteRect::at(0, 14);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_540: SpriteRect = SpriteRect::at(0, 15);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_576: SpriteRect = SpriteRect::at(0, 16);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_612: SpriteRect = SpriteRect::at(0, 17);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_648: SpriteRect = SpriteRect::at(0, 18);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_684: SpriteRect = SpriteRect::at(0, 19);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_720: SpriteRect = SpriteRect::at(0, 20);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_756: SpriteRect = SpriteRect::at(0, 21);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_792: SpriteRect = SpriteRect::at(0, 22);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_828: SpriteRect = SpriteRect::at(0, 23);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_864: SpriteRect = SpriteRect::at(0, 24);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_900: SpriteRect = SpriteRect::at(0, 25);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_1: SpriteRect = SpriteRect::at(1, 0);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_37: SpriteRect = SpriteRect::at(1, 1);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_73: SpriteRect = SpriteRect::at(1, 2);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_109: SpriteRect = SpriteRect::at(1, 3);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_145: SpriteRect = SpriteRect::at(1, 4);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_181: SpriteRect = SpriteRect::at(1, 5);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_217: SpriteRect = SpriteRect::at(1, 6);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_253: SpriteRect = SpriteRect::at(1, 7);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_289: SpriteRect = SpriteRect::at(1, 8);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_325: SpriteRect = SpriteRect::at(1, 9);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_361: SpriteRect = SpriteRect::at(1, 10);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_397: SpriteRect = SpriteRect::at(1, 11);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_433: SpriteRect = SpriteRect::at(1, 12);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_469: SpriteRect = SpriteRect::at(1, 13);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_505: SpriteRect = SpriteRect::at(1, 14);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_541: SpriteRect = SpriteRect::at(1, 15);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_577: SpriteRect = SpriteRect::at(1, 16);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_613: SpriteRect = SpriteRect::at(1, 17);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_649: SpriteRect = SpriteRect::at(1, 18);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_685: SpriteRect = SpriteRect::at(1, 19);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_721: SpriteRect = SpriteRect::at(1, 20);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_757: SpriteRect = SpriteRect::at(1, 21);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_793: SpriteRect = SpriteRect::at(1, 22);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_829: SpriteRect = SpriteRect::at(1, 23);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_865: SpriteRect = SpriteRect::at(1, 24);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_901: SpriteRect = SpriteRect::at(1, 25);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_2: SpriteRect = SpriteRect::at(2, 0);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_38: SpriteRect = SpriteRect::at(2, 1);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_74: SpriteRect = SpriteRect::at(2, 2);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_110: SpriteRect = SpriteRect::at(2, 3);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_146: SpriteRect = SpriteRect::at(2, 4);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_182: SpriteRect = SpriteRect::at(2, 5);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_218: SpriteRect = SpriteRect::at(2, 6);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_254: SpriteRect = SpriteRect::at(2, 7);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_290: SpriteRect = SpriteRect::at(2, 8);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_326: SpriteRect = SpriteRect::at(2, 9);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_362: SpriteRect = SpriteRect::at(2, 10);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_398: SpriteRect = SpriteRect::at(2, 11);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_434: SpriteRect = SpriteRect::at(2, 12);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_470: SpriteRect = SpriteRect::at(2, 13);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_506: SpriteRect = SpriteRect::at(2, 14);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_542: SpriteRect = SpriteRect::at(2, 15);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_578: SpriteRect = SpriteRect::at(2, 16);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_614: SpriteRect = SpriteRect::at(2, 17);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_650: SpriteRect = SpriteRect::at(2, 18);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_686: SpriteRect = SpriteRect::at(2, 19);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_722: SpriteRect = SpriteRect::at(2, 20);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_758: SpriteRect = SpriteRect::at(2, 21);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_794: SpriteRect = SpriteRect::at(2, 22);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_830: SpriteRect = SpriteRect::at(2, 23);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_866: SpriteRect = SpriteRect::at(2, 24);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_902: SpriteRect = SpriteRect::at(2, 25);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_3: SpriteRect = SpriteRect::at(3, 0);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_39: SpriteRect = SpriteRect::at(3, 1);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_75: SpriteRect = SpriteRect::at(3, 2);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_111: SpriteRect = SpriteRect::at(3, 3);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_147: SpriteRect = SpriteRect::at(3, 4);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_183: SpriteRect = SpriteRect::at(3, 5);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_219: SpriteRect = SpriteRect::at(3, 6);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_255: SpriteRect = SpriteRect::at(3, 7);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_291: SpriteRect = SpriteRect::at(3, 8);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_327: SpriteRect = SpriteRect::at(3, 9);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_363: SpriteRect = SpriteRect::at(3, 10);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_399: SpriteRect = SpriteRect::at(3, 11);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_435: SpriteRect = SpriteRect::at(3, 12);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_471: SpriteRect = SpriteRect::at(3, 13);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_507: SpriteRect = SpriteRect::at(3, 14);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_543: SpriteRect = SpriteRect::at(3, 15);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_579: SpriteRect = SpriteRect::at(3, 16);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_615: SpriteRect = SpriteRect::at(3, 17);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_651: SpriteRect = SpriteRect::at(3, 18);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_687: SpriteRect = SpriteRect::at(3, 19);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_723: SpriteRect = SpriteRect::at(3, 20);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_759: SpriteRect = SpriteRect::at(3, 21);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_795: SpriteRect = SpriteRect::at(3, 22);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_831: SpriteRect = SpriteRect::at(3, 23);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_867: SpriteRect = SpriteRect::at(3, 24);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_903: SpriteRect = SpriteRect::at(3, 25);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_4: SpriteRect = SpriteRect::at(4, 0);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_40: SpriteRect = SpriteRect::at(4, 1);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_76: SpriteRect = SpriteRect::at(4, 2);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_112: SpriteRect = SpriteRect::at(4, 3);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_148: SpriteRect = SpriteRect::at(4, 4);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_184: SpriteRect = SpriteRect::at(4, 5);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_220: SpriteRect = SpriteRect::at(4, 6);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_256: SpriteRect = SpriteRect::at(4, 7);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_292: SpriteRect = SpriteRect::at(4, 8);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_328: SpriteRect = SpriteRect::at(4, 9);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_364: SpriteRect = SpriteRect::at(4, 10);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_400: SpriteRect = SpriteRect::at(4, 11);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_436: SpriteRect = SpriteRect::at(4, 12);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_472: SpriteRect = SpriteRect::at(4, 13);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_508: SpriteRect = SpriteRect::at(4, 14);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_544: SpriteRect = SpriteRect::at(4, 15);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_580: SpriteRect = SpriteRect::at(4, 16);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_616: SpriteRect = SpriteRect::at(4, 17);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_652: SpriteRect = SpriteRect::at(4, 18);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_688: SpriteRect = SpriteRect::at(4, 19);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_724: SpriteRect = SpriteRect::at(4, 20);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_760: SpriteRect = SpriteRect::at(4, 21);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_796: SpriteRect = SpriteRect::at(4, 22);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_832: SpriteRect = SpriteRect::at(4, 23);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_868: SpriteRect = SpriteRect::at(4, 24);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_904: SpriteRect = SpriteRect::at(4, 25);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_5: SpriteRect = SpriteRect::at(5, 0);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_41: SpriteRect = SpriteRect::at(5, 1);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_77: SpriteRect = SpriteRect::at(5, 2);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_113: SpriteRect = SpriteRect::at(5, 3);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_149: SpriteRect = SpriteRect::at(5, 4);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_185: SpriteRect = SpriteRect::at(5, 5);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_221: SpriteRect = SpriteRect::at(5, 6);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_257: SpriteRect = SpriteRect::at(5, 7);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_293: SpriteRect = SpriteRect::at(5, 8);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_329: SpriteRect = SpriteRect::at(5, 9);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_365: SpriteRect = SpriteRect::at(5, 10);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_401: SpriteRect = SpriteRect::at(5, 11);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_437: SpriteRect = SpriteRect::at(5, 12);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_473: SpriteRect = SpriteRect::at(5, 13);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_509: SpriteRect = SpriteRect::at(5, 14);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_545: SpriteRect = SpriteRect::at(5, 15);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_581: SpriteRect = SpriteRect::at(5, 16);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_617: SpriteRect = SpriteRect::at(5, 17);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_653: SpriteRect = SpriteRect::at(5, 18);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_689: SpriteRect = SpriteRect::at(5, 19);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_725: SpriteRect = SpriteRect::at(5, 20);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_761: SpriteRect = SpriteRect::at(5, 21);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_797: SpriteRect = SpriteRect::at(5, 22);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_833: SpriteRect = SpriteRect::at(5, 23);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_869: SpriteRect = SpriteRect::at(5, 24);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_905: SpriteRect = SpriteRect::at(5, 25);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_6: SpriteRect = SpriteRect::at(6, 0);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_42: SpriteRect = SpriteRect::at(6, 1);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_78: SpriteRect = SpriteRect::at(6, 2);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_114: SpriteRect = SpriteRect::at(6, 3);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_150: SpriteRect = SpriteRect::at(6, 4);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_186: SpriteRect = SpriteRect::at(6, 5);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_222: SpriteRect = SpriteRect::at(6, 6);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_258: SpriteRect = SpriteRect::at(6, 7);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_294: SpriteRect = SpriteRect::at(6, 8);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_330: SpriteRect = SpriteRect::at(6, 9);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_366: SpriteRect = SpriteRect::at(6, 10);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_402: SpriteRect = SpriteRect::at(6, 11);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_438: SpriteRect = SpriteRect::at(6, 12);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_474: SpriteRect = SpriteRect::at(6, 13);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_510: SpriteRect = SpriteRect::at(6, 14);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_546: SpriteRect = SpriteRect::at(6, 15);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_582: SpriteRect = SpriteRect::at(6, 16);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_618: SpriteRect = SpriteRect::at(6, 17);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_654: SpriteRect = SpriteRect::at(6, 18);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_690: SpriteRect = SpriteRect::at(6, 19);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_726: SpriteRect = SpriteRect::at(6, 20);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_762: SpriteRect = SpriteRect::at(6, 21);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_798: SpriteRect = SpriteRect::at(6, 22);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_834: SpriteRect = SpriteRect::at(6, 23);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_870: SpriteRect = SpriteRect::at(6, 24);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_906: SpriteRect = SpriteRect::at(6, 25);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_7: SpriteRect = SpriteRect::at(7, 0);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_43: SpriteRect = SpriteRect::at(7, 1);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_79: SpriteRect = SpriteRect::at(7, 2);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_115: SpriteRect = SpriteRect::at(7, 3);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_151: SpriteRect = SpriteRect::at(7, 4);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_187: SpriteRect = SpriteRect::at(7, 5);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_223: SpriteRect = SpriteRect::at(7, 6);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_259: SpriteRect = SpriteRect::at(7, 7);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_295: SpriteRect = SpriteRect::at(7, 8);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_331: SpriteRect = SpriteRect::at(7, 9);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_367: SpriteRect = SpriteRect::at(7, 10);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_403: SpriteRect = SpriteRect::at(7, 11);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_439: SpriteRect = SpriteRect::at(7, 12);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_475: SpriteRect = SpriteRect::at(7, 13);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_511: SpriteRect = SpriteRect::at(7, 14);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_547: SpriteRect = SpriteRect::at(7, 15);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_583: SpriteRect = SpriteRect::at(7, 16);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_619: SpriteRect = SpriteRect::at(7, 17);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_655: SpriteRect = SpriteRect::at(7, 18);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_691: SpriteRect = SpriteRect::at(7, 19);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_727: SpriteRect = SpriteRect::at(7, 20);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_763: SpriteRect = SpriteRect::at(7, 21);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_799: SpriteRect = SpriteRect::at(7, 22);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_835: SpriteRect = SpriteRect::at(7, 23);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_871: SpriteRect = SpriteRect::at(7, 24);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_907: SpriteRect = SpriteRect::at(7, 25);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_8: SpriteRect = SpriteRect::at(8, 0);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_44: SpriteRect = SpriteRect::at(8, 1);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_80: SpriteRect = SpriteRect::at(8, 2);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_116: SpriteRect = SpriteRect::at(8, 3);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_152: SpriteRect = SpriteRect::at(8, 4);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_188: SpriteRect = SpriteRect::at(8, 5);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_224: SpriteRect = SpriteRect::at(8, 6);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_260: SpriteRect = SpriteRect::at(8, 7);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_296: SpriteRect = SpriteRect::at(8, 8);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_332: SpriteRect = SpriteRect::at(8, 9);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_368: SpriteRect = SpriteRect::at(8, 10);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_404: SpriteRect = SpriteRect::at(8, 11);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_440: SpriteRect = SpriteRect::at(8, 12);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_476: SpriteRect = SpriteRect::at(8, 13);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_512: SpriteRect = SpriteRect::at(8, 14);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_548: SpriteRect = SpriteRect::at(8, 15);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_584: SpriteRect = SpriteRect::at(8, 16);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_620: SpriteRect = SpriteRect::at(8, 17);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_656: SpriteRect = SpriteRect::at(8, 18);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_692: SpriteRect = SpriteRect::at(8, 19);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_728: SpriteRect = SpriteRect::at(8, 20);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_764: SpriteRect = SpriteRect::at(8, 21);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_800: SpriteRect = SpriteRect::at(8, 22);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_836: SpriteRect = SpriteRect::at(8, 23);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_872: SpriteRect = SpriteRect::at(8, 24);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_908: SpriteRect = SpriteRect::at(8, 25);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_9: SpriteRect = SpriteRect::at(9, 0);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_45: SpriteRect = SpriteRect::at(9, 1);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_81: SpriteRect = SpriteRect::at(9, 2);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_117: SpriteRect = SpriteRect::at(9, 3);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_153: SpriteRect = SpriteRect::at(9, 4);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_189: SpriteRect = SpriteRect::at(9, 5);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_225: SpriteRect = SpriteRect::at(9, 6);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_261: SpriteRect = SpriteRect::at(9, 7);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_297: SpriteRect = SpriteRect::at(9, 8);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_333: SpriteRect = SpriteRect::at(9, 9);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_369: SpriteRect = SpriteRect::at(9, 10);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_405: SpriteRect = SpriteRect::at(9, 11);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_441: SpriteRect = SpriteRect::at(9, 12);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_477: SpriteRect = SpriteRect::at(9, 13);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_513: SpriteRect = SpriteRect::at(9, 14);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_549: SpriteRect = SpriteRect::at(9, 15);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_585: SpriteRect = SpriteRect::at(9, 16);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_621: SpriteRect = SpriteRect::at(9, 17);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_657: SpriteRect = SpriteRect::at(9, 18);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_693: SpriteRect = SpriteRect::at(9, 19);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_729: SpriteRect = SpriteRect::at(9, 20);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_765: SpriteRect = SpriteRect::at(9, 21);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_801: SpriteRect = SpriteRect::at(9, 22);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_837: SpriteRect = SpriteRect::at(9, 23);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_873: SpriteRect = SpriteRect::at(9, 24);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_909: SpriteRect = SpriteRect::at(9, 25);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_10: SpriteRect = SpriteRect::at(10, 0);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_46: SpriteRect = SpriteRect::at(10, 1);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_82: SpriteRect = SpriteRect::at(10, 2);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_118: SpriteRect = SpriteRect::at(10, 3);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_154: SpriteRect = SpriteRect::at(10, 4);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_190: SpriteRect = SpriteRect::at(10, 5);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_226: SpriteRect = SpriteRect::at(10, 6);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_262: SpriteRect = SpriteRect::at(10, 7);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_298: SpriteRect = SpriteRect::at(10, 8);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_334: SpriteRect = SpriteRect::at(10, 9);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_370: SpriteRect = SpriteRect::at(10, 10);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_406: SpriteRect = SpriteRect::at(10, 11);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_442: SpriteRect = SpriteRect::at(10, 12);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_478: SpriteRect = SpriteRect::at(10, 13);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_514: SpriteRect = SpriteRect::at(10, 14);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_550: SpriteRect = SpriteRect::at(10, 15);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_586: SpriteRect = SpriteRect::at(10, 16);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_622: SpriteRect = SpriteRect::at(10, 17);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_658: SpriteRect = SpriteRect::at(10, 18);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_694: SpriteRect = SpriteRect::at(10, 19);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_730: SpriteRect = SpriteRect::at(10, 20);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_766: SpriteRect = SpriteRect::at(10, 21);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_802: SpriteRect = SpriteRect::at(10, 22);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_838: SpriteRect = SpriteRect::at(10, 23);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_874: SpriteRect = SpriteRect::at(10, 24);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_910: SpriteRect = SpriteRect::at(10, 25);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_11: SpriteRect = SpriteRect::at(11, 0);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_47: SpriteRect = SpriteRect::at(11, 1);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_83: SpriteRect = SpriteRect::at(11, 2);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_119: SpriteRect = SpriteRect::at(11, 3);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_155: SpriteRect = SpriteRect::at(11, 4);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_191: SpriteRect = SpriteRect::at(11, 5);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_227: SpriteRect = SpriteRect::at(11, 6);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_263: SpriteRect = SpriteRect::at(11, 7);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_299: SpriteRect = SpriteRect::at(11, 8);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_335: SpriteRect = SpriteRect::at(11, 9);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_371: SpriteRect = SpriteRect::at(11, 10);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_407: SpriteRect = SpriteRect::at(11, 11);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_443: SpriteRect = SpriteRect::at(11, 12);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_479: SpriteRect = SpriteRect::at(11, 13);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_515: SpriteRect = SpriteRect::at(11, 14);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_551: SpriteRect = SpriteRect::at(11, 15);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_587: SpriteRect = SpriteRect::at(11, 16);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_623: SpriteRect = SpriteRect::at(11, 17);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_659: SpriteRect = SpriteRect::at(11, 18);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_695: SpriteRect = SpriteRect::at(11, 19);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_731: SpriteRect = SpriteRect::at(11, 20);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_767: SpriteRect = SpriteRect::at(11, 21);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_803: SpriteRect = SpriteRect::at(11, 22);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_839: SpriteRect = SpriteRect::at(11, 23);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_875: SpriteRect = SpriteRect::at(11, 24);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_911: SpriteRect = SpriteRect::at(11, 25);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_12: SpriteRect = SpriteRect::at(12, 0);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_48: SpriteRect = SpriteRect::at(12, 1);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_84: SpriteRect = SpriteRect::at(12, 2);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_120: SpriteRect = SpriteRect::at(12, 3);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_156: SpriteRect = SpriteRect::at(12, 4);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_192: SpriteRect = SpriteRect::at(12, 5);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_228: SpriteRect = SpriteRect::at(12, 6);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_264: SpriteRect = SpriteRect::at(12, 7);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_300: SpriteRect = SpriteRect::at(12, 8);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_336: SpriteRect = SpriteRect::at(12, 9);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_372: SpriteRect = SpriteRect::at(12, 10);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_408: SpriteRect = SpriteRect::at(12, 11);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_444: SpriteRect = SpriteRect::at(12, 12);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_480: SpriteRect = SpriteRect::at(12, 13);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_516: SpriteRect = SpriteRect::at(12, 14);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_552: SpriteRect = SpriteRect::at(12, 15);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_588: SpriteRect = SpriteRect::at(12, 16);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_624: SpriteRect = SpriteRect::at(12, 17);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_660: SpriteRect = SpriteRect::at(12, 18);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_696: SpriteRect = SpriteRect::at(12, 19);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_732: SpriteRect = SpriteRect::at(12, 20);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_768: SpriteRect = SpriteRect::at(12, 21);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_804: SpriteRect = SpriteRect::at(12, 22);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_840: SpriteRect = SpriteRect::at(12, 23);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_876: SpriteRect = SpriteRect::at(12, 24);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_912: SpriteRect = SpriteRect::at(12, 25);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_13: SpriteRect = SpriteRect::at(13, 0);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_49: SpriteRect = SpriteRect::at(13, 1);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_85: SpriteRect = SpriteRect::at(13, 2);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_121: SpriteRect = SpriteRect::at(13, 3);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_157: SpriteRect = SpriteRect::at(13, 4);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_193: SpriteRect = SpriteRect::at(13, 5);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_229: SpriteRect = SpriteRect::at(13, 6);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_265: SpriteRect = SpriteRect::at(13, 7);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_301: SpriteRect = SpriteRect::at(13, 8);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_337: SpriteRect = SpriteRect::at(13, 9);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_373: SpriteRect = SpriteRect::at(13, 10);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_409: SpriteRect = SpriteRect::at(13, 11);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_445: SpriteRect = SpriteRect::at(13, 12);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_481: SpriteRect = SpriteRect::at(13, 13);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_517: SpriteRect = SpriteRect::at(13, 14);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_553: SpriteRect = SpriteRect::at(13, 15);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_589: SpriteRect = SpriteRect::at(13, 16);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_625: SpriteRect = SpriteRect::at(13, 17);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_661: SpriteRect = SpriteRect::at(13, 18);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_697: SpriteRect = SpriteRect::at(13, 19);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_733: SpriteRect = SpriteRect::at(13, 20);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_769: SpriteRect = SpriteRect::at(13, 21);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_805: SpriteRect = SpriteRect::at(13, 22);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_841: SpriteRect = SpriteRect::at(13, 23);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_877: SpriteRect = SpriteRect::at(13, 24);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_913: SpriteRect = SpriteRect::at(13, 25);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_14: SpriteRect = SpriteRect::at(14, 0);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_50: SpriteRect = SpriteRect::at(14, 1);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_86: SpriteRect = SpriteRect::at(14, 2);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_122: SpriteRect = SpriteRect::at(14, 3);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_158: SpriteRect = SpriteRect::at(14, 4);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_194: SpriteRect = SpriteRect::at(14, 5);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_230: SpriteRect = SpriteRect::at(14, 6);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_266: SpriteRect = SpriteRect::at(14, 7);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_302: SpriteRect = SpriteRect::at(14, 8);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_338: SpriteRect = SpriteRect::at(14, 9);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_374: SpriteRect = SpriteRect::at(14, 10);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_410: SpriteRect = SpriteRect::at(14, 11);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_446: SpriteRect = SpriteRect::at(14, 12);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_482: SpriteRect = SpriteRect::at(14, 13);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_518: SpriteRect = SpriteRect::at(14, 14);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_554: SpriteRect = SpriteRect::at(14, 15);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_590: SpriteRect = SpriteRect::at(14, 16);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_626: SpriteRect = SpriteRect::at(14, 17);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_662: SpriteRect = SpriteRect::at(14, 18);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_698: SpriteRect = SpriteRect::at(14, 19);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_734: SpriteRect = SpriteRect::at(14, 20);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_770: SpriteRect = SpriteRect::at(14, 21);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_806: SpriteRect = SpriteRect::at(14, 22);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_842: SpriteRect = SpriteRect::at(14, 23);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_878: SpriteRect = SpriteRect::at(14, 24);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_914: SpriteRect = SpriteRect::at(14, 25);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_15: SpriteRect = SpriteRect::at(15, 0);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_51: SpriteRect = SpriteRect::at(15, 1);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_87: SpriteRect = SpriteRect::at(15, 2);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_123: SpriteRect = SpriteRect::at(15, 3);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_159: SpriteRect = SpriteRect::at(15, 4);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_195: SpriteRect = SpriteRect::at(15, 5);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_231: SpriteRect = SpriteRect::at(15, 6);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_267: SpriteRect = SpriteRect::at(15, 7);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_303: SpriteRect = SpriteRect::at(15, 8);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_339: SpriteRect = SpriteRect::at(15, 9);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_375: SpriteRect = SpriteRect::at(15, 10);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_411: SpriteRect = SpriteRect::at(15, 11);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_447: SpriteRect = SpriteRect::at(15, 12);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_483: SpriteRect = SpriteRect::at(15, 13);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_519: SpriteRect = SpriteRect::at(15, 14);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_555: SpriteRect = SpriteRect::at(15, 15);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_591: SpriteRect = SpriteRect::at(15, 16);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_627: SpriteRect = SpriteRect::at(15, 17);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_663: SpriteRect = SpriteRect::at(15, 18);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_699: SpriteRect = SpriteRect::at(15, 19);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_735: SpriteRect = SpriteRect::at(15, 20);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_771: SpriteRect = SpriteRect::at(15, 21);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_807: SpriteRect = SpriteRect::at(15, 22);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_843: SpriteRect = SpriteRect::at(15, 23);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_879: SpriteRect = SpriteRect::at(15, 24);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_915: SpriteRect = SpriteRect::at(15, 25);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_16: SpriteRect = SpriteRect::at(16, 0);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_52: SpriteRect = SpriteRect::at(16, 1);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_88: SpriteRect = SpriteRect::at(16, 2);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_124: SpriteRect = SpriteRect::at(16, 3);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_160: SpriteRect = SpriteRect::at(16, 4);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_196: SpriteRect = SpriteRect::at(16, 5);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_232: SpriteRect = SpriteRect::at(16, 6);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_268: SpriteRect = SpriteRect::at(16, 7);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_304: SpriteRect = SpriteRect::at(16, 8);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_340: SpriteRect = SpriteRect::at(16, 9);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_376: SpriteRect = SpriteRect::at(16, 10);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_412: SpriteRect = SpriteRect::at(16, 11);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_448: SpriteRect = SpriteRect::at(16, 12);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_484: SpriteRect = SpriteRect::at(16, 13);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_520: SpriteRect = SpriteRect::at(16, 14);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_556: SpriteRect = SpriteRect::at(16, 15);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_592: SpriteRect = SpriteRect::at(16, 16);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_628: SpriteRect = SpriteRect::at(16, 17);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_664: SpriteRect = SpriteRect::at(16, 18);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_700: SpriteRect = SpriteRect::at(16, 19);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_736: SpriteRect = SpriteRect::at(16, 20);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_772: SpriteRect = SpriteRect::at(16, 21);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_808: SpriteRect = SpriteRect::at(16, 22);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_844: SpriteRect = SpriteRect::at(16, 23);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_880: SpriteRect = SpriteRect::at(16, 24);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_916: SpriteRect = SpriteRect::at(16, 25);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_17: SpriteRect = SpriteRect::at(17, 0);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_53: SpriteRect = SpriteRect::at(17, 1);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_89: SpriteRect = SpriteRect::at(17, 2);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_125: SpriteRect = SpriteRect::at(17, 3);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_161: SpriteRect = SpriteRect::at(17, 4);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_197: SpriteRect = SpriteRect::at(17, 5);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_233: SpriteRect = SpriteRect::at(17, 6);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_269: SpriteRect = SpriteRect::at(17, 7);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_305: SpriteRect = SpriteRect::at(17, 8);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_341: SpriteRect = SpriteRect::at(17, 9);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_377: SpriteRect = SpriteRect::at(17, 10);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_413: SpriteRect = SpriteRect::at(17, 11);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_449: SpriteRect = SpriteRect::at(17, 12);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_485: SpriteRect = SpriteRect::at(17, 13);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_521: SpriteRect = SpriteRect::at(17, 14);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_557: SpriteRect = SpriteRect::at(17, 15);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_593: SpriteRect = SpriteRect::at(17, 16);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_629: SpriteRect = SpriteRect::at(17, 17);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_665: SpriteRect = SpriteRect::at(17, 18);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_701: SpriteRect = SpriteRect::at(17, 19);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_737: SpriteRect = SpriteRect::at(17, 20);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_773: SpriteRect = SpriteRect::at(17, 21);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_809: SpriteRect = SpriteRect::at(17, 22);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_845: SpriteRect = SpriteRect::at(17, 23);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_881: SpriteRect = SpriteRect::at(17, 24);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_917: SpriteRect = SpriteRect::at(17, 25);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_18: SpriteRect = SpriteRect::at(18, 0);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_54: SpriteRect = SpriteRect::at(18, 1);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_90: SpriteRect = SpriteRect::at(18, 2);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_126: SpriteRect = SpriteRect::at(18, 3);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_162: SpriteRect = SpriteRect::at(18, 4);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_198: SpriteRect = SpriteRect::at(18, 5);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_234: SpriteRect = SpriteRect::at(18, 6);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_270: SpriteRect = SpriteRect::at(18, 7);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_306: SpriteRect = SpriteRect::at(18, 8);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_342: SpriteRect = SpriteRect::at(18, 9);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_378: SpriteRect = SpriteRect::at(18, 10);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_414: SpriteRect = SpriteRect::at(18, 11);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_450: SpriteRect = SpriteRect::at(18, 12);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_486: SpriteRect = SpriteRect::at(18, 13);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_522: SpriteRect = SpriteRect::at(18, 14);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_558: SpriteRect = SpriteRect::at(18, 15);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_594: SpriteRect = SpriteRect::at(18, 16);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_630: SpriteRect = SpriteRect::at(18, 17);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_666: SpriteRect = SpriteRect::at(18, 18);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_702: SpriteRect = SpriteRect::at(18, 19);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_738: SpriteRect = SpriteRect::at(18, 20);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_774: SpriteRect = SpriteRect::at(18, 21);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_810: SpriteRect = SpriteRect::at(18, 22);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_846: SpriteRect = SpriteRect::at(18, 23);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_882: SpriteRect = SpriteRect::at(18, 24);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_918: SpriteRect = SpriteRect::at(18, 25);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_19: SpriteRect = SpriteRect::at(19, 0);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_55: SpriteRect = SpriteRect::at(19, 1);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_91: SpriteRect = SpriteRect::at(19, 2);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_127: SpriteRect = SpriteRect::at(19, 3);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_163: SpriteRect = SpriteRect::at(19, 4);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_199: SpriteRect = SpriteRect::at(19, 5);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_235: SpriteRect = SpriteRect::at(19, 6);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_271: SpriteRect = SpriteRect::at(19, 7);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_307: SpriteRect = SpriteRect::at(19, 8);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_343: SpriteRect = SpriteRect::at(19, 9);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_379: SpriteRect = SpriteRect::at(19, 10);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_415: SpriteRect = SpriteRect::at(19, 11);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_451: SpriteRect = SpriteRect::at(19, 12);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_487: SpriteRect = SpriteRect::at(19, 13);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_523: SpriteRect = SpriteRect::at(19, 14);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_559: SpriteRect = SpriteRect::at(19, 15);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_595: SpriteRect = SpriteRect::at(19, 16);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_631: SpriteRect = SpriteRect::at(19, 17);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_667: SpriteRect = SpriteRect::at(19, 18);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_703: SpriteRect = SpriteRect::at(19, 19);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_739: SpriteRect = SpriteRect::at(19, 20);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_775: SpriteRect = SpriteRect::at(19, 21);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_811: SpriteRect = SpriteRect::at(19, 22);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_847: SpriteRect = SpriteRect::at(19, 23);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_883: SpriteRect = SpriteRect::at(19, 24);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_919: SpriteRect = SpriteRect::at(19, 25);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_20: SpriteRect = SpriteRect::at(20, 0);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_56: SpriteRect = SpriteRect::at(20, 1);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_92: SpriteRect = SpriteRect::at(20, 2);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_128: SpriteRect = SpriteRect::at(20, 3);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_164: SpriteRect = SpriteRect::at(20, 4);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_200: SpriteRect = SpriteRect::at(20, 5);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_236: SpriteRect = SpriteRect::at(20, 6);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_272: SpriteRect = SpriteRect::at(20, 7);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_308: SpriteRect = SpriteRect::at(20, 8);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_344: SpriteRect = SpriteRect::at(20, 9);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_380: SpriteRect = SpriteRect::at(20, 10);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_416: SpriteRect = SpriteRect::at(20, 11);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_452: SpriteRect = SpriteRect::at(20, 12);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_488: SpriteRect = SpriteRect::at(20, 13);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_524: SpriteRect = SpriteRect::at(20, 14);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_560: SpriteRect = SpriteRect::at(20, 15);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_596: SpriteRect = SpriteRect::at(20, 16);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_632: SpriteRect = SpriteRect::at(20, 17);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_668: SpriteRect = SpriteRect::at(20, 18);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_704: SpriteRect = SpriteRect::at(20, 19);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_740: SpriteRect = SpriteRect::at(20, 20);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_776: SpriteRect = SpriteRect::at(20, 21);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_812: SpriteRect = SpriteRect::at(20, 22);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_848: SpriteRect = SpriteRect::at(20, 23);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_884: SpriteRect = SpriteRect::at(20, 24);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_920: SpriteRect = SpriteRect::at(20, 25);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_21: SpriteRect = SpriteRect::at(21, 0);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_57: SpriteRect = SpriteRect::at(21, 1);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_93: SpriteRect = SpriteRect::at(21, 2);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_129: SpriteRect = SpriteRect::at(21, 3);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_165: SpriteRect = SpriteRect::at(21, 4);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_201: SpriteRect = SpriteRect::at(21, 5);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_237: SpriteRect = SpriteRect::at(21, 6);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_273: SpriteRect = SpriteRect::at(21, 7);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_309: SpriteRect = SpriteRect::at(21, 8);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_345: SpriteRect = SpriteRect::at(21, 9);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_381: SpriteRect = SpriteRect::at(21, 10);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_417: SpriteRect = SpriteRect::at(21, 11);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_453: SpriteRect = SpriteRect::at(21, 12);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_489: SpriteRect = SpriteRect::at(21, 13);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_525: SpriteRect = SpriteRect::at(21, 14);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_561: SpriteRect = SpriteRect::at(21, 15);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_597: SpriteRect = SpriteRect::at(21, 16);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_633: SpriteRect = SpriteRect::at(21, 17);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_669: SpriteRect = SpriteRect::at(21, 18);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_705: SpriteRect = SpriteRect::at(21, 19);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_741: SpriteRect = SpriteRect::at(21, 20);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_777: SpriteRect = SpriteRect::at(21, 21);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_813: SpriteRect = SpriteRect::at(21, 22);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_849: SpriteRect = SpriteRect::at(21, 23);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_885: SpriteRect = SpriteRect::at(21, 24);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_921: SpriteRect = SpriteRect::at(21, 25);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_22: SpriteRect = SpriteRect::at(22, 0);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_58: SpriteRect = SpriteRect::at(22, 1);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_94: SpriteRect = SpriteRect::at(22, 2);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_130: SpriteRect = SpriteRect::at(22, 3);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_166: SpriteRect = SpriteRect::at(22, 4);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_202: SpriteRect = SpriteRect::at(22, 5);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_238: SpriteRect = SpriteRect::at(22, 6);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_274: SpriteRect = SpriteRect::at(22, 7);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_310: SpriteRect = SpriteRect::at(22, 8);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_346: SpriteRect = SpriteRect::at(22, 9);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_382: SpriteRect = SpriteRect::at(22, 10);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_418: SpriteRect = SpriteRect::at(22, 11);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_454: SpriteRect = SpriteRect::at(22, 12);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_490: SpriteRect = SpriteRect::at(22, 13);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_526: SpriteRect = SpriteRect::at(22, 14);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_562: SpriteRect = SpriteRect::at(22, 15);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_598: SpriteRect = SpriteRect::at(22, 16);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_634: SpriteRect = SpriteRect::at(22, 17);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_670: SpriteRect = SpriteRect::at(22, 18);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_706: SpriteRect = SpriteRect::at(22, 19);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_742: SpriteRect = SpriteRect::at(22, 20);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_778: SpriteRect = SpriteRect::at(22, 21);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_814: SpriteRect = SpriteRect::at(22, 22);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_850: SpriteRect = SpriteRect::at(22, 23);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_886: SpriteRect = SpriteRect::at(22, 24);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_922: SpriteRect = SpriteRect::at(22, 25);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_23: SpriteRect = SpriteRect::at(23, 0);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_59: SpriteRect = SpriteRect::at(23, 1);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_95: SpriteRect = SpriteRect::at(23, 2);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_131: SpriteRect = SpriteRect::at(23, 3);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_167: SpriteRect = SpriteRect::at(23, 4);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_203: SpriteRect = SpriteRect::at(23, 5);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_239: SpriteRect = SpriteRect::at(23, 6);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_275: SpriteRect = SpriteRect::at(23, 7);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_311: SpriteRect = SpriteRect::at(23, 8);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_347: SpriteRect = SpriteRect::at(23, 9);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_383: SpriteRect = SpriteRect::at(23, 10);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_419: SpriteRect = SpriteRect::at(23, 11);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_455: SpriteRect = SpriteRect::at(23, 12);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_491: SpriteRect = SpriteRect::at(23, 13);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_527: SpriteRect = SpriteRect::at(23, 14);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_563: SpriteRect = SpriteRect::at(23, 15);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_599: SpriteRect = SpriteRect::at(23, 16);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_635: SpriteRect = SpriteRect::at(23, 17);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_671: SpriteRect = SpriteRect::at(23, 18);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_707: SpriteRect = SpriteRect::at(23, 19);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_743: SpriteRect = SpriteRect::at(23, 20);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_779: SpriteRect = SpriteRect::at(23, 21);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_815: SpriteRect = SpriteRect::at(23, 22);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_851: SpriteRect = SpriteRect::at(23, 23);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_887: SpriteRect = SpriteRect::at(23, 24);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_923: SpriteRect = SpriteRect::at(23, 25);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_24: SpriteRect = SpriteRect::at(24, 0);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_60: SpriteRect = SpriteRect::at(24, 1);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_96: SpriteRect = SpriteRect::at(24, 2);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_132: SpriteRect = SpriteRect::at(24, 3);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_168: SpriteRect = SpriteRect::at(24, 4);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_204: SpriteRect = SpriteRect::at(24, 5);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_240: SpriteRect = SpriteRect::at(24, 6);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_276: SpriteRect = SpriteRect::at(24, 7);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_312: SpriteRect = SpriteRect::at(24, 8);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_348: SpriteRect = SpriteRect::at(24, 9);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_384: SpriteRect = SpriteRect::at(24, 10);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_420: SpriteRect = SpriteRect::at(24, 11);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_456: SpriteRect = SpriteRect::at(24, 12);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_492: SpriteRect = SpriteRect::at(24, 13);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_528: SpriteRect = SpriteRect::at(24, 14);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_564: SpriteRect = SpriteRect::at(24, 15);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_600: SpriteRect = SpriteRect::at(24, 16);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_636: SpriteRect = SpriteRect::at(24, 17);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_672: SpriteRect = SpriteRect::at(24, 18);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_708: SpriteRect = SpriteRect::at(24, 19);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_744: SpriteRect = SpriteRect::at(24, 20);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_780: SpriteRect = SpriteRect::at(24, 21);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_816: SpriteRect = SpriteRect::at(24, 22);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_852: SpriteRect = SpriteRect::at(24, 23);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_888: SpriteRect = SpriteRect::at(24, 24);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_924: SpriteRect = SpriteRect::at(24, 25);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_25: SpriteRect = SpriteRect::at(25, 0);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_61: SpriteRect = SpriteRect::at(25, 1);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_97: SpriteRect = SpriteRect::at(25, 2);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_133: SpriteRect = SpriteRect::at(25, 3);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_169: SpriteRect = SpriteRect::at(25, 4);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_205: SpriteRect = SpriteRect::at(25, 5);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_241: SpriteRect = SpriteRect::at(25, 6);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_277: SpriteRect = SpriteRect::at(25, 7);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_313: SpriteRect = SpriteRect::at(25, 8);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_349: SpriteRect = SpriteRect::at(25, 9);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_385: SpriteRect = SpriteRect::at(25, 10);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_421: SpriteRect = SpriteRect::at(25, 11);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_457: SpriteRect = SpriteRect::at(25, 12);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_493: SpriteRect = SpriteRect::at(25, 13);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_529: SpriteRect = SpriteRect::at(25, 14);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_565: SpriteRect = SpriteRect::at(25, 15);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_601: SpriteRect = SpriteRect::at(25, 16);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_637: SpriteRect = SpriteRect::at(25, 17);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_673: SpriteRect = SpriteRect::at(25, 18);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_709: SpriteRect = SpriteRect::at(25, 19);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_745: SpriteRect = SpriteRect::at(25, 20);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_781: SpriteRect = SpriteRect::at(25, 21);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_817: SpriteRect = SpriteRect::at(25, 22);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_853: SpriteRect = SpriteRect::at(25, 23);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_889: SpriteRect = SpriteRect::at(25, 24);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_925: SpriteRect = SpriteRect::at(25, 25);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_26: SpriteRect = SpriteRect::at(26, 0);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_62: SpriteRect = SpriteRect::at(26, 1);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_98: SpriteRect = SpriteRect::at(26, 2);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_134: SpriteRect = SpriteRect::at(26, 3);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_170: SpriteRect = SpriteRect::at(26, 4);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_206: SpriteRect = SpriteRect::at(26, 5);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_242: SpriteRect = SpriteRect::at(26, 6);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_278: SpriteRect = SpriteRect::at(26, 7);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_314: SpriteRect = SpriteRect::at(26, 8);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_350: SpriteRect = SpriteRect::at(26, 9);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_386: SpriteRect = SpriteRect::at(26, 10);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_422: SpriteRect = SpriteRect::at(26, 11);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_458: SpriteRect = SpriteRect::at(26, 12);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_494: SpriteRect = SpriteRect::at(26, 13);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_530: SpriteRect = SpriteRect::at(26, 14);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_566: SpriteRect = SpriteRect::at(26, 15);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_602: SpriteRect = SpriteRect::at(26, 16);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_638: SpriteRect = SpriteRect::at(26, 17);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_674: SpriteRect = SpriteRect::at(26, 18);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_710: SpriteRect = SpriteRect::at(26, 19);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_746: SpriteRect = SpriteRect::at(26, 20);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_782: SpriteRect = SpriteRect::at(26, 21);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_818: SpriteRect = SpriteRect::at(26, 22);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_854: SpriteRect = SpriteRect::at(26, 23);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_890: SpriteRect = SpriteRect::at(26, 24);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_926: SpriteRect = SpriteRect::at(26, 25);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_27: SpriteRect = SpriteRect::at(27, 0);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_63: SpriteRect = SpriteRect::at(27, 1);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_99: SpriteRect = SpriteRect::at(27, 2);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_135: SpriteRect = SpriteRect::at(27, 3);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_171: SpriteRect = SpriteRect::at(27, 4);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_207: SpriteRect = SpriteRect::at(27, 5);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_243: SpriteRect = SpriteRect::at(27, 6);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_279: SpriteRect = SpriteRect::at(27, 7);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_315: SpriteRect = SpriteRect::at(27, 8);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_351: SpriteRect = SpriteRect::at(27, 9);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_387: SpriteRect = SpriteRect::at(27, 10);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_423: SpriteRect = SpriteRect::at(27, 11);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_459: SpriteRect = SpriteRect::at(27, 12);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_495: SpriteRect = SpriteRect::at(27, 13);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_531: SpriteRect = SpriteRect::at(27, 14);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_567: SpriteRect = SpriteRect::at(27, 15);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_603: SpriteRect = SpriteRect::at(27, 16);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_639: SpriteRect = SpriteRect::at(27, 17);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_675: SpriteRect = SpriteRect::at(27, 18);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_711: SpriteRect = SpriteRect::at(27, 19);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_747: SpriteRect = SpriteRect::at(27, 20);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_783: SpriteRect = SpriteRect::at(27, 21);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_819: SpriteRect = SpriteRect::at(27, 22);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_855: SpriteRect = SpriteRect::at(27, 23);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_891: SpriteRect = SpriteRect::at(27, 24);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_927: SpriteRect = SpriteRect::at(27, 25);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_28: SpriteRect = SpriteRect::at(28, 0);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_64: SpriteRect = SpriteRect::at(28, 1);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_100: SpriteRect = SpriteRect::at(28, 2);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_136: SpriteRect = SpriteRect::at(28, 3);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_172: SpriteRect = SpriteRect::at(28, 4);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_208: SpriteRect = SpriteRect::at(28, 5);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_244: SpriteRect = SpriteRect::at(28, 6);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_280: SpriteRect = SpriteRect::at(28, 7);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_316: SpriteRect = SpriteRect::at(28, 8);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_352: SpriteRect = SpriteRect::at(28, 9);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_388: SpriteRect = SpriteRect::at(28, 10);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_424: SpriteRect = SpriteRect::at(28, 11);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_460: SpriteRect = SpriteRect::at(28, 12);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_496: SpriteRect = SpriteRect::at(28, 13);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_532: SpriteRect = SpriteRect::at(28, 14);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_568: SpriteRect = SpriteRect::at(28, 15);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_604: SpriteRect = SpriteRect::at(28, 16);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_640: SpriteRect = SpriteRect::at(28, 17);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_676: SpriteRect = SpriteRect::at(28, 18);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_712: SpriteRect = SpriteRect::at(28, 19);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_748: SpriteRect = SpriteRect::at(28, 20);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_784: SpriteRect = SpriteRect::at(28, 21);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_820: SpriteRect = SpriteRect::at(28, 22);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_856: SpriteRect = SpriteRect::at(28, 23);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_892: SpriteRect = SpriteRect::at(28, 24);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_928: SpriteRect = SpriteRect::at(28, 25);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_29: SpriteRect = SpriteRect::at(29, 0);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_65: SpriteRect = SpriteRect::at(29, 1);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_101: SpriteRect = SpriteRect::at(29, 2);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_137: SpriteRect = SpriteRect::at(29, 3);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_173: SpriteRect = SpriteRect::at(29, 4);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_209: SpriteRect = SpriteRect::at(29, 5);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_245: SpriteRect = SpriteRect::at(29, 6);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_281: SpriteRect = SpriteRect::at(29, 7);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_317: SpriteRect = SpriteRect::at(29, 8);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_353: SpriteRect = SpriteRect::at(29, 9);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_389: SpriteRect = SpriteRect::at(29, 10);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_425: SpriteRect = SpriteRect::at(29, 11);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_461: SpriteRect = SpriteRect::at(29, 12);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_497: SpriteRect = SpriteRect::at(29, 13);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_533: SpriteRect = SpriteRect::at(29, 14);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_569: SpriteRect = SpriteRect::at(29, 15);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_605: SpriteRect = SpriteRect::at(29, 16);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_641: SpriteRect = SpriteRect::at(29, 17);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_677: SpriteRect = SpriteRect::at(29, 18);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_713: SpriteRect = SpriteRect::at(29, 19);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_749: SpriteRect = SpriteRect::at(29, 20);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_785: SpriteRect = SpriteRect::at(29, 21);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_821: SpriteRect = SpriteRect::at(29, 22);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_857: SpriteRect = SpriteRect::at(29, 23);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_893: SpriteRect = SpriteRect::at(29, 24);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_929: SpriteRect = SpriteRect::at(29, 25);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_30: SpriteRect = SpriteRect::at(30, 0);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_66: SpriteRect = SpriteRect::at(30, 1);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_102: SpriteRect = SpriteRect::at(30, 2);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_138: SpriteRect = SpriteRect::at(30, 3);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_174: SpriteRect = SpriteRect::at(30, 4);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_210: SpriteRect = SpriteRect::at(30, 5);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_246: SpriteRect = SpriteRect::at(30, 6);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_282: SpriteRect = SpriteRect::at(30, 7);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_318: SpriteRect = SpriteRect::at(30, 8);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_354: SpriteRect = SpriteRect::at(30, 9);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_390: SpriteRect = SpriteRect::at(30, 10);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_426: SpriteRect = SpriteRect::at(30, 11);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_462: SpriteRect = SpriteRect::at(30, 12);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_498: SpriteRect = SpriteRect::at(30, 13);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_534: SpriteRect = SpriteRect::at(30, 14);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_570: SpriteRect = SpriteRect::at(30, 15);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_606: SpriteRect = SpriteRect::at(30, 16);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_642: SpriteRect = SpriteRect::at(30, 17);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_678: SpriteRect = SpriteRect::at(30, 18);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_714: SpriteRect = SpriteRect::at(30, 19);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_750: SpriteRect = SpriteRect::at(30, 20);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_786: SpriteRect = SpriteRect::at(30, 21);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_822: SpriteRect = SpriteRect::at(30, 22);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_858: SpriteRect = SpriteRect::at(30, 23);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_894: SpriteRect = SpriteRect::at(30, 24);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_930: SpriteRect = SpriteRect::at(30, 25);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_31: SpriteRect = SpriteRect::at(31, 0);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_67: SpriteRect = SpriteRect::at(31, 1);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_103: SpriteRect = SpriteRect::at(31, 2);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_139: SpriteRect = SpriteRect::at(31, 3);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_175: SpriteRect = SpriteRect::at(31, 4);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_211: SpriteRect = SpriteRect::at(31, 5);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_247: SpriteRect = SpriteRect::at(31, 6);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_283: SpriteRect = SpriteRect::at(31, 7);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_319: SpriteRect = SpriteRect::at(31, 8);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_355: SpriteRect = SpriteRect::at(31, 9);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_391: SpriteRect = SpriteRect::at(31, 10);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_427: SpriteRect = SpriteRect::at(31, 11);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_463: SpriteRect = SpriteRect::at(31, 12);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_499: SpriteRect = SpriteRect::at(31, 13);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_535: SpriteRect = SpriteRect::at(31, 14);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_571: SpriteRect = SpriteRect::at(31, 15);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_607: SpriteRect = SpriteRect::at(31, 16);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_643: SpriteRect = SpriteRect::at(31, 17);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_679: SpriteRect = SpriteRect::at(31, 18);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_715: SpriteRect = SpriteRect::at(31, 19);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_751: SpriteRect = SpriteRect::at(31, 20);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_787: SpriteRect = SpriteRect::at(31, 21);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_823: SpriteRect = SpriteRect::at(31, 22);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_859: SpriteRect = SpriteRect::at(31, 23);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_895: SpriteRect = SpriteRect::at(31, 24);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_931: SpriteRect = SpriteRect::at(31, 25);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_32: SpriteRect = SpriteRect::at(32, 0);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_68: SpriteRect = SpriteRect::at(32, 1);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_104: SpriteRect = SpriteRect::at(32, 2);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_140: SpriteRect = SpriteRect::at(32, 3);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_176: SpriteRect = SpriteRect::at(32, 4);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_212: SpriteRect = SpriteRect::at(32, 5);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_248: SpriteRect = SpriteRect::at(32, 6);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_284: SpriteRect = SpriteRect::at(32, 7);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_320: SpriteRect = SpriteRect::at(32, 8);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_356: SpriteRect = SpriteRect::at(32, 9);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_392: SpriteRect = SpriteRect::at(32, 10);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_428: SpriteRect = SpriteRect::at(32, 11);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_464: SpriteRect = SpriteRect::at(32, 12);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_500: SpriteRect = SpriteRect::at(32, 13);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_536: SpriteRect = SpriteRect::at(32, 14);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_572: SpriteRect = SpriteRect::at(32, 15);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_608: SpriteRect = SpriteRect::at(32, 16);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_644: SpriteRect = SpriteRect::at(32, 17);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_680: SpriteRect = SpriteRect::at(32, 18);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_716: SpriteRect = SpriteRect::at(32, 19);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_752: SpriteRect = SpriteRect::at(32, 20);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_788: SpriteRect = SpriteRect::at(32, 21);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_824: SpriteRect = SpriteRect::at(32, 22);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_860: SpriteRect = SpriteRect::at(32, 23);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_896: SpriteRect = SpriteRect::at(32, 24);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_932: SpriteRect = SpriteRect::at(32, 25);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_33: SpriteRect = SpriteRect::at(33, 0);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_69: SpriteRect = SpriteRect::at(33, 1);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_105: SpriteRect = SpriteRect::at(33, 2);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_141: SpriteRect = SpriteRect::at(33, 3);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_177: SpriteRect = SpriteRect::at(33, 4);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_213: SpriteRect = SpriteRect::at(33, 5);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_249: SpriteRect = SpriteRect::at(33, 6);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_285: SpriteRect = SpriteRect::at(33, 7);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_321: SpriteRect = SpriteRect::at(33, 8);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_357: SpriteRect = SpriteRect::at(33, 9);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_393: SpriteRect = SpriteRect::at(33, 10);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_429: SpriteRect = SpriteRect::at(33, 11);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_465: SpriteRect = SpriteRect::at(33, 12);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_501: SpriteRect = SpriteRect::at(33, 13);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_537: SpriteRect = SpriteRect::at(33, 14);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_573: SpriteRect = SpriteRect::at(33, 15);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_609: SpriteRect = SpriteRect::at(33, 16);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_645: SpriteRect = SpriteRect::at(33, 17);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_681: SpriteRect = SpriteRect::at(33, 18);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_717: SpriteRect = SpriteRect::at(33, 19);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_753: SpriteRect = SpriteRect::at(33, 20);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_789: SpriteRect = SpriteRect::at(33, 21);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_825: SpriteRect = SpriteRect::at(33, 22);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_861: SpriteRect = SpriteRect::at(33, 23);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_897: SpriteRect = SpriteRect::at(33, 24);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_933: SpriteRect = SpriteRect::at(33, 25);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_34: SpriteRect = SpriteRect::at(34, 0);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_70: SpriteRect = SpriteRect::at(34, 1);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_106: SpriteRect = SpriteRect::at(34, 2);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_142: SpriteRect = SpriteRect::at(34, 3);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_178: SpriteRect = SpriteRect::at(34, 4);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_214: SpriteRect = SpriteRect::at(34, 5);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_250: SpriteRect = SpriteRect::at(34, 6);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_286: SpriteRect = SpriteRect::at(34, 7);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_322: SpriteRect = SpriteRect::at(34, 8);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_358: SpriteRect = SpriteRect::at(34, 9);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_394: SpriteRect = SpriteRect::at(34, 10);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_430: SpriteRect = SpriteRect::at(34, 11);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_466: SpriteRect = SpriteRect::at(34, 12);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_502: SpriteRect = SpriteRect::at(34, 13);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_538: SpriteRect = SpriteRect::at(34, 14);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_574: SpriteRect = SpriteRect::at(34, 15);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_610: SpriteRect = SpriteRect::at(34, 16);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_646: SpriteRect = SpriteRect::at(34, 17);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_682: SpriteRect = SpriteRect::at(34, 18);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_718: SpriteRect = SpriteRect::at(34, 19);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_754: SpriteRect = SpriteRect::at(34, 20);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_790: SpriteRect = SpriteRect::at(34, 21);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_826: SpriteRect = SpriteRect::at(34, 22);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_862: SpriteRect = SpriteRect::at(34, 23);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_898: SpriteRect = SpriteRect::at(34, 24);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_934: SpriteRect = SpriteRect::at(34, 25);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_35: SpriteRect = SpriteRect::at(35, 0);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_71: SpriteRect = SpriteRect::at(35, 1);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_107: SpriteRect = SpriteRect::at(35, 2);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_143: SpriteRect = SpriteRect::at(35, 3);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_179: SpriteRect = SpriteRect::at(35, 4);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_215: SpriteRect = SpriteRect::at(35, 5);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_251: SpriteRect = SpriteRect::at(35, 6);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_287: SpriteRect = SpriteRect::at(35, 7);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_323: SpriteRect = SpriteRect::at(35, 8);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_359: SpriteRect = SpriteRect::at(35, 9);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_395: SpriteRect = SpriteRect::at(35, 10);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_431: SpriteRect = SpriteRect::at(35, 11);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_467: SpriteRect = SpriteRect::at(35, 12);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_503: SpriteRect = SpriteRect::at(35, 13);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_539: SpriteRect = SpriteRect::at(35, 14);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_575: SpriteRect = SpriteRect::at(35, 15);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_611: SpriteRect = SpriteRect::at(35, 16);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_647: SpriteRect = SpriteRect::at(35, 17);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_683: SpriteRect = SpriteRect::at(35, 18);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_719: SpriteRect = SpriteRect::at(35, 19);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_755: SpriteRect = SpriteRect::at(35, 20);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_791: SpriteRect = SpriteRect::at(35, 21);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_827: SpriteRect = SpriteRect::at(35, 22);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_863: SpriteRect = SpriteRect::at(35, 23);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_899: SpriteRect = SpriteRect::at(35, 24);
#[allow(dead_code)]
pub const MONSTERS_SPRITE_935: SpriteRect = SpriteRect::at(35, 25);
