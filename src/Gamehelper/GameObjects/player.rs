use std::collections::hash_map::IntoValues;
use std::sync::{Mutex, OnceLock};
use crate::add_log;
use crate::gameobjects::inventoryslot;
use crate::gameobjects::inventoryslot::Inventoryslot;
use crate::gameobjects::item_handler::{Item, ItemsTypes};


pub(crate) struct Player {
    pub name: String,
    inventory: [ItemsTypes;10],
    health: u8,
    attack: i8,
    defense: i8,
    level: i8,
    pub alive: bool,
    max_hp: i8
}



impl Player{

    pub fn new(name: String) -> Self{



        Self {
            name,
            inventory: [
                ItemsTypes::InventorySlot(Inventoryslot::empty()),
                ItemsTypes::InventorySlot(Inventoryslot::empty()),
                ItemsTypes::InventorySlot(Inventoryslot::empty()),
                ItemsTypes::InventorySlot(Inventoryslot::empty()),
                ItemsTypes::InventorySlot(Inventoryslot::empty()),
                ItemsTypes::InventorySlot(Inventoryslot::empty()),
                ItemsTypes::InventorySlot(Inventoryslot::empty()),
                ItemsTypes::InventorySlot(Inventoryslot::empty()),
                ItemsTypes::InventorySlot(Inventoryslot::empty()),
                ItemsTypes::InventorySlot(Inventoryslot::empty()),
            ],
            health: 100,
            alive: true,
            attack: 50,
            defense: 1,
            level: 0,
            max_hp: 100

        }
    }

    //Function for the inventory display
    fn display_inventory(&self){todo!()}

    //Functions for combat of the player
    pub fn attack(&self) -> &i8{
        //todo: Check for equipment
        &self.attack
    }
    
    pub fn take_dmg(&mut self, dmg: i8){
        self.health = self.health - dmg as u8;
        if(self.health <= 0){
            self.alive = false;
        }
    }

    pub fn get_hp(&self) -> &u8 {
        &self.health
    }

    pub fn get_max_hp(&self) -> &i8{
        &self.max_hp
    }
    
    
    //Loot to inventory
    fn add_loot(&mut self, inventory_slot: i8, item: ItemsTypes){
        if(inventory_slot <= self.inventory.len() as i8){

            self.inventory[inventory_slot as usize] = item;

        }else {
            add_log("Inventory Slot doesn't exist")
        }
    }

    pub fn get_inventory(&self) -> &[ItemsTypes]{
        &self.inventory
    }


    pub fn get_player(&self) -> &Player{
        &self
    }


    
    pub fn player_ref() -> &'static Mutex<Player>{
        static PLAYER: OnceLock<Mutex<Player>> = OnceLock::new();
        
        PLAYER.get_or_init(||{
            let player = Mutex::new(Player::new("Playerholder".to_string()));
            player
        })

    }
    
    
    
    
    
    
}