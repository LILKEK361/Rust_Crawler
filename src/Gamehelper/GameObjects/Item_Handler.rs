
enum ItemTypes{

    Weapontype,
    Equiptype,
    Consumable,
    InventorySlot,

}
#[derive(Clone)]
pub(crate) enum Equipmintslots {
    Head,
    Torso,
    Hands,
    Weapeon,
    Pants,
    Shoes,
    None,
}


pub(crate) enum Items {

    EquipItem(crate::gameobjects::equip_item::EquipItem),
    WeaponItem(crate::gameobjects::weaponitem::WeaponItem),
    InventorySlot(crate::gameobjects::inventoryslot::Inventoryslot),

}

impl Item for Items {
    fn get_name(&self) -> &str {
        match self {
            Items::EquipItem(item) => item.get_name(),
            Items::WeaponItem(item) => item.get_name(),
            Items::InventorySlot(item) => item.get_name()
        }
    }

    fn get_equipment_slot(&self) -> Vec<Equipmintslots> {
        match self {
            Items::EquipItem(item) => item.get_equipment_slot(),
            Items::InventorySlot(item) => item.get_equipment_slot(),
            Items::WeaponItem(item) => item.get_equipment_slot()
        }
    }

    fn get_des(&self) -> &str {
        match self {
            Items::EquipItem(item) => item.get_des(),
            Items::InventorySlot(item) => item.get_des(),
            Items::WeaponItem(item) => item.get_des()
        }
    }
}

pub trait Item: Sync + Send {
    fn get_name(&self) -> &str;
    fn get_equipment_slot(&self) -> Vec<crate::gameobjects::item_handler::Equipmintslots>;
    fn get_des(&self) -> &str;




}





