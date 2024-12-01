use crate::gameobjects::encounter::{Encounter, EncounterTypes};
use crate::gameobjects::item_handler::{Equipmintslots, Item};

struct WeaponItem {

    //Common stats for item
    name: String,
    equip_slot: Vec<Equipmintslots>,
    des: String,

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

    fn get_equipment_slot(&self) -> &[Equipmintslots] {
        &self.equip_slot
    }
    
    fn get_des(&self) -> &str {
        &self.des
    }
}