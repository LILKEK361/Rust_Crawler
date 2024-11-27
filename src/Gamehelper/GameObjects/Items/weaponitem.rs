use crate::item_handler::Item;

struct WeaponItem {

    //Common stats for item
    name: String,
    equip_slot: Vec<String>,
    des: String,

    //Specific WeaponItem stats:
    dmg_bonus: u8,
}

impl WeaponItem {
    fn get_bonus_dmg(&self){

    }
}

impl Item for WeaponItem {
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