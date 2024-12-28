use crate::gameobjects::item_handler::{Equipmintslots, Item, Raritys};


#[derive(Clone)]
pub(crate) struct EquipItem{
    //Standart item attributs
    name: String,
    des: String,

    //Stats of the item
    equip_slot: Equipmintslots,
    armor_buff: i8,
    dmg_buff: u8,
    rarity: Raritys,

    value: i8

}

impl EquipItem {

    fn get_armor_buff(&self) -> &i8{
        &self.armor_buff
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

    fn get_value(&self) -> &i8 {
        &self.value
    }
    fn get_bonus_dmg(&self) -> &u8{
        &self.dmg_buff
    }
}