use crate::ecs::assets::ASSETS;
use common::Spell;
use std::collections::HashMap;

#[derive(Default)]
pub struct AllSpells(pub HashMap<u32, Spell>);

impl AllSpells {
    pub fn new() -> Self {
        let assets = ASSETS
            .load_rec_dir::<Spell>("assets.spells")
            .expect("Failed to load spells");

        let all_spells: HashMap<u32, Spell> = assets
            .read()
            .ids()
            .filter_map(|id| ASSETS.load::<Spell>(id).ok()?.read().clone().into())
            .map(|spell: Spell| (spell.id, spell))
            .collect();
        // dbg!(&all_spells);
        AllSpells(all_spells)
    }
}
