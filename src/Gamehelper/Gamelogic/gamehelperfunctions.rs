use rand::Rng;
use crate::db_manager_ref;
use crate::gamehelper::dbpaths;
use crate::gameobjects::inventoryslot::Inventoryslot;
use crate::gameobjects::item_handler::{Equipmintslots, ItemsTypes, Raritys};
use crate::gameobjects::treasure_item::TreasureItem;
use crate::gameobjects::weaponitem::WeaponItem;

//This function returns a random weapon without specified attributes
pub fn generat_random_weapon() -> ItemsTypes{
    let weapons = db_manager_ref().lock().unwrap().search("Select * from weapons;");

    let random_number = rand::rng().random_range(0..=weapons.len() - 1);

    let weapon = weapons.get(random_number).unwrap();

    ItemsTypes::WeaponItem(WeaponItem::new(weapon.get::<_,String>(1),weapon.get::<_,String>(2),Raritys::from(weapon.get::<_,String>(3)),weapon.get::<_,i32>(4) as u8))

}

pub fn generate_random_treaure() -> ItemsTypes{
    let treasures = db_manager_ref().lock().unwrap().search("Select * from treasures;");

    let random_number = rand::rng().random_range(0..=treasures.len() - 1);

    let treasure = treasures.get(random_number).unwrap();

    ItemsTypes::TreasureItem(TreasureItem::new(treasure.get::<_,String>(1), Equipmintslots::None, treasure.get::<_,String>(2), treasure.get::<_,String>(3), Raritys::from(treasure.get::<_,String>(4))))

}

