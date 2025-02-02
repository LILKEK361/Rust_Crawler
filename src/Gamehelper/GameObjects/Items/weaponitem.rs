use crate::gameobjects::encounter::{Encounter, EncounterTypes};
use crate::gameobjects::item_handler::{Equipmintslots, Item, Raritys};

#[derive(Clone)]
pub(crate) struct WeaponItem {
    //Common stats for item
    name: String,
    equip_slot: Equipmintslots,
    des: String,
    rarity: Raritys,
    //Specific WeaponItem stats:
    dmg_bonus: u8,
    value: u8,
}

impl WeaponItem {
    pub fn new(name: String, des: String, rarity: Raritys, dmg_bonus: u8, value: u8) -> Self {
        Self {
            name,
            equip_slot: Equipmintslots::Hands,
            des,
            rarity,
            dmg_bonus,
            value,
        }
    }

    pub fn get_stats(&self) -> () {}
}

impl Item for WeaponItem {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_equipment_slot(&self) -> &Equipmintslots {
        &self.equip_slot
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
        &self.dmg_bonus
    }
}
