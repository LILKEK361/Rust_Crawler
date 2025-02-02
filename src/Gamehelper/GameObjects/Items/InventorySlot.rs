use crate::gameobjects::item_handler::Item;
use crate::gameobjects::item_handler::{Equipmintslots, Raritys};
use std::net::Incoming;

#[derive(Clone)]
pub(crate) struct Inventoryslot {
    name: String,
    eq_slot: Equipmintslots,
    rarity: Raritys,
}

impl Inventoryslot {
    pub fn empty() -> Self {
        Self {
            name: "Empty".into(),
            eq_slot: Equipmintslots::None,
            rarity: Raritys::COMMON,
        }
    }
}

impl Item for Inventoryslot {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_equipment_slot(&self) -> &Equipmintslots {
        &self.eq_slot
    }

    fn get_des(&self) -> &str {
        "Empty"
    }
    fn get_rarity(&self) -> &Raritys {
        &self.rarity
    }
    fn get_value(&self) -> &u8 {
        &0
    }

    fn get_bonus_dmg(&self) -> &u8 {
        &0
    }
}
