use crate::gameobjects::item_handler::Raritys::{BROKEN, DEMONIC, GODLY, RARE};

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

    pub fn to_string(&self) -> &str {
        match &self {
            Raritys::TRASH => "trash",
            Raritys::COMMON => "common",
            RARE => "rare",
            GODLY => "godly",
            BROKEN => "broken",
            DEMONIC => "demonic",
            _ => "Mh bug"
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

impl Equipmintslots {
    pub fn to_string(&self) -> & str {
        match self {
            Equipmintslots::Head => "Head",
            Equipmintslots::Torso => "Torso",
            Equipmintslots::Hands => "Hands",
            Equipmintslots::Weapeon => "Weapon",
            Equipmintslots::Pants => "Pants",
            Equipmintslots::Shoes => "Shoes",
            Equipmintslots::None => "None",
        }
    }
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

    fn get_equipment_slot(&self) -> &Equipmintslots {
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
    fn get_equipment_slot(&self) -> &Equipmintslots;
    fn get_des(&self) -> &str;
    fn get_rarity(&self) -> &Raritys;




}





