use std::ffi::CString;
use crate::gameobjects::item_handler::{Equipmintslots, Item, Raritys};
use crate::gameobjects::passiv_handler::PassivTypes;

#[derive(Clone)]
pub struct TreasureItem {
    name: String,
    equipmintslots: Equipmintslots,
    des: String,
    passiv: PassivTypes,
    rarity: Raritys
}

impl TreasureItem {
    pub fn new(name: String, equipmintslots: Equipmintslots, des:String, passiv: String, rarity: Raritys) -> Self {
        Self {
            name,
            equipmintslots,
            des,
            passiv: PassivTypes::create_passiv(passiv),
            rarity
        }
    }
}

impl Item for TreasureItem {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_equipment_slot(&self) -> &Equipmintslots {
        &self.equipmintslots
    }

    fn get_des(&self) -> &str {
        &self.des
    }

    fn get_rarity(&self) -> &Raritys {
        &self.rarity
    }
}