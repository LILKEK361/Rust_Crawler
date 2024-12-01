
enum ItemTypes{

    Weapontype,
    Equiptype,
    Consumable,
    InventorySlot,

}

pub(crate) enum Equipmintslots {
    Head,
    Torso,
    Hands,
    Weapeon,
    Pants,
    Shoes,
    None,
}


pub trait Item{
    fn get_name(&self) -> &str;
    fn get_equipment_slot(&self) -> &[Equipmintslots];
    fn get_des(&self) -> &str;




}




