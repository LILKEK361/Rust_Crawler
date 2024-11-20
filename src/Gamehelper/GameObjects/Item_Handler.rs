
enum ItemTypes{

    Weapontype,
    Equiptype,
    Consumable,

}


pub trait Item{
    fn get_name(&self) -> &String;
    fn get_equipment_slot(&self) -> &Vec<String>;
    fn get_des(&self) -> &String;

}




