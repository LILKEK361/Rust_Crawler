use rand::Rng;
use crate::db_manager_ref;
use crate::gamehelper::dbpaths;
use crate::gameobjects::item_handler::{ItemsTypes, Raritys};
use crate::gameobjects::weaponitem::WeaponItem;

//This function returns a random weapon without specified attributes
pub fn generat_random_weapon() -> ItemsTypes{
    let weapons = db_manager_ref().lock().unwrap().search("Select * from weapons;");

    let random_number = rand::rng().random_range(0..=weapons.len() - 1);

    let weapon = weapons.get(random_number).unwrap();

    ItemsTypes::WeaponItem(WeaponItem::new(weapon.get::<_,String>(1),weapon.get::<_,String>(2),Raritys::from(weapon.get::<_,String>(3)).unwrap(),weapon.get::<_,i32>(4) as u8))


}