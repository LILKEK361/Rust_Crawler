use rand::Rng;
use crate::db_manager_ref;
use crate::gamehelper::dbpaths;
use crate::gameobjects::consumable_item::Consumable;
use crate::gameobjects::equip_item::EquipItem;
use crate::gameobjects::inventoryslot::Inventoryslot;
use crate::gameobjects::item_handler::{Equipmintslots, ItemsTypes, Raritys};
use crate::gameobjects::treasure_item::TreasureItem;
use crate::gameobjects::weaponitem::WeaponItem;

//This function returns a random weapon without specified attributes
pub fn generate_random_weapon() -> ItemsTypes{
    let weapons = db_manager_ref().lock().unwrap().search("Select * from weapons;");

    let random_number = rand::rng().random_range(0..=weapons.len() - 1);

    let weapon = weapons.get(random_number).unwrap();

    ItemsTypes::WeaponItem(WeaponItem::new(weapon.get::<_,String>(1),weapon.get::<_,String>(2),Raritys::from(weapon.get::<_,String>(3)),weapon.get::<_,i32>(4) as u8, weapon.get::<_, i32>(5) as u8))

}

pub fn generate_random_treaure() -> ItemsTypes{
    let treasures = db_manager_ref().lock().unwrap().search("Select * from treasures;");

    let random_number = rand::rng().random_range(0..=treasures.len() - 1);

    let treasure = treasures.get(random_number).unwrap();

    ItemsTypes::TreasureItem(TreasureItem::new(treasure.get::<_,String>(1), Equipmintslots::None, treasure.get::<_,String>(2), treasure.get::<_,String>(3), Raritys::from(treasure.get::<_,String>(4)), treasure.get::<_, i32>(5) as u8))

}

pub fn generate_random_equip() -> ItemsTypes{
    let equips = db_manager_ref().lock().unwrap().search("Select * from equip;");

    let random_number = rand::rng().random_range(0..=equips.len() - 1);

    let eq = equips.get(random_number).unwrap();

    ItemsTypes::EquipItem(EquipItem::new(eq.get::<_, String>(1), eq.get::<_, String>(2), Equipmintslots::from_string(eq.get::<_, String>(3)), eq.get::<_, i32>(5) as u8, Raritys::from(eq.get::<_, String>(4)), eq.get::<_, i32>(6) as u8))

}

pub fn generate_random_consumable() -> ItemsTypes {
    let consumables = db_manager_ref().lock().unwrap().search("Select * from consumables");

    let random_number = rand::rng().random_range(0..=consumables.len() - 1);

    let con = consumables.get(random_number).unwrap();

    ItemsTypes::ConsumableItem(Consumable::new(con.get::<_,String>(1),con.get::<_,String>(2), Raritys::from(con.get::<_,String>(3)), con.get::<_,i32>(4) as u8, con.get::<_,i32>(5) as u8 ,con.get::<_,i32>(6) as u8 ))
}

pub fn generate_random_drop() -> ItemsTypes {

    let mut loot: Vec<ItemsTypes> = vec![];

    for i in 0..3 {
        loot.push(generate_random_consumable());
        //loot.push(generate_random_equip());
        //loot.push(generate_random_weapon());
    }

    let random_number = rand::rng().random_range(0..=loot.len() - 1);

    loot.get(random_number).unwrap().to_owned()

}

