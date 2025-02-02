use crate::gameobjects::item_handler::{Equipmintslots, Item, Raritys};
use crate::gameobjects::passiv_handler::PassivTypes;
use std::collections::hash_map::Values;
use std::ffi::CString;

#[derive(Clone)]
pub struct TreasureItem {
    name: String,
    equipmintslots: Equipmintslots,
    des: String,
    passiv: PassivTypes,
    rarity: Raritys,
    value: u8,
    bonus_dmg: u8,
}

impl TreasureItem {
    pub fn new(
        name: String,
        equipmintslots: Equipmintslots,
        des: String,
        passiv: String,
        rarity: Raritys,
        value: u8,
    ) -> Self {
        Self {
            name,
            equipmintslots,
            des,
            passiv: PassivTypes::create_passiv(passiv),
            rarity,
            value,
            bonus_dmg: 5,
        }
    }

    pub fn get_passiv(&self) -> &PassivTypes {
        &self.passiv
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

    fn get_value(&self) -> &u8 {
        &self.value
    }

    fn get_bonus_dmg(&self) -> &u8 {
        &self.bonus_dmg
    }
}
