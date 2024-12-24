
enum ItemTypes{

    Weapontype,
    Equiptype,
    Consumable,
    InventorySlot,

}

#[derive(Clone)]
pub enum Raritys {
    TRASH,
    COMMON,
    RARE,
    GODLY,
    SHIZO,
    BROKEN,
    DEMONIC,

    
}

impl Raritys {
    pub fn from(s: String) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "trash" => Some(Raritys::TRASH),
            "common" => Some(Raritys::COMMON),
            "rare" => Some(Raritys::RARE),
            "godly" => Some(Raritys::GODLY),
            "shizo" => Some(Raritys::SHIZO),
            "broken" => Some(Raritys::BROKEN),
            "demonic" => Some(Raritys::DEMONIC),
            _ => None, // Return None for invalid inputs
        }
    }


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

#[derive(Clone)]
pub(crate) enum ItemsTypes {

    EquipItem(crate::gameobjects::equip_item::EquipItem),
    WeaponItem(crate::gameobjects::weaponitem::WeaponItem),
    InventorySlot(crate::gameobjects::inventoryslot::Inventoryslot),

}



impl Item for ItemsTypes {
    fn get_name(&self) -> &str {
        match self {
            ItemsTypes::EquipItem(item) => item.get_name(),
            ItemsTypes::WeaponItem(item) => item.get_name(),
            ItemsTypes::InventorySlot(item) => item.get_name()
        }
    }

    fn get_equipment_slot(&self) -> Vec<Equipmintslots> {
        match self {
            ItemsTypes::EquipItem(item) => item.get_equipment_slot(),
            ItemsTypes::InventorySlot(item) => item.get_equipment_slot(),
            ItemsTypes::WeaponItem(item) => item.get_equipment_slot()
        }
    }

    fn get_des(&self) -> &str {
        match self {
            ItemsTypes::EquipItem(item) => item.get_des(),
            ItemsTypes::InventorySlot(item) => item.get_des(),
            ItemsTypes::WeaponItem(item) => item.get_des()
        }
    }
    
    fn get_rarity(&self) -> &Raritys {
        match self {
            ItemsTypes::EquipItem(item) => item.get_rarity(),
            ItemsTypes::InventorySlot(item) => item.get_rarity(),
            ItemsTypes::WeaponItem(item) => item.get_rarity(),
        }
    }
}

pub trait Item: Sync + Send + Clone {
    fn get_name(&self) -> &str;
    fn get_equipment_slot(&self) -> Vec<crate::gameobjects::item_handler::Equipmintslots>;
    fn get_des(&self) -> &str;
    fn get_rarity(&self) -> &Raritys;




}





