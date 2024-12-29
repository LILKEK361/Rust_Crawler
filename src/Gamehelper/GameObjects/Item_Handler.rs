use std::ops::Deref;
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
    pub fn from(s: String) ->Self {
        match s.to_lowercase().as_str() {
            "trash" => Raritys::TRASH,
            "common" => Raritys::COMMON,
            "rare" => Raritys::RARE,
            "godly" => Raritys::GODLY,
            "shizo" => Raritys::SHIZO,
            "broken" => Raritys::BROKEN,
            "demonic" => Raritys::DEMONIC,
             // Return None for invalid inputs
            _ => {Raritys::TRASH}
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

#[derive(Clone, Eq, PartialEq, Hash)]
pub(crate) enum Equipmintslots {
    Head,
    Torso,
    Hands,
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
            Equipmintslots::Pants => "Pants",
            Equipmintslots::Shoes => "Shoes",
            Equipmintslots::None => "None",
        }
    }

    pub fn from_string(input: String) -> Self {
        match input.to_ascii_lowercase().as_str() {
            "head" => Equipmintslots::Head,
            "torso" => Equipmintslots::Torso,
            "hands" => Equipmintslots::Hands,
            "pants" => Equipmintslots::Pants,
            "shoes" => Equipmintslots::Shoes,
            _ => Equipmintslots::None
        }
    }
}

#[derive(Clone)]
pub(crate) enum ItemsTypes {

    EquipItem(crate::gameobjects::equip_item::EquipItem),
    WeaponItem(crate::gameobjects::weaponitem::WeaponItem),
    InventorySlot(crate::gameobjects::inventoryslot::Inventoryslot),
    TreasureItem(crate::gameobjects::treasure_item::TreasureItem),
    ConsumableItem(crate::gameobjects::consumable_item::Consumable)

}





impl Item for ItemsTypes {


    fn get_name(&self) -> &str {
        match self {
            ItemsTypes::EquipItem(item) => item.get_name(),
            ItemsTypes::WeaponItem(item) => item.get_name(),
            ItemsTypes::InventorySlot(item) => item.get_name(),
            ItemsTypes::TreasureItem(item) => item.get_name(),
            ItemsTypes::ConsumableItem(item) => item.get_name(),
        }
    }

    fn get_equipment_slot(&self) -> &Equipmintslots {
        match self {
            ItemsTypes::EquipItem(item) => item.get_equipment_slot(),
            ItemsTypes::InventorySlot(item) => item.get_equipment_slot(),
            ItemsTypes::WeaponItem(item) => item.get_equipment_slot(),
            ItemsTypes::TreasureItem(item) => item.get_equipment_slot(),
            ItemsTypes::ConsumableItem(item) => item.get_equipment_slot(),


        }
    }

    fn get_des(&self) -> &str {
        match self {
            ItemsTypes::EquipItem(item) => item.get_des(),
            ItemsTypes::InventorySlot(item) => item.get_des(),
            ItemsTypes::WeaponItem(item) => item.get_des(),
            ItemsTypes::TreasureItem(item) => item.get_des(),
            ItemsTypes::ConsumableItem(item) => item.get_des(),


        }
    }
    
    fn get_rarity(&self) -> &Raritys {
        match self {
            ItemsTypes::EquipItem(item) => item.get_rarity(),
            ItemsTypes::InventorySlot(item) => item.get_rarity(),
            ItemsTypes::WeaponItem(item) => item.get_rarity(),
            ItemsTypes::TreasureItem(item) => item.get_rarity(),
            ItemsTypes::ConsumableItem(item) => item.get_rarity(),


        }
    }

     fn get_value(&self) -> &u8 {
        match self {
            ItemsTypes::EquipItem(item) => item.get_value(),
            ItemsTypes::InventorySlot(item) => item.get_value(),
            ItemsTypes::WeaponItem(item) => item.get_value(),
            ItemsTypes::TreasureItem(item) => item.get_value(),
            ItemsTypes::ConsumableItem(item) => item.get_value(),


        }
    }

    fn get_bonus_dmg(&self) -> &u8 {
        match self {
            ItemsTypes::EquipItem(item) => item.get_bonus_dmg(),
            ItemsTypes::InventorySlot(item) => &0u8,
            ItemsTypes::WeaponItem(item) => item.get_bonus_dmg(),
            ItemsTypes::TreasureItem(item) => item.get_bonus_dmg(),
            ItemsTypes::ConsumableItem(item) => item.get_bonus_dmg(),


        }
    }


}

pub trait Item: Sync + Send + Clone {
    fn get_name(&self) -> &str;
    fn get_equipment_slot(&self) -> &Equipmintslots;
    fn get_des(&self) -> &str;
    fn get_rarity(&self) -> &Raritys;
    fn get_value(&self) -> &u8;
    fn get_bonus_dmg(&self) -> &u8;


}





