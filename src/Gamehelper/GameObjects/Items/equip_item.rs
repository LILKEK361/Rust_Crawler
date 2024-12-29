use crate::gameobjects::item_handler::{Equipmintslots, Item, Raritys};


#[derive(Clone)]
pub(crate) struct EquipItem{
    //Standart item attributs
    name: String,
    des: String,

    //Stats of the item
    equip_slot: Equipmintslots,
    armor_buf: u8,
    dmg_buf: u8,
    rarity: Raritys,

    value: u8

}

impl EquipItem {

    pub fn new(name: String, des: String, slot: Equipmintslots, armor_buf: u8, rarity: Raritys, value: u8) -> Self {
        Self {
            name,
            des,
            equip_slot: slot,
            armor_buf,
            rarity,
            value,
            dmg_buf: 0
        }
    }
    pub fn get_armor_buff(&self) -> &u8{
        &self.armor_buf
    }
    
    
}

impl Item for EquipItem {
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
    fn get_bonus_dmg(&self) -> &u8{
        &self.dmg_buf
    }
}