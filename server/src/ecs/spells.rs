use crate::ecs::assets::ASSETS;
use common::Spell;

#[derive(Default)]
pub struct AllSpells {
    pub all_spells: Vec<Spell>,
}

impl AllSpells {
    pub fn new() -> Self {
        let assets = ASSETS
            .load_rec_dir::<Spell>("assets.spells")
            .expect("Failed to load spells");

        let all_spells = assets
            .read()
            .ids()
            .filter_map(|id| {
                let spell = ASSETS.load::<Spell>(id).ok()?.read().clone();
                Some(spell)
            })
            .collect();

        dbg!(&all_spells);
        AllSpells { all_spells }
    }
}
