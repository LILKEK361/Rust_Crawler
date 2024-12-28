use crate::gamelogic;
use crate::gamelogic::room_descriptions;
use crate::gameobjects::encounter::Encounter;
use crate::gameobjects::item_handler::{Item, ItemsTypes};
use crate::gameobjects::treasure_item::TreasureItem;

#[derive(Clone)]
pub struct Treasure {
    name: String,
    des: String,
    treasure: ItemsTypes,
    t_type: String,
}


impl Treasure {
    pub fn new() -> Self {
        Self {
            name: String::from(room_descriptions::TREASURETITLE),
            des: String::from(room_descriptions::TREASUREDES),
            treasure: gamelogic::gamehelperfunctions::generate_random_treaure(),
            t_type: String::from("Chest") ,
        }
    }

    pub fn take(&mut self) -> ItemsTypes{
        self.treasure.to_owned()
    }
 }


impl Encounter for Treasure {
    fn get_Name(&self) -> &str {
        &self.name
    }

    fn get_Type(&self) -> &str {
        &self.t_type
    }

    fn get_description(&self) -> &str {
        &self.des
    }
}