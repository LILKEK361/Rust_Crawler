use crate::gameobjects::encounter::{Encounter, EncounterTypes};
use crate::gameobjects::item_handler::{Equipmintslots, Item, Raritys};

#[derive(Clone)]
pub(crate) struct WeaponItem {

    //Common stats for item
    name: String,
    equip_slot: Vec<Equipmintslots>,
    des: String,
    rarity: Raritys,
    //Specific WeaponItem stats:
    dmg_bonus: u8,
}

impl WeaponItem {
    fn get_bonus_dmg(&self){

    }
}

impl Item for WeaponItem {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_equipment_slot(&self) -> Vec<crate::gameobjects::item_handler::Equipmintslots> {
        self.equip_slot.clone()
    }

    fn get_des(&self) -> &str {
        &self.des
    }
    fn get_rarity(&self) -> &Raritys {
        &self.rarity
    }
}