use crate::sprites::SpriteRect;

use phf::phf_map;
pub static ITEMS_SPRITES: phf::Map<&'static str, SpriteRect> = phf_map! {
    // consumables
    "purple_potion" => ITEMS_SPRITE_25,
    "red_potion" => ITEMS_SPRITE_26,

    // sprites for UI
    "small_gold_pile" => ITEMS_SPRITE_138,

    // wearables
    "green_leather_boots" => ITEMS_SPRITE_1,
    "mace" => ITEMS_SPRITE_2,
    "trident" => ITEMS_SPRITE_4,
    "greatsword" => ITEMS_SPRITE_5,
    "longsword" => ITEMS_SPRITE_6,
    "scimitar" => ITEMS_SPRITE_7,
    "purple_tip_wood_staff" => ITEMS_SPRITE_8,
    "purple_tip_silver_staff" => ITEMS_SPRITE_9,
    "red_cloth_boots" => ITEMS_SPRITE_576,
    "white_wizard_hat" => ITEMS_SPRITE_288,
};

#[allow(dead_code)]
pub const ITEMS_SPRITE_0: SpriteRect = SpriteRect::item_at(0.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_48: SpriteRect = SpriteRect::item_at(0.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_96: SpriteRect = SpriteRect::item_at(0.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_144: SpriteRect = SpriteRect::item_at(0.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_192: SpriteRect = SpriteRect::item_at(0.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_240: SpriteRect = SpriteRect::item_at(0.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_288: SpriteRect = SpriteRect::item_at(0.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_336: SpriteRect = SpriteRect::item_at(0.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_384: SpriteRect = SpriteRect::item_at(0.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_432: SpriteRect = SpriteRect::item_at(0.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_480: SpriteRect = SpriteRect::item_at(0.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_528: SpriteRect = SpriteRect::item_at(0.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_576: SpriteRect = SpriteRect::item_at(0.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_624: SpriteRect = SpriteRect::item_at(0.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_672: SpriteRect = SpriteRect::item_at(0.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_1: SpriteRect = SpriteRect::item_at(1.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_49: SpriteRect = SpriteRect::item_at(1.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_97: SpriteRect = SpriteRect::item_at(1.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_145: SpriteRect = SpriteRect::item_at(1.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_193: SpriteRect = SpriteRect::item_at(1.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_241: SpriteRect = SpriteRect::item_at(1.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_289: SpriteRect = SpriteRect::item_at(1.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_337: SpriteRect = SpriteRect::item_at(1.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_385: SpriteRect = SpriteRect::item_at(1.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_433: SpriteRect = SpriteRect::item_at(1.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_481: SpriteRect = SpriteRect::item_at(1.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_529: SpriteRect = SpriteRect::item_at(1.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_577: SpriteRect = SpriteRect::item_at(1.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_625: SpriteRect = SpriteRect::item_at(1.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_673: SpriteRect = SpriteRect::item_at(1.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_2: SpriteRect = SpriteRect::item_at(2.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_50: SpriteRect = SpriteRect::item_at(2.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_98: SpriteRect = SpriteRect::item_at(2.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_146: SpriteRect = SpriteRect::item_at(2.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_194: SpriteRect = SpriteRect::item_at(2.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_242: SpriteRect = SpriteRect::item_at(2.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_290: SpriteRect = SpriteRect::item_at(2.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_338: SpriteRect = SpriteRect::item_at(2.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_386: SpriteRect = SpriteRect::item_at(2.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_434: SpriteRect = SpriteRect::item_at(2.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_482: SpriteRect = SpriteRect::item_at(2.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_530: SpriteRect = SpriteRect::item_at(2.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_578: SpriteRect = SpriteRect::item_at(2.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_626: SpriteRect = SpriteRect::item_at(2.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_674: SpriteRect = SpriteRect::item_at(2.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_3: SpriteRect = SpriteRect::item_at(3.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_51: SpriteRect = SpriteRect::item_at(3.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_99: SpriteRect = SpriteRect::item_at(3.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_147: SpriteRect = SpriteRect::item_at(3.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_195: SpriteRect = SpriteRect::item_at(3.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_243: SpriteRect = SpriteRect::item_at(3.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_291: SpriteRect = SpriteRect::item_at(3.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_339: SpriteRect = SpriteRect::item_at(3.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_387: SpriteRect = SpriteRect::item_at(3.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_435: SpriteRect = SpriteRect::item_at(3.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_483: SpriteRect = SpriteRect::item_at(3.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_531: SpriteRect = SpriteRect::item_at(3.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_579: SpriteRect = SpriteRect::item_at(3.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_627: SpriteRect = SpriteRect::item_at(3.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_675: SpriteRect = SpriteRect::item_at(3.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_4: SpriteRect = SpriteRect::item_at(4.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_52: SpriteRect = SpriteRect::item_at(4.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_100: SpriteRect = SpriteRect::item_at(4.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_148: SpriteRect = SpriteRect::item_at(4.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_196: SpriteRect = SpriteRect::item_at(4.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_244: SpriteRect = SpriteRect::item_at(4.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_292: SpriteRect = SpriteRect::item_at(4.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_340: SpriteRect = SpriteRect::item_at(4.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_388: SpriteRect = SpriteRect::item_at(4.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_436: SpriteRect = SpriteRect::item_at(4.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_484: SpriteRect = SpriteRect::item_at(4.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_532: SpriteRect = SpriteRect::item_at(4.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_580: SpriteRect = SpriteRect::item_at(4.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_628: SpriteRect = SpriteRect::item_at(4.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_676: SpriteRect = SpriteRect::item_at(4.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_5: SpriteRect = SpriteRect::item_at(5.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_53: SpriteRect = SpriteRect::item_at(5.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_101: SpriteRect = SpriteRect::item_at(5.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_149: SpriteRect = SpriteRect::item_at(5.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_197: SpriteRect = SpriteRect::item_at(5.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_245: SpriteRect = SpriteRect::item_at(5.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_293: SpriteRect = SpriteRect::item_at(5.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_341: SpriteRect = SpriteRect::item_at(5.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_389: SpriteRect = SpriteRect::item_at(5.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_437: SpriteRect = SpriteRect::item_at(5.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_485: SpriteRect = SpriteRect::item_at(5.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_533: SpriteRect = SpriteRect::item_at(5.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_581: SpriteRect = SpriteRect::item_at(5.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_629: SpriteRect = SpriteRect::item_at(5.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_677: SpriteRect = SpriteRect::item_at(5.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_6: SpriteRect = SpriteRect::item_at(6.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_54: SpriteRect = SpriteRect::item_at(6.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_102: SpriteRect = SpriteRect::item_at(6.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_150: SpriteRect = SpriteRect::item_at(6.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_198: SpriteRect = SpriteRect::item_at(6.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_246: SpriteRect = SpriteRect::item_at(6.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_294: SpriteRect = SpriteRect::item_at(6.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_342: SpriteRect = SpriteRect::item_at(6.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_390: SpriteRect = SpriteRect::item_at(6.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_438: SpriteRect = SpriteRect::item_at(6.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_486: SpriteRect = SpriteRect::item_at(6.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_534: SpriteRect = SpriteRect::item_at(6.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_582: SpriteRect = SpriteRect::item_at(6.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_630: SpriteRect = SpriteRect::item_at(6.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_678: SpriteRect = SpriteRect::item_at(6.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_7: SpriteRect = SpriteRect::item_at(7.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_55: SpriteRect = SpriteRect::item_at(7.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_103: SpriteRect = SpriteRect::item_at(7.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_151: SpriteRect = SpriteRect::item_at(7.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_199: SpriteRect = SpriteRect::item_at(7.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_247: SpriteRect = SpriteRect::item_at(7.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_295: SpriteRect = SpriteRect::item_at(7.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_343: SpriteRect = SpriteRect::item_at(7.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_391: SpriteRect = SpriteRect::item_at(7.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_439: SpriteRect = SpriteRect::item_at(7.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_487: SpriteRect = SpriteRect::item_at(7.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_535: SpriteRect = SpriteRect::item_at(7.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_583: SpriteRect = SpriteRect::item_at(7.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_631: SpriteRect = SpriteRect::item_at(7.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_679: SpriteRect = SpriteRect::item_at(7.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_8: SpriteRect = SpriteRect::item_at(8.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_56: SpriteRect = SpriteRect::item_at(8.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_104: SpriteRect = SpriteRect::item_at(8.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_152: SpriteRect = SpriteRect::item_at(8.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_200: SpriteRect = SpriteRect::item_at(8.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_248: SpriteRect = SpriteRect::item_at(8.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_296: SpriteRect = SpriteRect::item_at(8.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_344: SpriteRect = SpriteRect::item_at(8.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_392: SpriteRect = SpriteRect::item_at(8.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_440: SpriteRect = SpriteRect::item_at(8.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_488: SpriteRect = SpriteRect::item_at(8.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_536: SpriteRect = SpriteRect::item_at(8.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_584: SpriteRect = SpriteRect::item_at(8.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_632: SpriteRect = SpriteRect::item_at(8.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_680: SpriteRect = SpriteRect::item_at(8.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_9: SpriteRect = SpriteRect::item_at(9.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_57: SpriteRect = SpriteRect::item_at(9.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_105: SpriteRect = SpriteRect::item_at(9.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_153: SpriteRect = SpriteRect::item_at(9.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_201: SpriteRect = SpriteRect::item_at(9.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_249: SpriteRect = SpriteRect::item_at(9.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_297: SpriteRect = SpriteRect::item_at(9.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_345: SpriteRect = SpriteRect::item_at(9.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_393: SpriteRect = SpriteRect::item_at(9.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_441: SpriteRect = SpriteRect::item_at(9.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_489: SpriteRect = SpriteRect::item_at(9.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_537: SpriteRect = SpriteRect::item_at(9.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_585: SpriteRect = SpriteRect::item_at(9.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_633: SpriteRect = SpriteRect::item_at(9.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_681: SpriteRect = SpriteRect::item_at(9.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_10: SpriteRect = SpriteRect::item_at(10.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_58: SpriteRect = SpriteRect::item_at(10.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_106: SpriteRect = SpriteRect::item_at(10.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_154: SpriteRect = SpriteRect::item_at(10.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_202: SpriteRect = SpriteRect::item_at(10.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_250: SpriteRect = SpriteRect::item_at(10.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_298: SpriteRect = SpriteRect::item_at(10.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_346: SpriteRect = SpriteRect::item_at(10.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_394: SpriteRect = SpriteRect::item_at(10.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_442: SpriteRect = SpriteRect::item_at(10.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_490: SpriteRect = SpriteRect::item_at(10.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_538: SpriteRect = SpriteRect::item_at(10.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_586: SpriteRect = SpriteRect::item_at(10.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_634: SpriteRect = SpriteRect::item_at(10.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_682: SpriteRect = SpriteRect::item_at(10.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_11: SpriteRect = SpriteRect::item_at(11.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_59: SpriteRect = SpriteRect::item_at(11.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_107: SpriteRect = SpriteRect::item_at(11.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_155: SpriteRect = SpriteRect::item_at(11.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_203: SpriteRect = SpriteRect::item_at(11.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_251: SpriteRect = SpriteRect::item_at(11.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_299: SpriteRect = SpriteRect::item_at(11.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_347: SpriteRect = SpriteRect::item_at(11.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_395: SpriteRect = SpriteRect::item_at(11.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_443: SpriteRect = SpriteRect::item_at(11.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_491: SpriteRect = SpriteRect::item_at(11.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_539: SpriteRect = SpriteRect::item_at(11.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_587: SpriteRect = SpriteRect::item_at(11.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_635: SpriteRect = SpriteRect::item_at(11.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_683: SpriteRect = SpriteRect::item_at(11.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_12: SpriteRect = SpriteRect::item_at(12.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_60: SpriteRect = SpriteRect::item_at(12.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_108: SpriteRect = SpriteRect::item_at(12.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_156: SpriteRect = SpriteRect::item_at(12.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_204: SpriteRect = SpriteRect::item_at(12.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_252: SpriteRect = SpriteRect::item_at(12.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_300: SpriteRect = SpriteRect::item_at(12.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_348: SpriteRect = SpriteRect::item_at(12.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_396: SpriteRect = SpriteRect::item_at(12.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_444: SpriteRect = SpriteRect::item_at(12.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_492: SpriteRect = SpriteRect::item_at(12.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_540: SpriteRect = SpriteRect::item_at(12.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_588: SpriteRect = SpriteRect::item_at(12.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_636: SpriteRect = SpriteRect::item_at(12.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_684: SpriteRect = SpriteRect::item_at(12.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_13: SpriteRect = SpriteRect::item_at(13.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_61: SpriteRect = SpriteRect::item_at(13.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_109: SpriteRect = SpriteRect::item_at(13.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_157: SpriteRect = SpriteRect::item_at(13.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_205: SpriteRect = SpriteRect::item_at(13.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_253: SpriteRect = SpriteRect::item_at(13.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_301: SpriteRect = SpriteRect::item_at(13.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_349: SpriteRect = SpriteRect::item_at(13.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_397: SpriteRect = SpriteRect::item_at(13.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_445: SpriteRect = SpriteRect::item_at(13.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_493: SpriteRect = SpriteRect::item_at(13.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_541: SpriteRect = SpriteRect::item_at(13.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_589: SpriteRect = SpriteRect::item_at(13.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_637: SpriteRect = SpriteRect::item_at(13.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_685: SpriteRect = SpriteRect::item_at(13.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_14: SpriteRect = SpriteRect::item_at(14.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_62: SpriteRect = SpriteRect::item_at(14.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_110: SpriteRect = SpriteRect::item_at(14.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_158: SpriteRect = SpriteRect::item_at(14.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_206: SpriteRect = SpriteRect::item_at(14.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_254: SpriteRect = SpriteRect::item_at(14.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_302: SpriteRect = SpriteRect::item_at(14.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_350: SpriteRect = SpriteRect::item_at(14.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_398: SpriteRect = SpriteRect::item_at(14.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_446: SpriteRect = SpriteRect::item_at(14.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_494: SpriteRect = SpriteRect::item_at(14.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_542: SpriteRect = SpriteRect::item_at(14.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_590: SpriteRect = SpriteRect::item_at(14.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_638: SpriteRect = SpriteRect::item_at(14.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_686: SpriteRect = SpriteRect::item_at(14.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_15: SpriteRect = SpriteRect::item_at(15.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_63: SpriteRect = SpriteRect::item_at(15.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_111: SpriteRect = SpriteRect::item_at(15.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_159: SpriteRect = SpriteRect::item_at(15.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_207: SpriteRect = SpriteRect::item_at(15.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_255: SpriteRect = SpriteRect::item_at(15.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_303: SpriteRect = SpriteRect::item_at(15.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_351: SpriteRect = SpriteRect::item_at(15.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_399: SpriteRect = SpriteRect::item_at(15.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_447: SpriteRect = SpriteRect::item_at(15.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_495: SpriteRect = SpriteRect::item_at(15.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_543: SpriteRect = SpriteRect::item_at(15.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_591: SpriteRect = SpriteRect::item_at(15.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_639: SpriteRect = SpriteRect::item_at(15.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_687: SpriteRect = SpriteRect::item_at(15.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_16: SpriteRect = SpriteRect::item_at(16.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_64: SpriteRect = SpriteRect::item_at(16.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_112: SpriteRect = SpriteRect::item_at(16.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_160: SpriteRect = SpriteRect::item_at(16.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_208: SpriteRect = SpriteRect::item_at(16.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_256: SpriteRect = SpriteRect::item_at(16.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_304: SpriteRect = SpriteRect::item_at(16.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_352: SpriteRect = SpriteRect::item_at(16.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_400: SpriteRect = SpriteRect::item_at(16.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_448: SpriteRect = SpriteRect::item_at(16.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_496: SpriteRect = SpriteRect::item_at(16.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_544: SpriteRect = SpriteRect::item_at(16.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_592: SpriteRect = SpriteRect::item_at(16.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_640: SpriteRect = SpriteRect::item_at(16.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_688: SpriteRect = SpriteRect::item_at(16.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_17: SpriteRect = SpriteRect::item_at(17.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_65: SpriteRect = SpriteRect::item_at(17.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_113: SpriteRect = SpriteRect::item_at(17.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_161: SpriteRect = SpriteRect::item_at(17.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_209: SpriteRect = SpriteRect::item_at(17.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_257: SpriteRect = SpriteRect::item_at(17.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_305: SpriteRect = SpriteRect::item_at(17.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_353: SpriteRect = SpriteRect::item_at(17.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_401: SpriteRect = SpriteRect::item_at(17.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_449: SpriteRect = SpriteRect::item_at(17.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_497: SpriteRect = SpriteRect::item_at(17.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_545: SpriteRect = SpriteRect::item_at(17.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_593: SpriteRect = SpriteRect::item_at(17.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_641: SpriteRect = SpriteRect::item_at(17.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_689: SpriteRect = SpriteRect::item_at(17.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_18: SpriteRect = SpriteRect::item_at(18.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_66: SpriteRect = SpriteRect::item_at(18.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_114: SpriteRect = SpriteRect::item_at(18.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_162: SpriteRect = SpriteRect::item_at(18.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_210: SpriteRect = SpriteRect::item_at(18.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_258: SpriteRect = SpriteRect::item_at(18.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_306: SpriteRect = SpriteRect::item_at(18.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_354: SpriteRect = SpriteRect::item_at(18.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_402: SpriteRect = SpriteRect::item_at(18.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_450: SpriteRect = SpriteRect::item_at(18.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_498: SpriteRect = SpriteRect::item_at(18.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_546: SpriteRect = SpriteRect::item_at(18.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_594: SpriteRect = SpriteRect::item_at(18.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_642: SpriteRect = SpriteRect::item_at(18.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_690: SpriteRect = SpriteRect::item_at(18.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_19: SpriteRect = SpriteRect::item_at(19.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_67: SpriteRect = SpriteRect::item_at(19.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_115: SpriteRect = SpriteRect::item_at(19.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_163: SpriteRect = SpriteRect::item_at(19.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_211: SpriteRect = SpriteRect::item_at(19.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_259: SpriteRect = SpriteRect::item_at(19.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_307: SpriteRect = SpriteRect::item_at(19.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_355: SpriteRect = SpriteRect::item_at(19.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_403: SpriteRect = SpriteRect::item_at(19.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_451: SpriteRect = SpriteRect::item_at(19.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_499: SpriteRect = SpriteRect::item_at(19.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_547: SpriteRect = SpriteRect::item_at(19.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_595: SpriteRect = SpriteRect::item_at(19.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_643: SpriteRect = SpriteRect::item_at(19.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_691: SpriteRect = SpriteRect::item_at(19.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_20: SpriteRect = SpriteRect::item_at(20.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_68: SpriteRect = SpriteRect::item_at(20.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_116: SpriteRect = SpriteRect::item_at(20.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_164: SpriteRect = SpriteRect::item_at(20.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_212: SpriteRect = SpriteRect::item_at(20.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_260: SpriteRect = SpriteRect::item_at(20.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_308: SpriteRect = SpriteRect::item_at(20.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_356: SpriteRect = SpriteRect::item_at(20.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_404: SpriteRect = SpriteRect::item_at(20.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_452: SpriteRect = SpriteRect::item_at(20.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_500: SpriteRect = SpriteRect::item_at(20.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_548: SpriteRect = SpriteRect::item_at(20.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_596: SpriteRect = SpriteRect::item_at(20.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_644: SpriteRect = SpriteRect::item_at(20.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_692: SpriteRect = SpriteRect::item_at(20.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_21: SpriteRect = SpriteRect::item_at(21.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_69: SpriteRect = SpriteRect::item_at(21.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_117: SpriteRect = SpriteRect::item_at(21.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_165: SpriteRect = SpriteRect::item_at(21.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_213: SpriteRect = SpriteRect::item_at(21.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_261: SpriteRect = SpriteRect::item_at(21.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_309: SpriteRect = SpriteRect::item_at(21.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_357: SpriteRect = SpriteRect::item_at(21.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_405: SpriteRect = SpriteRect::item_at(21.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_453: SpriteRect = SpriteRect::item_at(21.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_501: SpriteRect = SpriteRect::item_at(21.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_549: SpriteRect = SpriteRect::item_at(21.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_597: SpriteRect = SpriteRect::item_at(21.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_645: SpriteRect = SpriteRect::item_at(21.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_693: SpriteRect = SpriteRect::item_at(21.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_22: SpriteRect = SpriteRect::item_at(22.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_70: SpriteRect = SpriteRect::item_at(22.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_118: SpriteRect = SpriteRect::item_at(22.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_166: SpriteRect = SpriteRect::item_at(22.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_214: SpriteRect = SpriteRect::item_at(22.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_262: SpriteRect = SpriteRect::item_at(22.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_310: SpriteRect = SpriteRect::item_at(22.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_358: SpriteRect = SpriteRect::item_at(22.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_406: SpriteRect = SpriteRect::item_at(22.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_454: SpriteRect = SpriteRect::item_at(22.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_502: SpriteRect = SpriteRect::item_at(22.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_550: SpriteRect = SpriteRect::item_at(22.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_598: SpriteRect = SpriteRect::item_at(22.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_646: SpriteRect = SpriteRect::item_at(22.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_694: SpriteRect = SpriteRect::item_at(22.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_23: SpriteRect = SpriteRect::item_at(23.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_71: SpriteRect = SpriteRect::item_at(23.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_119: SpriteRect = SpriteRect::item_at(23.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_167: SpriteRect = SpriteRect::item_at(23.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_215: SpriteRect = SpriteRect::item_at(23.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_263: SpriteRect = SpriteRect::item_at(23.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_311: SpriteRect = SpriteRect::item_at(23.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_359: SpriteRect = SpriteRect::item_at(23.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_407: SpriteRect = SpriteRect::item_at(23.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_455: SpriteRect = SpriteRect::item_at(23.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_503: SpriteRect = SpriteRect::item_at(23.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_551: SpriteRect = SpriteRect::item_at(23.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_599: SpriteRect = SpriteRect::item_at(23.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_647: SpriteRect = SpriteRect::item_at(23.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_695: SpriteRect = SpriteRect::item_at(23.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_24: SpriteRect = SpriteRect::item_at(24.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_72: SpriteRect = SpriteRect::item_at(24.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_120: SpriteRect = SpriteRect::item_at(24.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_168: SpriteRect = SpriteRect::item_at(24.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_216: SpriteRect = SpriteRect::item_at(24.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_264: SpriteRect = SpriteRect::item_at(24.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_312: SpriteRect = SpriteRect::item_at(24.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_360: SpriteRect = SpriteRect::item_at(24.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_408: SpriteRect = SpriteRect::item_at(24.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_456: SpriteRect = SpriteRect::item_at(24.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_504: SpriteRect = SpriteRect::item_at(24.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_552: SpriteRect = SpriteRect::item_at(24.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_600: SpriteRect = SpriteRect::item_at(24.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_648: SpriteRect = SpriteRect::item_at(24.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_696: SpriteRect = SpriteRect::item_at(24.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_25: SpriteRect = SpriteRect::item_at(25.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_73: SpriteRect = SpriteRect::item_at(25.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_121: SpriteRect = SpriteRect::item_at(25.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_169: SpriteRect = SpriteRect::item_at(25.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_217: SpriteRect = SpriteRect::item_at(25.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_265: SpriteRect = SpriteRect::item_at(25.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_313: SpriteRect = SpriteRect::item_at(25.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_361: SpriteRect = SpriteRect::item_at(25.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_409: SpriteRect = SpriteRect::item_at(25.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_457: SpriteRect = SpriteRect::item_at(25.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_505: SpriteRect = SpriteRect::item_at(25.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_553: SpriteRect = SpriteRect::item_at(25.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_601: SpriteRect = SpriteRect::item_at(25.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_649: SpriteRect = SpriteRect::item_at(25.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_697: SpriteRect = SpriteRect::item_at(25.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_26: SpriteRect = SpriteRect::item_at(26.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_74: SpriteRect = SpriteRect::item_at(26.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_122: SpriteRect = SpriteRect::item_at(26.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_170: SpriteRect = SpriteRect::item_at(26.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_218: SpriteRect = SpriteRect::item_at(26.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_266: SpriteRect = SpriteRect::item_at(26.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_314: SpriteRect = SpriteRect::item_at(26.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_362: SpriteRect = SpriteRect::item_at(26.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_410: SpriteRect = SpriteRect::item_at(26.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_458: SpriteRect = SpriteRect::item_at(26.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_506: SpriteRect = SpriteRect::item_at(26.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_554: SpriteRect = SpriteRect::item_at(26.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_602: SpriteRect = SpriteRect::item_at(26.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_650: SpriteRect = SpriteRect::item_at(26.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_698: SpriteRect = SpriteRect::item_at(26.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_27: SpriteRect = SpriteRect::item_at(27.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_75: SpriteRect = SpriteRect::item_at(27.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_123: SpriteRect = SpriteRect::item_at(27.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_171: SpriteRect = SpriteRect::item_at(27.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_219: SpriteRect = SpriteRect::item_at(27.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_267: SpriteRect = SpriteRect::item_at(27.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_315: SpriteRect = SpriteRect::item_at(27.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_363: SpriteRect = SpriteRect::item_at(27.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_411: SpriteRect = SpriteRect::item_at(27.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_459: SpriteRect = SpriteRect::item_at(27.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_507: SpriteRect = SpriteRect::item_at(27.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_555: SpriteRect = SpriteRect::item_at(27.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_603: SpriteRect = SpriteRect::item_at(27.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_651: SpriteRect = SpriteRect::item_at(27.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_699: SpriteRect = SpriteRect::item_at(27.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_28: SpriteRect = SpriteRect::item_at(28.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_76: SpriteRect = SpriteRect::item_at(28.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_124: SpriteRect = SpriteRect::item_at(28.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_172: SpriteRect = SpriteRect::item_at(28.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_220: SpriteRect = SpriteRect::item_at(28.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_268: SpriteRect = SpriteRect::item_at(28.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_316: SpriteRect = SpriteRect::item_at(28.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_364: SpriteRect = SpriteRect::item_at(28.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_412: SpriteRect = SpriteRect::item_at(28.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_460: SpriteRect = SpriteRect::item_at(28.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_508: SpriteRect = SpriteRect::item_at(28.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_556: SpriteRect = SpriteRect::item_at(28.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_604: SpriteRect = SpriteRect::item_at(28.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_652: SpriteRect = SpriteRect::item_at(28.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_700: SpriteRect = SpriteRect::item_at(28.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_29: SpriteRect = SpriteRect::item_at(29.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_77: SpriteRect = SpriteRect::item_at(29.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_125: SpriteRect = SpriteRect::item_at(29.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_173: SpriteRect = SpriteRect::item_at(29.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_221: SpriteRect = SpriteRect::item_at(29.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_269: SpriteRect = SpriteRect::item_at(29.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_317: SpriteRect = SpriteRect::item_at(29.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_365: SpriteRect = SpriteRect::item_at(29.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_413: SpriteRect = SpriteRect::item_at(29.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_461: SpriteRect = SpriteRect::item_at(29.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_509: SpriteRect = SpriteRect::item_at(29.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_557: SpriteRect = SpriteRect::item_at(29.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_605: SpriteRect = SpriteRect::item_at(29.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_653: SpriteRect = SpriteRect::item_at(29.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_701: SpriteRect = SpriteRect::item_at(29.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_30: SpriteRect = SpriteRect::item_at(30.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_78: SpriteRect = SpriteRect::item_at(30.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_126: SpriteRect = SpriteRect::item_at(30.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_174: SpriteRect = SpriteRect::item_at(30.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_222: SpriteRect = SpriteRect::item_at(30.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_270: SpriteRect = SpriteRect::item_at(30.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_318: SpriteRect = SpriteRect::item_at(30.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_366: SpriteRect = SpriteRect::item_at(30.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_414: SpriteRect = SpriteRect::item_at(30.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_462: SpriteRect = SpriteRect::item_at(30.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_510: SpriteRect = SpriteRect::item_at(30.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_558: SpriteRect = SpriteRect::item_at(30.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_606: SpriteRect = SpriteRect::item_at(30.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_654: SpriteRect = SpriteRect::item_at(30.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_702: SpriteRect = SpriteRect::item_at(30.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_31: SpriteRect = SpriteRect::item_at(31.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_79: SpriteRect = SpriteRect::item_at(31.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_127: SpriteRect = SpriteRect::item_at(31.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_175: SpriteRect = SpriteRect::item_at(31.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_223: SpriteRect = SpriteRect::item_at(31.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_271: SpriteRect = SpriteRect::item_at(31.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_319: SpriteRect = SpriteRect::item_at(31.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_367: SpriteRect = SpriteRect::item_at(31.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_415: SpriteRect = SpriteRect::item_at(31.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_463: SpriteRect = SpriteRect::item_at(31.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_511: SpriteRect = SpriteRect::item_at(31.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_559: SpriteRect = SpriteRect::item_at(31.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_607: SpriteRect = SpriteRect::item_at(31.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_655: SpriteRect = SpriteRect::item_at(31.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_703: SpriteRect = SpriteRect::item_at(31.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_32: SpriteRect = SpriteRect::item_at(32.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_80: SpriteRect = SpriteRect::item_at(32.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_128: SpriteRect = SpriteRect::item_at(32.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_176: SpriteRect = SpriteRect::item_at(32.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_224: SpriteRect = SpriteRect::item_at(32.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_272: SpriteRect = SpriteRect::item_at(32.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_320: SpriteRect = SpriteRect::item_at(32.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_368: SpriteRect = SpriteRect::item_at(32.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_416: SpriteRect = SpriteRect::item_at(32.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_464: SpriteRect = SpriteRect::item_at(32.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_512: SpriteRect = SpriteRect::item_at(32.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_560: SpriteRect = SpriteRect::item_at(32.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_608: SpriteRect = SpriteRect::item_at(32.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_656: SpriteRect = SpriteRect::item_at(32.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_704: SpriteRect = SpriteRect::item_at(32.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_33: SpriteRect = SpriteRect::item_at(33.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_81: SpriteRect = SpriteRect::item_at(33.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_129: SpriteRect = SpriteRect::item_at(33.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_177: SpriteRect = SpriteRect::item_at(33.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_225: SpriteRect = SpriteRect::item_at(33.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_273: SpriteRect = SpriteRect::item_at(33.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_321: SpriteRect = SpriteRect::item_at(33.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_369: SpriteRect = SpriteRect::item_at(33.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_417: SpriteRect = SpriteRect::item_at(33.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_465: SpriteRect = SpriteRect::item_at(33.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_513: SpriteRect = SpriteRect::item_at(33.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_561: SpriteRect = SpriteRect::item_at(33.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_609: SpriteRect = SpriteRect::item_at(33.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_657: SpriteRect = SpriteRect::item_at(33.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_705: SpriteRect = SpriteRect::item_at(33.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_34: SpriteRect = SpriteRect::item_at(34.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_82: SpriteRect = SpriteRect::item_at(34.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_130: SpriteRect = SpriteRect::item_at(34.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_178: SpriteRect = SpriteRect::item_at(34.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_226: SpriteRect = SpriteRect::item_at(34.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_274: SpriteRect = SpriteRect::item_at(34.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_322: SpriteRect = SpriteRect::item_at(34.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_370: SpriteRect = SpriteRect::item_at(34.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_418: SpriteRect = SpriteRect::item_at(34.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_466: SpriteRect = SpriteRect::item_at(34.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_514: SpriteRect = SpriteRect::item_at(34.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_562: SpriteRect = SpriteRect::item_at(34.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_610: SpriteRect = SpriteRect::item_at(34.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_658: SpriteRect = SpriteRect::item_at(34.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_706: SpriteRect = SpriteRect::item_at(34.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_35: SpriteRect = SpriteRect::item_at(35.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_83: SpriteRect = SpriteRect::item_at(35.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_131: SpriteRect = SpriteRect::item_at(35.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_179: SpriteRect = SpriteRect::item_at(35.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_227: SpriteRect = SpriteRect::item_at(35.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_275: SpriteRect = SpriteRect::item_at(35.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_323: SpriteRect = SpriteRect::item_at(35.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_371: SpriteRect = SpriteRect::item_at(35.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_419: SpriteRect = SpriteRect::item_at(35.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_467: SpriteRect = SpriteRect::item_at(35.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_515: SpriteRect = SpriteRect::item_at(35.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_563: SpriteRect = SpriteRect::item_at(35.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_611: SpriteRect = SpriteRect::item_at(35.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_659: SpriteRect = SpriteRect::item_at(35.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_707: SpriteRect = SpriteRect::item_at(35.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_36: SpriteRect = SpriteRect::item_at(36.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_84: SpriteRect = SpriteRect::item_at(36.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_132: SpriteRect = SpriteRect::item_at(36.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_180: SpriteRect = SpriteRect::item_at(36.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_228: SpriteRect = SpriteRect::item_at(36.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_276: SpriteRect = SpriteRect::item_at(36.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_324: SpriteRect = SpriteRect::item_at(36.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_372: SpriteRect = SpriteRect::item_at(36.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_420: SpriteRect = SpriteRect::item_at(36.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_468: SpriteRect = SpriteRect::item_at(36.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_516: SpriteRect = SpriteRect::item_at(36.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_564: SpriteRect = SpriteRect::item_at(36.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_612: SpriteRect = SpriteRect::item_at(36.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_660: SpriteRect = SpriteRect::item_at(36.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_708: SpriteRect = SpriteRect::item_at(36.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_37: SpriteRect = SpriteRect::item_at(37.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_85: SpriteRect = SpriteRect::item_at(37.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_133: SpriteRect = SpriteRect::item_at(37.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_181: SpriteRect = SpriteRect::item_at(37.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_229: SpriteRect = SpriteRect::item_at(37.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_277: SpriteRect = SpriteRect::item_at(37.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_325: SpriteRect = SpriteRect::item_at(37.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_373: SpriteRect = SpriteRect::item_at(37.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_421: SpriteRect = SpriteRect::item_at(37.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_469: SpriteRect = SpriteRect::item_at(37.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_517: SpriteRect = SpriteRect::item_at(37.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_565: SpriteRect = SpriteRect::item_at(37.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_613: SpriteRect = SpriteRect::item_at(37.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_661: SpriteRect = SpriteRect::item_at(37.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_709: SpriteRect = SpriteRect::item_at(37.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_38: SpriteRect = SpriteRect::item_at(38.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_86: SpriteRect = SpriteRect::item_at(38.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_134: SpriteRect = SpriteRect::item_at(38.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_182: SpriteRect = SpriteRect::item_at(38.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_230: SpriteRect = SpriteRect::item_at(38.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_278: SpriteRect = SpriteRect::item_at(38.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_326: SpriteRect = SpriteRect::item_at(38.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_374: SpriteRect = SpriteRect::item_at(38.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_422: SpriteRect = SpriteRect::item_at(38.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_470: SpriteRect = SpriteRect::item_at(38.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_518: SpriteRect = SpriteRect::item_at(38.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_566: SpriteRect = SpriteRect::item_at(38.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_614: SpriteRect = SpriteRect::item_at(38.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_662: SpriteRect = SpriteRect::item_at(38.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_710: SpriteRect = SpriteRect::item_at(38.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_39: SpriteRect = SpriteRect::item_at(39.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_87: SpriteRect = SpriteRect::item_at(39.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_135: SpriteRect = SpriteRect::item_at(39.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_183: SpriteRect = SpriteRect::item_at(39.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_231: SpriteRect = SpriteRect::item_at(39.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_279: SpriteRect = SpriteRect::item_at(39.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_327: SpriteRect = SpriteRect::item_at(39.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_375: SpriteRect = SpriteRect::item_at(39.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_423: SpriteRect = SpriteRect::item_at(39.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_471: SpriteRect = SpriteRect::item_at(39.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_519: SpriteRect = SpriteRect::item_at(39.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_567: SpriteRect = SpriteRect::item_at(39.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_615: SpriteRect = SpriteRect::item_at(39.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_663: SpriteRect = SpriteRect::item_at(39.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_711: SpriteRect = SpriteRect::item_at(39.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_40: SpriteRect = SpriteRect::item_at(40.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_88: SpriteRect = SpriteRect::item_at(40.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_136: SpriteRect = SpriteRect::item_at(40.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_184: SpriteRect = SpriteRect::item_at(40.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_232: SpriteRect = SpriteRect::item_at(40.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_280: SpriteRect = SpriteRect::item_at(40.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_328: SpriteRect = SpriteRect::item_at(40.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_376: SpriteRect = SpriteRect::item_at(40.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_424: SpriteRect = SpriteRect::item_at(40.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_472: SpriteRect = SpriteRect::item_at(40.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_520: SpriteRect = SpriteRect::item_at(40.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_568: SpriteRect = SpriteRect::item_at(40.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_616: SpriteRect = SpriteRect::item_at(40.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_664: SpriteRect = SpriteRect::item_at(40.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_712: SpriteRect = SpriteRect::item_at(40.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_41: SpriteRect = SpriteRect::item_at(41.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_89: SpriteRect = SpriteRect::item_at(41.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_137: SpriteRect = SpriteRect::item_at(41.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_185: SpriteRect = SpriteRect::item_at(41.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_233: SpriteRect = SpriteRect::item_at(41.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_281: SpriteRect = SpriteRect::item_at(41.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_329: SpriteRect = SpriteRect::item_at(41.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_377: SpriteRect = SpriteRect::item_at(41.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_425: SpriteRect = SpriteRect::item_at(41.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_473: SpriteRect = SpriteRect::item_at(41.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_521: SpriteRect = SpriteRect::item_at(41.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_569: SpriteRect = SpriteRect::item_at(41.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_617: SpriteRect = SpriteRect::item_at(41.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_665: SpriteRect = SpriteRect::item_at(41.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_713: SpriteRect = SpriteRect::item_at(41.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_42: SpriteRect = SpriteRect::item_at(42.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_90: SpriteRect = SpriteRect::item_at(42.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_138: SpriteRect = SpriteRect::item_at(42.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_186: SpriteRect = SpriteRect::item_at(42.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_234: SpriteRect = SpriteRect::item_at(42.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_282: SpriteRect = SpriteRect::item_at(42.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_330: SpriteRect = SpriteRect::item_at(42.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_378: SpriteRect = SpriteRect::item_at(42.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_426: SpriteRect = SpriteRect::item_at(42.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_474: SpriteRect = SpriteRect::item_at(42.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_522: SpriteRect = SpriteRect::item_at(42.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_570: SpriteRect = SpriteRect::item_at(42.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_618: SpriteRect = SpriteRect::item_at(42.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_666: SpriteRect = SpriteRect::item_at(42.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_714: SpriteRect = SpriteRect::item_at(42.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_43: SpriteRect = SpriteRect::item_at(43.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_91: SpriteRect = SpriteRect::item_at(43.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_139: SpriteRect = SpriteRect::item_at(43.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_187: SpriteRect = SpriteRect::item_at(43.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_235: SpriteRect = SpriteRect::item_at(43.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_283: SpriteRect = SpriteRect::item_at(43.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_331: SpriteRect = SpriteRect::item_at(43.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_379: SpriteRect = SpriteRect::item_at(43.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_427: SpriteRect = SpriteRect::item_at(43.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_475: SpriteRect = SpriteRect::item_at(43.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_523: SpriteRect = SpriteRect::item_at(43.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_571: SpriteRect = SpriteRect::item_at(43.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_619: SpriteRect = SpriteRect::item_at(43.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_667: SpriteRect = SpriteRect::item_at(43.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_715: SpriteRect = SpriteRect::item_at(43.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_44: SpriteRect = SpriteRect::item_at(44.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_92: SpriteRect = SpriteRect::item_at(44.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_140: SpriteRect = SpriteRect::item_at(44.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_188: SpriteRect = SpriteRect::item_at(44.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_236: SpriteRect = SpriteRect::item_at(44.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_284: SpriteRect = SpriteRect::item_at(44.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_332: SpriteRect = SpriteRect::item_at(44.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_380: SpriteRect = SpriteRect::item_at(44.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_428: SpriteRect = SpriteRect::item_at(44.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_476: SpriteRect = SpriteRect::item_at(44.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_524: SpriteRect = SpriteRect::item_at(44.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_572: SpriteRect = SpriteRect::item_at(44.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_620: SpriteRect = SpriteRect::item_at(44.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_668: SpriteRect = SpriteRect::item_at(44.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_716: SpriteRect = SpriteRect::item_at(44.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_45: SpriteRect = SpriteRect::item_at(45.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_93: SpriteRect = SpriteRect::item_at(45.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_141: SpriteRect = SpriteRect::item_at(45.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_189: SpriteRect = SpriteRect::item_at(45.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_237: SpriteRect = SpriteRect::item_at(45.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_285: SpriteRect = SpriteRect::item_at(45.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_333: SpriteRect = SpriteRect::item_at(45.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_381: SpriteRect = SpriteRect::item_at(45.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_429: SpriteRect = SpriteRect::item_at(45.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_477: SpriteRect = SpriteRect::item_at(45.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_525: SpriteRect = SpriteRect::item_at(45.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_573: SpriteRect = SpriteRect::item_at(45.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_621: SpriteRect = SpriteRect::item_at(45.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_669: SpriteRect = SpriteRect::item_at(45.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_717: SpriteRect = SpriteRect::item_at(45.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_46: SpriteRect = SpriteRect::item_at(46.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_94: SpriteRect = SpriteRect::item_at(46.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_142: SpriteRect = SpriteRect::item_at(46.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_190: SpriteRect = SpriteRect::item_at(46.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_238: SpriteRect = SpriteRect::item_at(46.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_286: SpriteRect = SpriteRect::item_at(46.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_334: SpriteRect = SpriteRect::item_at(46.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_382: SpriteRect = SpriteRect::item_at(46.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_430: SpriteRect = SpriteRect::item_at(46.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_478: SpriteRect = SpriteRect::item_at(46.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_526: SpriteRect = SpriteRect::item_at(46.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_574: SpriteRect = SpriteRect::item_at(46.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_622: SpriteRect = SpriteRect::item_at(46.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_670: SpriteRect = SpriteRect::item_at(46.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_718: SpriteRect = SpriteRect::item_at(46.0, 14.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_47: SpriteRect = SpriteRect::item_at(47.0, 0.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_95: SpriteRect = SpriteRect::item_at(47.0, 1.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_143: SpriteRect = SpriteRect::item_at(47.0, 2.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_191: SpriteRect = SpriteRect::item_at(47.0, 3.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_239: SpriteRect = SpriteRect::item_at(47.0, 4.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_287: SpriteRect = SpriteRect::item_at(47.0, 5.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_335: SpriteRect = SpriteRect::item_at(47.0, 6.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_383: SpriteRect = SpriteRect::item_at(47.0, 7.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_431: SpriteRect = SpriteRect::item_at(47.0, 8.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_479: SpriteRect = SpriteRect::item_at(47.0, 9.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_527: SpriteRect = SpriteRect::item_at(47.0, 10.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_575: SpriteRect = SpriteRect::item_at(47.0, 11.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_623: SpriteRect = SpriteRect::item_at(47.0, 12.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_671: SpriteRect = SpriteRect::item_at(47.0, 13.0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_719: SpriteRect = SpriteRect::item_at(47.0, 14.0);
