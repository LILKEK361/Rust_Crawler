use std::collections::hash_map::IntoValues;
use crate::gameobjects::inventoryslot;
use crate::gameobjects::inventoryslot::Inventoryslot;
use crate::gameobjects::item_handler::Item;

pub(crate) struct Player<'a> {
    name: &'a str,
    inventory: [Box<dyn crate::gameobjects::item_handler::Item>;10],
    health: i32,
    attack: i32,
    defense: i32,
    level: i32,
    pub(crate) alive: bool,
}



impl Player<'static>{

    pub fn new() -> Self{



        Self {
            name: &"Common player",
            inventory: [
                Box::new(Inventoryslot::empty()),
                Box::new(Inventoryslot::empty()),
                Box::new(Inventoryslot::empty()),
                Box::new(Inventoryslot::empty()),
                Box::new(Inventoryslot::empty()),
                Box::new(Inventoryslot::empty()),
                Box::new(Inventoryslot::empty()),
                Box::new(Inventoryslot::empty()),
                Box::new(Inventoryslot::empty()),
                Box::new(Inventoryslot::empty()),
            ],
            health: 100,
            alive: true,
            attack: 10,
            defense: 1,
            level: 0,

        }
    }

    //Function for the inventory display
    fn display_inventory(&self){todo!()}

    //Functions for combat of the player
    fn attack(&self){
        //Check for equipment
        todo!()
    }
    
    fn take_dmg(&self){todo!()}
    
    
    //Loot to inventory
    fn add_loot(&self){todo!()}
    
    
    
    
}