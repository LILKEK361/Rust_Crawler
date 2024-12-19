use crate::gameobjects::item_handler::{Equipmintslots, Item, Raritys};


#[derive(Clone)]
pub(crate) struct EquipItem{
    //Standart item attributs
    name: String,
    des: String,

    //Stats of the item
    equip_slot: Vec<Equipmintslots>,
    armor_buff: i8,
    dmg_buff: i8,
    rarity: Raritys

}

impl EquipItem {
    fn get_armor_buff(&self) -> &i8{
        &self.armor_buff
    }
    fn get_dmg_buff(&self) -> &i8{
        &self.dmg_buff
    }
    
}

impl Item for EquipItem {
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