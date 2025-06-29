use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    gen_sprites("terrain", 35, 33);
    gen_sprites("monsters", 36, 26);
    gen_sprites("items", 48, 15);
    gen_sprites("damage_fx", 3, 10);
    gen_sprites("spellfx_anim_1", 8, 21);
    gen_sprites("spellfx_anim_2", 8, 21);
    gen_sprites("spellfx_anim_3", 8, 21);
    gen_sprites("spellfx_anim_4", 8, 21);
    gen_sprites("spellfx_anim_5", 8, 21);
    gen_sprites("spellfx_missiles", 8, 19);
}

fn gen_sprites(category: &str, x: usize, y: usize) {
    let out_dir = env::var("CARGO_TARGET_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join(format!("{}_sprites.rs", category));
    let mut f = File::create(&dest_path).unwrap();
    writeln!(f, "use crate::sprites::SpriteRect;").unwrap();
    for col in 0..x {
        for row in 0..y {
            writeln!(f, "#[allow(dead_code)]").unwrap();
            writeln!(
                f,
                "pub const {}_SPRITE_{}: SpriteRect = SpriteRect::at({}.0, {}.0);",
                category.to_uppercase(),
                row * x + col,
                col,
                row
            ).unwrap();
        }
    }
}