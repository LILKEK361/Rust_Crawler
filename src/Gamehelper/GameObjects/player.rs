use std::collections::hash_map::IntoValues;
use std::sync::{Mutex, OnceLock};
use crate::{add_log, gameobjects};
use crate::gameobjects::inventoryslot;
use crate::gameobjects::inventoryslot::Inventoryslot;
use crate::gameobjects::item_handler::{Item, ItemsTypes};



pub(crate) struct Player {
    pub name: String,
    inventory: Vec<ItemsTypes>,
    inventory_size: u8,
    health: u8,
    attack: i8,

    level: i8,
    pub alive: bool,
    max_hp: i8,
    in_inventory: bool,
    armor: i8,
    skillmod: i8,
    skills: Vec<String> //todo
}



impl Player{

    pub fn new(name: String) -> Self{



        Self {
            name,
            inventory: vec![
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
            skillmod: 0,
            inventory_size: 10,
            level: 0,
            max_hp: 100,
            in_inventory: false,
            armor: 5,
            skills: vec!["Todo".into()]

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

    pub fn defend(&mut self, dmg: i8){
        if(dmg - self.armor > 0){
            self.health = self.health - ((dmg - self.armor) as u8)
        }

        if(self.health <= 0){
            self.alive = false;
        }
    }

    pub fn get_skill(&self) -> &i8{
        &self.skillmod
    }
    
    //Loot to inventory
    pub fn add_loot(&mut self, item: ItemsTypes ) -> bool{
        let mut added = false;
        for slot in &mut self.inventory  {

            if(slot.get_name().to_ascii_lowercase().eq("empty")){
                add_log(&*format!("{} was added",item.get_name()));
                *slot = item;
                added = true;
                break
            }
        }

        added
    }

    pub fn is_in_inventory(&self) -> &bool{
        &self.in_inventory
    }

    pub fn set_inventory(&mut self, yes: bool) {
        self.in_inventory = yes;
    }
    pub fn get_inventory(&self) -> &[ItemsTypes]{
        &self.inventory
    }


    pub fn get_player(&self) -> &Player{
        &self
    }

    pub fn get_stats(&self) -> (&str,u8,i8,i8,i8,i8, &Vec<String>) {
        (&self.name, self.health, self.max_hp, self.inventory.len() as i8, self.armor, self.level, &self.skills  )
    }

    
    pub fn player_ref() -> &'static Mutex<Player>{
        static PLAYER: OnceLock<Mutex<Player>> = OnceLock::new();
        
        PLAYER.get_or_init(||{
            let player = Mutex::new(Player::new("Playerholder".to_string()));
            player
        })

    }
    
    
    
    
    
    
}