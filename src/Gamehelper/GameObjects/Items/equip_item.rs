use crate::item_handler::Item;

struct EquipItem{
    //Standart item attributs
    name: String,
    des: String,

    //Stats of the item
    equip_slot: Vec<String>,
    armor_buff: i8,
    dmg_buff: i8,

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
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_equipment_slot(&self) -> &Vec<String> {
        &self.equip_slot
    }

    fn get_des(&self) -> &String {
        &self.des
    }
}