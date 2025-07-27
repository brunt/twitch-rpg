use std::fs::File;
use std::io::Write;

fn main() {
    let mut file = File::create("src/sprites/item_sprite_map_gen.rs").unwrap();
    write!(file, "item_sprite_map![").unwrap();
    for i in 0..720 {
        write!(file, "{},", i).unwrap();
    }
    writeln!(file, "];").unwrap();
}
