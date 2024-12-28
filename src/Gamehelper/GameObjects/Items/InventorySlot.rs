use std::net::Incoming;
use crate::gameobjects::item_handler::{Equipmintslots, Raritys};
use crate::gameobjects::item_handler::Item;

#[derive(Clone)]
pub(crate) struct Inventoryslot{
    name: String,
    eq_slot:  Equipmintslots,
    rarity: Raritys,
    value: i8
}

 impl Inventoryslot {


    pub fn empty()-> Self {
       Self {
            name: "Empty".into(),
            eq_slot: Equipmintslots::None,
            rarity: Raritys::COMMON,
            value: 0
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
    fn get_value(&self) -> &i8 {
        &self.value
    }

    fn get_bonus_dmg(&self) -> &u8 {
        &0
    }


}
