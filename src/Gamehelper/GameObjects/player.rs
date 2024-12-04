use std::collections::hash_map::IntoValues;
use std::sync::{Mutex, OnceLock};
use crate::gameobjects::inventoryslot;
use crate::gameobjects::inventoryslot::Inventoryslot;
use crate::gameobjects::item_handler::Item;


pub(crate) struct Player {
    name: String,
    inventory: [Box<dyn crate::gameobjects::item_handler::Item>;10],
    health: i32,
    attack: i32,
    defense: i32,
    level: i32,
    pub(crate) alive: bool,
}



impl Player{

    pub fn new(name: String) -> Self{



        Self {
            name,
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
    
    pub fn player_ref() -> &'static Mutex<Player>{
        static PLAYER: OnceLock<Mutex<Player>> = OnceLock::new();
        
        PLAYER.get_or_init(||{
            let player = Mutex::new(Player::new("Playerholder".to_string()));
            player
        })
    }
    
    
    
    
    
    
}