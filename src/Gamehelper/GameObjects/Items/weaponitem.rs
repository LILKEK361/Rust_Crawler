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

    pub fn new(name: String, des: String, rarity: Raritys, dmg_bonus: u8) -> Self {

        Self {
            name,
            equip_slot: vec![Equipmintslots::Hands],
            des,
            rarity,
            dmg_bonus
        }

    }
    fn get_bonus_dmg(&self) -> &u8{
        &self.dmg_bonus
    }

    pub fn get_stats(&self) -> () {

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