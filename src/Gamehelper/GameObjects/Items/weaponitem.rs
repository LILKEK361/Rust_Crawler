use crate::item_handler::Item;

struct Weapon_item{
    name: String,
    dmg_bonus: u8,
    equip_slot: Vec<String>,
    des: String,
}

impl Weapon_item {
    fn get_bonus_dmg(&self){

    }
}

impl Item for Weapon_item{
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