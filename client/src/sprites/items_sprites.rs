use crate::sprites::SpriteRect;
use paste::paste;
use std::collections::HashMap;
use std::sync::LazyLock;

macro_rules! item_sprite_map {
    ($($i:expr),*) => {{
        let mut map = HashMap::new();
        paste! {
            $( map.insert($i, [<ITEMS_SPRITE_ $i>]); )*
        }
        map
    }};
}

pub static ITEMS_SPRITES: LazyLock<HashMap<usize, SpriteRect>> = LazyLock::new(|| {
    item_sprite_map![
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70,
        71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93,
        94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112,
        113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130,
        131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148,
        149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166,
        167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184,
        185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202,
        203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220,
        221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238,
        239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255, 256,
        257, 258, 259, 260, 261, 262, 263, 264, 265, 266, 267, 268, 269, 270, 271, 272, 273, 274,
        275, 276, 277, 278, 279, 280, 281, 282, 283, 284, 285, 286, 287, 288, 289, 290, 291, 292,
        293, 294, 295, 296, 297, 298, 299, 300, 301, 302, 303, 304, 305, 306, 307, 308, 309, 310,
        311, 312, 313, 314, 315, 316, 317, 318, 319, 320, 321, 322, 323, 324, 325, 326, 327, 328,
        329, 330, 331, 332, 333, 334, 335, 336, 337, 338, 339, 340, 341, 342, 343, 344, 345, 346,
        347, 348, 349, 350, 351, 352, 353, 354, 355, 356, 357, 358, 359, 360, 361, 362, 363, 364,
        365, 366, 367, 368, 369, 370, 371, 372, 373, 374, 375, 376, 377, 378, 379, 380, 381, 382,
        383, 384, 385, 386, 387, 388, 389, 390, 391, 392, 393, 394, 395, 396, 397, 398, 399, 400,
        401, 402, 403, 404, 405, 406, 407, 408, 409, 410, 411, 412, 413, 414, 415, 416, 417, 418,
        419, 420, 421, 422, 423, 424, 425, 426, 427, 428, 429, 430, 431, 432, 433, 434, 435, 436,
        437, 438, 439, 440, 441, 442, 443, 444, 445, 446, 447, 448, 449, 450, 451, 452, 453, 454,
        455, 456, 457, 458, 459, 460, 461, 462, 463, 464, 465, 466, 467, 468, 469, 470, 471, 472,
        473, 474, 475, 476, 477, 478, 479, 480, 481, 482, 483, 484, 485, 486, 487, 488, 489, 490,
        491, 492, 493, 494, 495, 496, 497, 498, 499, 500, 501, 502, 503, 504, 505, 506, 507, 508,
        509, 510, 511, 512, 513, 514, 515, 516, 517, 518, 519, 520, 521, 522, 523, 524, 525, 526,
        527, 528, 529, 530, 531, 532, 533, 534, 535, 536, 537, 538, 539, 540, 541, 542, 543, 544,
        545, 546, 547, 548, 549, 550, 551, 552, 553, 554, 555, 556, 557, 558, 559, 560, 561, 562,
        563, 564, 565, 566, 567, 568, 569, 570, 571, 572, 573, 574, 575, 576, 577, 578, 579, 580,
        581, 582, 583, 584, 585, 586, 587, 588, 589, 590, 591, 592, 593, 594, 595, 596, 597, 598,
        599, 600, 601, 602, 603, 604, 605, 606, 607, 608, 609, 610, 611, 612, 613, 614, 615, 616,
        617, 618, 619, 620, 621, 622, 623, 624, 625, 626, 627, 628, 629, 630, 631, 632, 633, 634,
        635, 636, 637, 638, 639, 640, 641, 642, 643, 644, 645, 646, 647, 648, 649, 650, 651, 652,
        653, 654, 655, 656, 657, 658, 659, 660, 661, 662, 663, 664, 665, 666, 667, 668, 669, 670,
        671, 672, 673, 674, 675, 676, 677, 678, 679, 680, 681, 682, 683, 684, 685, 686, 687, 688,
        689, 690, 691, 692, 693, 694, 695, 696, 697, 698, 699, 700, 701, 702, 703, 704, 705, 706,
        707, 708, 709, 710, 711, 712, 713, 714, 715, 716, 717, 718, 719
    ]
});

#[allow(dead_code)]
pub const ITEMS_SPRITE_0: SpriteRect = SpriteRect::item_at(0, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_48: SpriteRect = SpriteRect::item_at(0, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_96: SpriteRect = SpriteRect::item_at(0, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_144: SpriteRect = SpriteRect::item_at(0, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_192: SpriteRect = SpriteRect::item_at(0, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_240: SpriteRect = SpriteRect::item_at(0, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_288: SpriteRect = SpriteRect::item_at(0, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_336: SpriteRect = SpriteRect::item_at(0, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_384: SpriteRect = SpriteRect::item_at(0, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_432: SpriteRect = SpriteRect::item_at(0, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_480: SpriteRect = SpriteRect::item_at(0, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_528: SpriteRect = SpriteRect::item_at(0, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_576: SpriteRect = SpriteRect::item_at(0, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_624: SpriteRect = SpriteRect::item_at(0, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_672: SpriteRect = SpriteRect::item_at(0, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_1: SpriteRect = SpriteRect::item_at(1, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_49: SpriteRect = SpriteRect::item_at(1, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_97: SpriteRect = SpriteRect::item_at(1, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_145: SpriteRect = SpriteRect::item_at(1, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_193: SpriteRect = SpriteRect::item_at(1, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_241: SpriteRect = SpriteRect::item_at(1, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_289: SpriteRect = SpriteRect::item_at(1, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_337: SpriteRect = SpriteRect::item_at(1, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_385: SpriteRect = SpriteRect::item_at(1, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_433: SpriteRect = SpriteRect::item_at(1, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_481: SpriteRect = SpriteRect::item_at(1, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_529: SpriteRect = SpriteRect::item_at(1, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_577: SpriteRect = SpriteRect::item_at(1, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_625: SpriteRect = SpriteRect::item_at(1, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_673: SpriteRect = SpriteRect::item_at(1, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_2: SpriteRect = SpriteRect::item_at(2, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_50: SpriteRect = SpriteRect::item_at(2, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_98: SpriteRect = SpriteRect::item_at(2, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_146: SpriteRect = SpriteRect::item_at(2, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_194: SpriteRect = SpriteRect::item_at(2, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_242: SpriteRect = SpriteRect::item_at(2, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_290: SpriteRect = SpriteRect::item_at(2, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_338: SpriteRect = SpriteRect::item_at(2, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_386: SpriteRect = SpriteRect::item_at(2, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_434: SpriteRect = SpriteRect::item_at(2, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_482: SpriteRect = SpriteRect::item_at(2, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_530: SpriteRect = SpriteRect::item_at(2, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_578: SpriteRect = SpriteRect::item_at(2, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_626: SpriteRect = SpriteRect::item_at(2, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_674: SpriteRect = SpriteRect::item_at(2, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_3: SpriteRect = SpriteRect::item_at(3, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_51: SpriteRect = SpriteRect::item_at(3, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_99: SpriteRect = SpriteRect::item_at(3, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_147: SpriteRect = SpriteRect::item_at(3, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_195: SpriteRect = SpriteRect::item_at(3, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_243: SpriteRect = SpriteRect::item_at(3, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_291: SpriteRect = SpriteRect::item_at(3, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_339: SpriteRect = SpriteRect::item_at(3, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_387: SpriteRect = SpriteRect::item_at(3, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_435: SpriteRect = SpriteRect::item_at(3, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_483: SpriteRect = SpriteRect::item_at(3, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_531: SpriteRect = SpriteRect::item_at(3, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_579: SpriteRect = SpriteRect::item_at(3, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_627: SpriteRect = SpriteRect::item_at(3, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_675: SpriteRect = SpriteRect::item_at(3, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_4: SpriteRect = SpriteRect::item_at(4, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_52: SpriteRect = SpriteRect::item_at(4, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_100: SpriteRect = SpriteRect::item_at(4, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_148: SpriteRect = SpriteRect::item_at(4, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_196: SpriteRect = SpriteRect::item_at(4, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_244: SpriteRect = SpriteRect::item_at(4, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_292: SpriteRect = SpriteRect::item_at(4, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_340: SpriteRect = SpriteRect::item_at(4, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_388: SpriteRect = SpriteRect::item_at(4, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_436: SpriteRect = SpriteRect::item_at(4, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_484: SpriteRect = SpriteRect::item_at(4, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_532: SpriteRect = SpriteRect::item_at(4, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_580: SpriteRect = SpriteRect::item_at(4, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_628: SpriteRect = SpriteRect::item_at(4, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_676: SpriteRect = SpriteRect::item_at(4, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_5: SpriteRect = SpriteRect::item_at(5, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_53: SpriteRect = SpriteRect::item_at(5, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_101: SpriteRect = SpriteRect::item_at(5, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_149: SpriteRect = SpriteRect::item_at(5, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_197: SpriteRect = SpriteRect::item_at(5, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_245: SpriteRect = SpriteRect::item_at(5, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_293: SpriteRect = SpriteRect::item_at(5, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_341: SpriteRect = SpriteRect::item_at(5, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_389: SpriteRect = SpriteRect::item_at(5, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_437: SpriteRect = SpriteRect::item_at(5, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_485: SpriteRect = SpriteRect::item_at(5, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_533: SpriteRect = SpriteRect::item_at(5, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_581: SpriteRect = SpriteRect::item_at(5, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_629: SpriteRect = SpriteRect::item_at(5, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_677: SpriteRect = SpriteRect::item_at(5, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_6: SpriteRect = SpriteRect::item_at(6, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_54: SpriteRect = SpriteRect::item_at(6, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_102: SpriteRect = SpriteRect::item_at(6, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_150: SpriteRect = SpriteRect::item_at(6, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_198: SpriteRect = SpriteRect::item_at(6, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_246: SpriteRect = SpriteRect::item_at(6, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_294: SpriteRect = SpriteRect::item_at(6, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_342: SpriteRect = SpriteRect::item_at(6, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_390: SpriteRect = SpriteRect::item_at(6, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_438: SpriteRect = SpriteRect::item_at(6, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_486: SpriteRect = SpriteRect::item_at(6, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_534: SpriteRect = SpriteRect::item_at(6, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_582: SpriteRect = SpriteRect::item_at(6, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_630: SpriteRect = SpriteRect::item_at(6, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_678: SpriteRect = SpriteRect::item_at(6, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_7: SpriteRect = SpriteRect::item_at(7, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_55: SpriteRect = SpriteRect::item_at(7, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_103: SpriteRect = SpriteRect::item_at(7, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_151: SpriteRect = SpriteRect::item_at(7, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_199: SpriteRect = SpriteRect::item_at(7, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_247: SpriteRect = SpriteRect::item_at(7, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_295: SpriteRect = SpriteRect::item_at(7, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_343: SpriteRect = SpriteRect::item_at(7, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_391: SpriteRect = SpriteRect::item_at(7, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_439: SpriteRect = SpriteRect::item_at(7, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_487: SpriteRect = SpriteRect::item_at(7, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_535: SpriteRect = SpriteRect::item_at(7, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_583: SpriteRect = SpriteRect::item_at(7, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_631: SpriteRect = SpriteRect::item_at(7, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_679: SpriteRect = SpriteRect::item_at(7, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_8: SpriteRect = SpriteRect::item_at(8, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_56: SpriteRect = SpriteRect::item_at(8, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_104: SpriteRect = SpriteRect::item_at(8, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_152: SpriteRect = SpriteRect::item_at(8, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_200: SpriteRect = SpriteRect::item_at(8, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_248: SpriteRect = SpriteRect::item_at(8, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_296: SpriteRect = SpriteRect::item_at(8, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_344: SpriteRect = SpriteRect::item_at(8, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_392: SpriteRect = SpriteRect::item_at(8, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_440: SpriteRect = SpriteRect::item_at(8, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_488: SpriteRect = SpriteRect::item_at(8, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_536: SpriteRect = SpriteRect::item_at(8, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_584: SpriteRect = SpriteRect::item_at(8, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_632: SpriteRect = SpriteRect::item_at(8, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_680: SpriteRect = SpriteRect::item_at(8, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_9: SpriteRect = SpriteRect::item_at(9, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_57: SpriteRect = SpriteRect::item_at(9, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_105: SpriteRect = SpriteRect::item_at(9, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_153: SpriteRect = SpriteRect::item_at(9, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_201: SpriteRect = SpriteRect::item_at(9, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_249: SpriteRect = SpriteRect::item_at(9, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_297: SpriteRect = SpriteRect::item_at(9, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_345: SpriteRect = SpriteRect::item_at(9, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_393: SpriteRect = SpriteRect::item_at(9, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_441: SpriteRect = SpriteRect::item_at(9, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_489: SpriteRect = SpriteRect::item_at(9, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_537: SpriteRect = SpriteRect::item_at(9, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_585: SpriteRect = SpriteRect::item_at(9, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_633: SpriteRect = SpriteRect::item_at(9, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_681: SpriteRect = SpriteRect::item_at(9, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_10: SpriteRect = SpriteRect::item_at(10, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_58: SpriteRect = SpriteRect::item_at(10, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_106: SpriteRect = SpriteRect::item_at(10, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_154: SpriteRect = SpriteRect::item_at(10, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_202: SpriteRect = SpriteRect::item_at(10, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_250: SpriteRect = SpriteRect::item_at(10, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_298: SpriteRect = SpriteRect::item_at(10, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_346: SpriteRect = SpriteRect::item_at(10, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_394: SpriteRect = SpriteRect::item_at(10, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_442: SpriteRect = SpriteRect::item_at(10, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_490: SpriteRect = SpriteRect::item_at(10, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_538: SpriteRect = SpriteRect::item_at(10, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_586: SpriteRect = SpriteRect::item_at(10, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_634: SpriteRect = SpriteRect::item_at(10, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_682: SpriteRect = SpriteRect::item_at(10, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_11: SpriteRect = SpriteRect::item_at(11, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_59: SpriteRect = SpriteRect::item_at(11, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_107: SpriteRect = SpriteRect::item_at(11, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_155: SpriteRect = SpriteRect::item_at(11, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_203: SpriteRect = SpriteRect::item_at(11, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_251: SpriteRect = SpriteRect::item_at(11, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_299: SpriteRect = SpriteRect::item_at(11, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_347: SpriteRect = SpriteRect::item_at(11, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_395: SpriteRect = SpriteRect::item_at(11, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_443: SpriteRect = SpriteRect::item_at(11, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_491: SpriteRect = SpriteRect::item_at(11, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_539: SpriteRect = SpriteRect::item_at(11, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_587: SpriteRect = SpriteRect::item_at(11, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_635: SpriteRect = SpriteRect::item_at(11, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_683: SpriteRect = SpriteRect::item_at(11, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_12: SpriteRect = SpriteRect::item_at(12, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_60: SpriteRect = SpriteRect::item_at(12, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_108: SpriteRect = SpriteRect::item_at(12, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_156: SpriteRect = SpriteRect::item_at(12, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_204: SpriteRect = SpriteRect::item_at(12, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_252: SpriteRect = SpriteRect::item_at(12, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_300: SpriteRect = SpriteRect::item_at(12, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_348: SpriteRect = SpriteRect::item_at(12, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_396: SpriteRect = SpriteRect::item_at(12, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_444: SpriteRect = SpriteRect::item_at(12, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_492: SpriteRect = SpriteRect::item_at(12, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_540: SpriteRect = SpriteRect::item_at(12, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_588: SpriteRect = SpriteRect::item_at(12, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_636: SpriteRect = SpriteRect::item_at(12, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_684: SpriteRect = SpriteRect::item_at(12, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_13: SpriteRect = SpriteRect::item_at(13, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_61: SpriteRect = SpriteRect::item_at(13, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_109: SpriteRect = SpriteRect::item_at(13, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_157: SpriteRect = SpriteRect::item_at(13, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_205: SpriteRect = SpriteRect::item_at(13, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_253: SpriteRect = SpriteRect::item_at(13, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_301: SpriteRect = SpriteRect::item_at(13, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_349: SpriteRect = SpriteRect::item_at(13, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_397: SpriteRect = SpriteRect::item_at(13, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_445: SpriteRect = SpriteRect::item_at(13, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_493: SpriteRect = SpriteRect::item_at(13, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_541: SpriteRect = SpriteRect::item_at(13, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_589: SpriteRect = SpriteRect::item_at(13, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_637: SpriteRect = SpriteRect::item_at(13, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_685: SpriteRect = SpriteRect::item_at(13, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_14: SpriteRect = SpriteRect::item_at(14, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_62: SpriteRect = SpriteRect::item_at(14, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_110: SpriteRect = SpriteRect::item_at(14, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_158: SpriteRect = SpriteRect::item_at(14, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_206: SpriteRect = SpriteRect::item_at(14, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_254: SpriteRect = SpriteRect::item_at(14, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_302: SpriteRect = SpriteRect::item_at(14, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_350: SpriteRect = SpriteRect::item_at(14, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_398: SpriteRect = SpriteRect::item_at(14, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_446: SpriteRect = SpriteRect::item_at(14, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_494: SpriteRect = SpriteRect::item_at(14, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_542: SpriteRect = SpriteRect::item_at(14, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_590: SpriteRect = SpriteRect::item_at(14, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_638: SpriteRect = SpriteRect::item_at(14, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_686: SpriteRect = SpriteRect::item_at(14, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_15: SpriteRect = SpriteRect::item_at(15, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_63: SpriteRect = SpriteRect::item_at(15, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_111: SpriteRect = SpriteRect::item_at(15, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_159: SpriteRect = SpriteRect::item_at(15, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_207: SpriteRect = SpriteRect::item_at(15, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_255: SpriteRect = SpriteRect::item_at(15, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_303: SpriteRect = SpriteRect::item_at(15, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_351: SpriteRect = SpriteRect::item_at(15, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_399: SpriteRect = SpriteRect::item_at(15, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_447: SpriteRect = SpriteRect::item_at(15, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_495: SpriteRect = SpriteRect::item_at(15, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_543: SpriteRect = SpriteRect::item_at(15, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_591: SpriteRect = SpriteRect::item_at(15, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_639: SpriteRect = SpriteRect::item_at(15, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_687: SpriteRect = SpriteRect::item_at(15, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_16: SpriteRect = SpriteRect::item_at(16, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_64: SpriteRect = SpriteRect::item_at(16, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_112: SpriteRect = SpriteRect::item_at(16, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_160: SpriteRect = SpriteRect::item_at(16, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_208: SpriteRect = SpriteRect::item_at(16, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_256: SpriteRect = SpriteRect::item_at(16, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_304: SpriteRect = SpriteRect::item_at(16, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_352: SpriteRect = SpriteRect::item_at(16, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_400: SpriteRect = SpriteRect::item_at(16, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_448: SpriteRect = SpriteRect::item_at(16, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_496: SpriteRect = SpriteRect::item_at(16, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_544: SpriteRect = SpriteRect::item_at(16, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_592: SpriteRect = SpriteRect::item_at(16, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_640: SpriteRect = SpriteRect::item_at(16, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_688: SpriteRect = SpriteRect::item_at(16, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_17: SpriteRect = SpriteRect::item_at(17, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_65: SpriteRect = SpriteRect::item_at(17, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_113: SpriteRect = SpriteRect::item_at(17, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_161: SpriteRect = SpriteRect::item_at(17, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_209: SpriteRect = SpriteRect::item_at(17, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_257: SpriteRect = SpriteRect::item_at(17, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_305: SpriteRect = SpriteRect::item_at(17, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_353: SpriteRect = SpriteRect::item_at(17, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_401: SpriteRect = SpriteRect::item_at(17, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_449: SpriteRect = SpriteRect::item_at(17, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_497: SpriteRect = SpriteRect::item_at(17, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_545: SpriteRect = SpriteRect::item_at(17, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_593: SpriteRect = SpriteRect::item_at(17, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_641: SpriteRect = SpriteRect::item_at(17, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_689: SpriteRect = SpriteRect::item_at(17, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_18: SpriteRect = SpriteRect::item_at(18, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_66: SpriteRect = SpriteRect::item_at(18, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_114: SpriteRect = SpriteRect::item_at(18, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_162: SpriteRect = SpriteRect::item_at(18, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_210: SpriteRect = SpriteRect::item_at(18, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_258: SpriteRect = SpriteRect::item_at(18, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_306: SpriteRect = SpriteRect::item_at(18, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_354: SpriteRect = SpriteRect::item_at(18, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_402: SpriteRect = SpriteRect::item_at(18, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_450: SpriteRect = SpriteRect::item_at(18, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_498: SpriteRect = SpriteRect::item_at(18, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_546: SpriteRect = SpriteRect::item_at(18, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_594: SpriteRect = SpriteRect::item_at(18, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_642: SpriteRect = SpriteRect::item_at(18, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_690: SpriteRect = SpriteRect::item_at(18, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_19: SpriteRect = SpriteRect::item_at(19, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_67: SpriteRect = SpriteRect::item_at(19, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_115: SpriteRect = SpriteRect::item_at(19, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_163: SpriteRect = SpriteRect::item_at(19, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_211: SpriteRect = SpriteRect::item_at(19, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_259: SpriteRect = SpriteRect::item_at(19, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_307: SpriteRect = SpriteRect::item_at(19, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_355: SpriteRect = SpriteRect::item_at(19, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_403: SpriteRect = SpriteRect::item_at(19, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_451: SpriteRect = SpriteRect::item_at(19, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_499: SpriteRect = SpriteRect::item_at(19, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_547: SpriteRect = SpriteRect::item_at(19, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_595: SpriteRect = SpriteRect::item_at(19, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_643: SpriteRect = SpriteRect::item_at(19, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_691: SpriteRect = SpriteRect::item_at(19, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_20: SpriteRect = SpriteRect::item_at(20, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_68: SpriteRect = SpriteRect::item_at(20, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_116: SpriteRect = SpriteRect::item_at(20, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_164: SpriteRect = SpriteRect::item_at(20, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_212: SpriteRect = SpriteRect::item_at(20, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_260: SpriteRect = SpriteRect::item_at(20, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_308: SpriteRect = SpriteRect::item_at(20, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_356: SpriteRect = SpriteRect::item_at(20, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_404: SpriteRect = SpriteRect::item_at(20, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_452: SpriteRect = SpriteRect::item_at(20, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_500: SpriteRect = SpriteRect::item_at(20, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_548: SpriteRect = SpriteRect::item_at(20, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_596: SpriteRect = SpriteRect::item_at(20, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_644: SpriteRect = SpriteRect::item_at(20, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_692: SpriteRect = SpriteRect::item_at(20, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_21: SpriteRect = SpriteRect::item_at(21, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_69: SpriteRect = SpriteRect::item_at(21, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_117: SpriteRect = SpriteRect::item_at(21, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_165: SpriteRect = SpriteRect::item_at(21, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_213: SpriteRect = SpriteRect::item_at(21, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_261: SpriteRect = SpriteRect::item_at(21, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_309: SpriteRect = SpriteRect::item_at(21, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_357: SpriteRect = SpriteRect::item_at(21, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_405: SpriteRect = SpriteRect::item_at(21, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_453: SpriteRect = SpriteRect::item_at(21, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_501: SpriteRect = SpriteRect::item_at(21, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_549: SpriteRect = SpriteRect::item_at(21, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_597: SpriteRect = SpriteRect::item_at(21, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_645: SpriteRect = SpriteRect::item_at(21, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_693: SpriteRect = SpriteRect::item_at(21, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_22: SpriteRect = SpriteRect::item_at(22, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_70: SpriteRect = SpriteRect::item_at(22, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_118: SpriteRect = SpriteRect::item_at(22, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_166: SpriteRect = SpriteRect::item_at(22, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_214: SpriteRect = SpriteRect::item_at(22, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_262: SpriteRect = SpriteRect::item_at(22, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_310: SpriteRect = SpriteRect::item_at(22, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_358: SpriteRect = SpriteRect::item_at(22, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_406: SpriteRect = SpriteRect::item_at(22, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_454: SpriteRect = SpriteRect::item_at(22, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_502: SpriteRect = SpriteRect::item_at(22, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_550: SpriteRect = SpriteRect::item_at(22, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_598: SpriteRect = SpriteRect::item_at(22, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_646: SpriteRect = SpriteRect::item_at(22, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_694: SpriteRect = SpriteRect::item_at(22, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_23: SpriteRect = SpriteRect::item_at(23, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_71: SpriteRect = SpriteRect::item_at(23, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_119: SpriteRect = SpriteRect::item_at(23, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_167: SpriteRect = SpriteRect::item_at(23, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_215: SpriteRect = SpriteRect::item_at(23, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_263: SpriteRect = SpriteRect::item_at(23, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_311: SpriteRect = SpriteRect::item_at(23, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_359: SpriteRect = SpriteRect::item_at(23, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_407: SpriteRect = SpriteRect::item_at(23, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_455: SpriteRect = SpriteRect::item_at(23, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_503: SpriteRect = SpriteRect::item_at(23, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_551: SpriteRect = SpriteRect::item_at(23, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_599: SpriteRect = SpriteRect::item_at(23, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_647: SpriteRect = SpriteRect::item_at(23, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_695: SpriteRect = SpriteRect::item_at(23, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_24: SpriteRect = SpriteRect::item_at(24, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_72: SpriteRect = SpriteRect::item_at(24, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_120: SpriteRect = SpriteRect::item_at(24, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_168: SpriteRect = SpriteRect::item_at(24, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_216: SpriteRect = SpriteRect::item_at(24, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_264: SpriteRect = SpriteRect::item_at(24, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_312: SpriteRect = SpriteRect::item_at(24, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_360: SpriteRect = SpriteRect::item_at(24, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_408: SpriteRect = SpriteRect::item_at(24, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_456: SpriteRect = SpriteRect::item_at(24, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_504: SpriteRect = SpriteRect::item_at(24, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_552: SpriteRect = SpriteRect::item_at(24, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_600: SpriteRect = SpriteRect::item_at(24, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_648: SpriteRect = SpriteRect::item_at(24, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_696: SpriteRect = SpriteRect::item_at(24, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_25: SpriteRect = SpriteRect::item_at(25, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_73: SpriteRect = SpriteRect::item_at(25, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_121: SpriteRect = SpriteRect::item_at(25, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_169: SpriteRect = SpriteRect::item_at(25, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_217: SpriteRect = SpriteRect::item_at(25, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_265: SpriteRect = SpriteRect::item_at(25, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_313: SpriteRect = SpriteRect::item_at(25, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_361: SpriteRect = SpriteRect::item_at(25, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_409: SpriteRect = SpriteRect::item_at(25, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_457: SpriteRect = SpriteRect::item_at(25, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_505: SpriteRect = SpriteRect::item_at(25, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_553: SpriteRect = SpriteRect::item_at(25, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_601: SpriteRect = SpriteRect::item_at(25, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_649: SpriteRect = SpriteRect::item_at(25, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_697: SpriteRect = SpriteRect::item_at(25, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_26: SpriteRect = SpriteRect::item_at(26, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_74: SpriteRect = SpriteRect::item_at(26, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_122: SpriteRect = SpriteRect::item_at(26, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_170: SpriteRect = SpriteRect::item_at(26, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_218: SpriteRect = SpriteRect::item_at(26, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_266: SpriteRect = SpriteRect::item_at(26, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_314: SpriteRect = SpriteRect::item_at(26, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_362: SpriteRect = SpriteRect::item_at(26, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_410: SpriteRect = SpriteRect::item_at(26, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_458: SpriteRect = SpriteRect::item_at(26, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_506: SpriteRect = SpriteRect::item_at(26, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_554: SpriteRect = SpriteRect::item_at(26, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_602: SpriteRect = SpriteRect::item_at(26, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_650: SpriteRect = SpriteRect::item_at(26, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_698: SpriteRect = SpriteRect::item_at(26, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_27: SpriteRect = SpriteRect::item_at(27, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_75: SpriteRect = SpriteRect::item_at(27, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_123: SpriteRect = SpriteRect::item_at(27, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_171: SpriteRect = SpriteRect::item_at(27, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_219: SpriteRect = SpriteRect::item_at(27, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_267: SpriteRect = SpriteRect::item_at(27, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_315: SpriteRect = SpriteRect::item_at(27, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_363: SpriteRect = SpriteRect::item_at(27, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_411: SpriteRect = SpriteRect::item_at(27, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_459: SpriteRect = SpriteRect::item_at(27, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_507: SpriteRect = SpriteRect::item_at(27, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_555: SpriteRect = SpriteRect::item_at(27, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_603: SpriteRect = SpriteRect::item_at(27, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_651: SpriteRect = SpriteRect::item_at(27, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_699: SpriteRect = SpriteRect::item_at(27, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_28: SpriteRect = SpriteRect::item_at(28, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_76: SpriteRect = SpriteRect::item_at(28, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_124: SpriteRect = SpriteRect::item_at(28, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_172: SpriteRect = SpriteRect::item_at(28, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_220: SpriteRect = SpriteRect::item_at(28, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_268: SpriteRect = SpriteRect::item_at(28, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_316: SpriteRect = SpriteRect::item_at(28, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_364: SpriteRect = SpriteRect::item_at(28, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_412: SpriteRect = SpriteRect::item_at(28, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_460: SpriteRect = SpriteRect::item_at(28, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_508: SpriteRect = SpriteRect::item_at(28, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_556: SpriteRect = SpriteRect::item_at(28, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_604: SpriteRect = SpriteRect::item_at(28, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_652: SpriteRect = SpriteRect::item_at(28, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_700: SpriteRect = SpriteRect::item_at(28, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_29: SpriteRect = SpriteRect::item_at(29, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_77: SpriteRect = SpriteRect::item_at(29, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_125: SpriteRect = SpriteRect::item_at(29, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_173: SpriteRect = SpriteRect::item_at(29, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_221: SpriteRect = SpriteRect::item_at(29, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_269: SpriteRect = SpriteRect::item_at(29, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_317: SpriteRect = SpriteRect::item_at(29, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_365: SpriteRect = SpriteRect::item_at(29, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_413: SpriteRect = SpriteRect::item_at(29, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_461: SpriteRect = SpriteRect::item_at(29, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_509: SpriteRect = SpriteRect::item_at(29, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_557: SpriteRect = SpriteRect::item_at(29, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_605: SpriteRect = SpriteRect::item_at(29, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_653: SpriteRect = SpriteRect::item_at(29, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_701: SpriteRect = SpriteRect::item_at(29, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_30: SpriteRect = SpriteRect::item_at(30, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_78: SpriteRect = SpriteRect::item_at(30, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_126: SpriteRect = SpriteRect::item_at(30, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_174: SpriteRect = SpriteRect::item_at(30, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_222: SpriteRect = SpriteRect::item_at(30, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_270: SpriteRect = SpriteRect::item_at(30, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_318: SpriteRect = SpriteRect::item_at(30, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_366: SpriteRect = SpriteRect::item_at(30, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_414: SpriteRect = SpriteRect::item_at(30, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_462: SpriteRect = SpriteRect::item_at(30, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_510: SpriteRect = SpriteRect::item_at(30, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_558: SpriteRect = SpriteRect::item_at(30, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_606: SpriteRect = SpriteRect::item_at(30, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_654: SpriteRect = SpriteRect::item_at(30, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_702: SpriteRect = SpriteRect::item_at(30, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_31: SpriteRect = SpriteRect::item_at(31, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_79: SpriteRect = SpriteRect::item_at(31, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_127: SpriteRect = SpriteRect::item_at(31, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_175: SpriteRect = SpriteRect::item_at(31, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_223: SpriteRect = SpriteRect::item_at(31, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_271: SpriteRect = SpriteRect::item_at(31, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_319: SpriteRect = SpriteRect::item_at(31, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_367: SpriteRect = SpriteRect::item_at(31, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_415: SpriteRect = SpriteRect::item_at(31, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_463: SpriteRect = SpriteRect::item_at(31, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_511: SpriteRect = SpriteRect::item_at(31, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_559: SpriteRect = SpriteRect::item_at(31, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_607: SpriteRect = SpriteRect::item_at(31, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_655: SpriteRect = SpriteRect::item_at(31, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_703: SpriteRect = SpriteRect::item_at(31, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_32: SpriteRect = SpriteRect::item_at(32, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_80: SpriteRect = SpriteRect::item_at(32, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_128: SpriteRect = SpriteRect::item_at(32, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_176: SpriteRect = SpriteRect::item_at(32, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_224: SpriteRect = SpriteRect::item_at(32, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_272: SpriteRect = SpriteRect::item_at(32, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_320: SpriteRect = SpriteRect::item_at(32, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_368: SpriteRect = SpriteRect::item_at(32, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_416: SpriteRect = SpriteRect::item_at(32, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_464: SpriteRect = SpriteRect::item_at(32, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_512: SpriteRect = SpriteRect::item_at(32, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_560: SpriteRect = SpriteRect::item_at(32, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_608: SpriteRect = SpriteRect::item_at(32, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_656: SpriteRect = SpriteRect::item_at(32, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_704: SpriteRect = SpriteRect::item_at(32, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_33: SpriteRect = SpriteRect::item_at(33, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_81: SpriteRect = SpriteRect::item_at(33, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_129: SpriteRect = SpriteRect::item_at(33, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_177: SpriteRect = SpriteRect::item_at(33, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_225: SpriteRect = SpriteRect::item_at(33, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_273: SpriteRect = SpriteRect::item_at(33, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_321: SpriteRect = SpriteRect::item_at(33, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_369: SpriteRect = SpriteRect::item_at(33, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_417: SpriteRect = SpriteRect::item_at(33, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_465: SpriteRect = SpriteRect::item_at(33, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_513: SpriteRect = SpriteRect::item_at(33, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_561: SpriteRect = SpriteRect::item_at(33, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_609: SpriteRect = SpriteRect::item_at(33, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_657: SpriteRect = SpriteRect::item_at(33, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_705: SpriteRect = SpriteRect::item_at(33, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_34: SpriteRect = SpriteRect::item_at(34, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_82: SpriteRect = SpriteRect::item_at(34, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_130: SpriteRect = SpriteRect::item_at(34, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_178: SpriteRect = SpriteRect::item_at(34, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_226: SpriteRect = SpriteRect::item_at(34, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_274: SpriteRect = SpriteRect::item_at(34, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_322: SpriteRect = SpriteRect::item_at(34, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_370: SpriteRect = SpriteRect::item_at(34, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_418: SpriteRect = SpriteRect::item_at(34, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_466: SpriteRect = SpriteRect::item_at(34, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_514: SpriteRect = SpriteRect::item_at(34, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_562: SpriteRect = SpriteRect::item_at(34, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_610: SpriteRect = SpriteRect::item_at(34, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_658: SpriteRect = SpriteRect::item_at(34, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_706: SpriteRect = SpriteRect::item_at(34, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_35: SpriteRect = SpriteRect::item_at(35, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_83: SpriteRect = SpriteRect::item_at(35, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_131: SpriteRect = SpriteRect::item_at(35, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_179: SpriteRect = SpriteRect::item_at(35, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_227: SpriteRect = SpriteRect::item_at(35, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_275: SpriteRect = SpriteRect::item_at(35, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_323: SpriteRect = SpriteRect::item_at(35, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_371: SpriteRect = SpriteRect::item_at(35, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_419: SpriteRect = SpriteRect::item_at(35, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_467: SpriteRect = SpriteRect::item_at(35, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_515: SpriteRect = SpriteRect::item_at(35, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_563: SpriteRect = SpriteRect::item_at(35, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_611: SpriteRect = SpriteRect::item_at(35, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_659: SpriteRect = SpriteRect::item_at(35, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_707: SpriteRect = SpriteRect::item_at(35, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_36: SpriteRect = SpriteRect::item_at(36, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_84: SpriteRect = SpriteRect::item_at(36, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_132: SpriteRect = SpriteRect::item_at(36, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_180: SpriteRect = SpriteRect::item_at(36, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_228: SpriteRect = SpriteRect::item_at(36, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_276: SpriteRect = SpriteRect::item_at(36, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_324: SpriteRect = SpriteRect::item_at(36, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_372: SpriteRect = SpriteRect::item_at(36, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_420: SpriteRect = SpriteRect::item_at(36, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_468: SpriteRect = SpriteRect::item_at(36, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_516: SpriteRect = SpriteRect::item_at(36, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_564: SpriteRect = SpriteRect::item_at(36, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_612: SpriteRect = SpriteRect::item_at(36, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_660: SpriteRect = SpriteRect::item_at(36, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_708: SpriteRect = SpriteRect::item_at(36, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_37: SpriteRect = SpriteRect::item_at(37, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_85: SpriteRect = SpriteRect::item_at(37, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_133: SpriteRect = SpriteRect::item_at(37, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_181: SpriteRect = SpriteRect::item_at(37, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_229: SpriteRect = SpriteRect::item_at(37, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_277: SpriteRect = SpriteRect::item_at(37, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_325: SpriteRect = SpriteRect::item_at(37, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_373: SpriteRect = SpriteRect::item_at(37, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_421: SpriteRect = SpriteRect::item_at(37, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_469: SpriteRect = SpriteRect::item_at(37, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_517: SpriteRect = SpriteRect::item_at(37, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_565: SpriteRect = SpriteRect::item_at(37, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_613: SpriteRect = SpriteRect::item_at(37, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_661: SpriteRect = SpriteRect::item_at(37, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_709: SpriteRect = SpriteRect::item_at(37, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_38: SpriteRect = SpriteRect::item_at(38, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_86: SpriteRect = SpriteRect::item_at(38, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_134: SpriteRect = SpriteRect::item_at(38, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_182: SpriteRect = SpriteRect::item_at(38, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_230: SpriteRect = SpriteRect::item_at(38, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_278: SpriteRect = SpriteRect::item_at(38, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_326: SpriteRect = SpriteRect::item_at(38, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_374: SpriteRect = SpriteRect::item_at(38, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_422: SpriteRect = SpriteRect::item_at(38, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_470: SpriteRect = SpriteRect::item_at(38, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_518: SpriteRect = SpriteRect::item_at(38, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_566: SpriteRect = SpriteRect::item_at(38, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_614: SpriteRect = SpriteRect::item_at(38, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_662: SpriteRect = SpriteRect::item_at(38, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_710: SpriteRect = SpriteRect::item_at(38, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_39: SpriteRect = SpriteRect::item_at(39, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_87: SpriteRect = SpriteRect::item_at(39, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_135: SpriteRect = SpriteRect::item_at(39, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_183: SpriteRect = SpriteRect::item_at(39, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_231: SpriteRect = SpriteRect::item_at(39, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_279: SpriteRect = SpriteRect::item_at(39, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_327: SpriteRect = SpriteRect::item_at(39, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_375: SpriteRect = SpriteRect::item_at(39, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_423: SpriteRect = SpriteRect::item_at(39, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_471: SpriteRect = SpriteRect::item_at(39, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_519: SpriteRect = SpriteRect::item_at(39, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_567: SpriteRect = SpriteRect::item_at(39, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_615: SpriteRect = SpriteRect::item_at(39, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_663: SpriteRect = SpriteRect::item_at(39, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_711: SpriteRect = SpriteRect::item_at(39, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_40: SpriteRect = SpriteRect::item_at(40, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_88: SpriteRect = SpriteRect::item_at(40, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_136: SpriteRect = SpriteRect::item_at(40, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_184: SpriteRect = SpriteRect::item_at(40, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_232: SpriteRect = SpriteRect::item_at(40, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_280: SpriteRect = SpriteRect::item_at(40, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_328: SpriteRect = SpriteRect::item_at(40, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_376: SpriteRect = SpriteRect::item_at(40, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_424: SpriteRect = SpriteRect::item_at(40, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_472: SpriteRect = SpriteRect::item_at(40, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_520: SpriteRect = SpriteRect::item_at(40, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_568: SpriteRect = SpriteRect::item_at(40, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_616: SpriteRect = SpriteRect::item_at(40, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_664: SpriteRect = SpriteRect::item_at(40, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_712: SpriteRect = SpriteRect::item_at(40, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_41: SpriteRect = SpriteRect::item_at(41, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_89: SpriteRect = SpriteRect::item_at(41, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_137: SpriteRect = SpriteRect::item_at(41, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_185: SpriteRect = SpriteRect::item_at(41, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_233: SpriteRect = SpriteRect::item_at(41, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_281: SpriteRect = SpriteRect::item_at(41, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_329: SpriteRect = SpriteRect::item_at(41, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_377: SpriteRect = SpriteRect::item_at(41, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_425: SpriteRect = SpriteRect::item_at(41, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_473: SpriteRect = SpriteRect::item_at(41, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_521: SpriteRect = SpriteRect::item_at(41, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_569: SpriteRect = SpriteRect::item_at(41, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_617: SpriteRect = SpriteRect::item_at(41, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_665: SpriteRect = SpriteRect::item_at(41, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_713: SpriteRect = SpriteRect::item_at(41, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_42: SpriteRect = SpriteRect::item_at(42, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_90: SpriteRect = SpriteRect::item_at(42, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_138: SpriteRect = SpriteRect::item_at(42, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_186: SpriteRect = SpriteRect::item_at(42, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_234: SpriteRect = SpriteRect::item_at(42, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_282: SpriteRect = SpriteRect::item_at(42, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_330: SpriteRect = SpriteRect::item_at(42, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_378: SpriteRect = SpriteRect::item_at(42, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_426: SpriteRect = SpriteRect::item_at(42, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_474: SpriteRect = SpriteRect::item_at(42, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_522: SpriteRect = SpriteRect::item_at(42, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_570: SpriteRect = SpriteRect::item_at(42, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_618: SpriteRect = SpriteRect::item_at(42, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_666: SpriteRect = SpriteRect::item_at(42, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_714: SpriteRect = SpriteRect::item_at(42, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_43: SpriteRect = SpriteRect::item_at(43, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_91: SpriteRect = SpriteRect::item_at(43, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_139: SpriteRect = SpriteRect::item_at(43, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_187: SpriteRect = SpriteRect::item_at(43, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_235: SpriteRect = SpriteRect::item_at(43, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_283: SpriteRect = SpriteRect::item_at(43, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_331: SpriteRect = SpriteRect::item_at(43, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_379: SpriteRect = SpriteRect::item_at(43, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_427: SpriteRect = SpriteRect::item_at(43, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_475: SpriteRect = SpriteRect::item_at(43, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_523: SpriteRect = SpriteRect::item_at(43, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_571: SpriteRect = SpriteRect::item_at(43, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_619: SpriteRect = SpriteRect::item_at(43, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_667: SpriteRect = SpriteRect::item_at(43, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_715: SpriteRect = SpriteRect::item_at(43, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_44: SpriteRect = SpriteRect::item_at(44, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_92: SpriteRect = SpriteRect::item_at(44, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_140: SpriteRect = SpriteRect::item_at(44, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_188: SpriteRect = SpriteRect::item_at(44, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_236: SpriteRect = SpriteRect::item_at(44, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_284: SpriteRect = SpriteRect::item_at(44, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_332: SpriteRect = SpriteRect::item_at(44, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_380: SpriteRect = SpriteRect::item_at(44, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_428: SpriteRect = SpriteRect::item_at(44, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_476: SpriteRect = SpriteRect::item_at(44, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_524: SpriteRect = SpriteRect::item_at(44, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_572: SpriteRect = SpriteRect::item_at(44, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_620: SpriteRect = SpriteRect::item_at(44, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_668: SpriteRect = SpriteRect::item_at(44, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_716: SpriteRect = SpriteRect::item_at(44, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_45: SpriteRect = SpriteRect::item_at(45, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_93: SpriteRect = SpriteRect::item_at(45, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_141: SpriteRect = SpriteRect::item_at(45, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_189: SpriteRect = SpriteRect::item_at(45, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_237: SpriteRect = SpriteRect::item_at(45, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_285: SpriteRect = SpriteRect::item_at(45, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_333: SpriteRect = SpriteRect::item_at(45, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_381: SpriteRect = SpriteRect::item_at(45, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_429: SpriteRect = SpriteRect::item_at(45, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_477: SpriteRect = SpriteRect::item_at(45, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_525: SpriteRect = SpriteRect::item_at(45, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_573: SpriteRect = SpriteRect::item_at(45, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_621: SpriteRect = SpriteRect::item_at(45, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_669: SpriteRect = SpriteRect::item_at(45, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_717: SpriteRect = SpriteRect::item_at(45, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_46: SpriteRect = SpriteRect::item_at(46, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_94: SpriteRect = SpriteRect::item_at(46, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_142: SpriteRect = SpriteRect::item_at(46, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_190: SpriteRect = SpriteRect::item_at(46, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_238: SpriteRect = SpriteRect::item_at(46, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_286: SpriteRect = SpriteRect::item_at(46, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_334: SpriteRect = SpriteRect::item_at(46, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_382: SpriteRect = SpriteRect::item_at(46, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_430: SpriteRect = SpriteRect::item_at(46, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_478: SpriteRect = SpriteRect::item_at(46, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_526: SpriteRect = SpriteRect::item_at(46, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_574: SpriteRect = SpriteRect::item_at(46, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_622: SpriteRect = SpriteRect::item_at(46, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_670: SpriteRect = SpriteRect::item_at(46, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_718: SpriteRect = SpriteRect::item_at(46, 14);
#[allow(dead_code)]
pub const ITEMS_SPRITE_47: SpriteRect = SpriteRect::item_at(47, 0);
#[allow(dead_code)]
pub const ITEMS_SPRITE_95: SpriteRect = SpriteRect::item_at(47, 1);
#[allow(dead_code)]
pub const ITEMS_SPRITE_143: SpriteRect = SpriteRect::item_at(47, 2);
#[allow(dead_code)]
pub const ITEMS_SPRITE_191: SpriteRect = SpriteRect::item_at(47, 3);
#[allow(dead_code)]
pub const ITEMS_SPRITE_239: SpriteRect = SpriteRect::item_at(47, 4);
#[allow(dead_code)]
pub const ITEMS_SPRITE_287: SpriteRect = SpriteRect::item_at(47, 5);
#[allow(dead_code)]
pub const ITEMS_SPRITE_335: SpriteRect = SpriteRect::item_at(47, 6);
#[allow(dead_code)]
pub const ITEMS_SPRITE_383: SpriteRect = SpriteRect::item_at(47, 7);
#[allow(dead_code)]
pub const ITEMS_SPRITE_431: SpriteRect = SpriteRect::item_at(47, 8);
#[allow(dead_code)]
pub const ITEMS_SPRITE_479: SpriteRect = SpriteRect::item_at(47, 9);
#[allow(dead_code)]
pub const ITEMS_SPRITE_527: SpriteRect = SpriteRect::item_at(47, 10);
#[allow(dead_code)]
pub const ITEMS_SPRITE_575: SpriteRect = SpriteRect::item_at(47, 11);
#[allow(dead_code)]
pub const ITEMS_SPRITE_623: SpriteRect = SpriteRect::item_at(47, 12);
#[allow(dead_code)]
pub const ITEMS_SPRITE_671: SpriteRect = SpriteRect::item_at(47, 13);
#[allow(dead_code)]
pub const ITEMS_SPRITE_719: SpriteRect = SpriteRect::item_at(47, 14);
