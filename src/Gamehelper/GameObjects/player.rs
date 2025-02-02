use crate::gamelogic::gamehelperfunctions::{generate_random_equip, generate_random_weapon};
use crate::gamelogic::terminaldrawer::{drawer, tdrawer};
use crate::gameobjects::consumable_item::Consumable;
use crate::gameobjects::inventoryslot;
use crate::gameobjects::inventoryslot::Inventoryslot;
use crate::gameobjects::item_handler::{Equipmintslots, Item, ItemsTypes, Raritys};
use crate::gameobjects::passiv_handler::PassivTypes;
use crate::{add_log, gameobjects, gamestate_ref, Gamestate};
use std::ascii::AsciiExt;
use std::collections::hash_map::IntoValues;
use std::collections::HashMap;
use std::mem::forget;
use std::sync::{Mutex, OnceLock};

pub(crate) struct Player {
    pub name: String,
    inventory: Vec<ItemsTypes>,
    inventory_size: u8,
    health: u8,
    attack: u8,
    equipmentslots: HashMap<Equipmintslots, ItemsTypes>,
    level: i8,
    pub alive: bool,
    max_hp: u8,
    in_inventory: bool,
    armor: i8,
    skillmod: i8,
    skills: Vec<String>, //todo
    inspecting: (bool, u8),
}

impl Player {
    pub fn new(name: String) -> Self {
        Self {
            name,
            inventory: vec![
                generate_random_equip(),
                generate_random_weapon(),
                ItemsTypes::ConsumableItem(Consumable::new(
                    "Healing Potion".into(),
                    "A healing Potion".to_string(),
                    Raritys::TRASH,
                    1,
                    20,
                    0,
                )),
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
            attack: 5,
            skillmod: 0,
            inventory_size: 10,
            level: 0,
            max_hp: 100,
            in_inventory: false,
            armor: 5,
            skills: vec!["Todo".into()],
            equipmentslots: HashMap::from([
                (
                    Equipmintslots::Head,
                    ItemsTypes::InventorySlot(Inventoryslot::empty()),
                ),
                (
                    Equipmintslots::Torso,
                    ItemsTypes::InventorySlot(Inventoryslot::empty()),
                ),
                (
                    Equipmintslots::Hands,
                    ItemsTypes::InventorySlot(Inventoryslot::empty()),
                ),
                (
                    Equipmintslots::Pants,
                    ItemsTypes::InventorySlot(Inventoryslot::empty()),
                ),
                (
                    Equipmintslots::Shoes,
                    ItemsTypes::InventorySlot(Inventoryslot::empty()),
                ),
            ]),
            inspecting: (false, 0),
        }
    }

    //Function for the inventory display
    fn display_inventory(&self) {
        todo!()
    }

    //Functions for combat of the player
    pub fn attack(&self) -> u8 {
        //todo: Check for equipment
        self.attack
    }

    pub fn take_dmg(&mut self, dmg: i8) {
        let taken_dmg = (dmg as u8 - (self.armor / 2) as u8);
        add_log(&*format!("You took {} dmg,", taken_dmg));

        if (taken_dmg > self.health) {
            self.health = 0;
            self.alive = false;
            Self::player_died()
        } else {
            self.health = self.health - taken_dmg;
        }
    }

    pub fn take_true_dmg(&mut self, dmg: i8) {
        add_log(&*format!("You took {} dmg,", dmg as u8));

        if (dmg as u8 >= self.health) {
            self.health = 0;
            self.alive = false;
            Self::player_died()
        } else {
            self.health = self.health - dmg as u8;
        }
    }

    pub fn get_hp(&self) -> &u8 {
        &self.health
    }

    pub fn check_equipment_bonus_dmg(&self) -> u8 {
        let mut dmg_bonus = 0;
        for (k, v) in &self.equipmentslots {
            if (!v.get_name().eq("empty")) {
                dmg_bonus = dmg_bonus + v.get_bonus_dmg()
            }
        }
        dmg_bonus
    }

    pub fn get_max_hp(&self) -> &u8 {
        &self.max_hp
    }

    pub fn defend(&mut self, dmg: i8) {
        if ((dmg - (self.armor * 2)) as u8 > self.health) {
            self.alive = false;
            Self::player_died()
        }

        if (dmg - (self.armor * 2) > 0) {
            self.health = self.health - ((dmg - (self.armor * 2)) as u8)
        }
    }

    pub fn get_skill(&self) -> &i8 {
        &self.skillmod
    }

    //Loot to inventory
    pub fn add_loot(&mut self, item: ItemsTypes) -> bool {
        let mut added = false;
        for slot in &mut self.inventory {
            if (slot.get_name().to_ascii_lowercase().eq("empty")) {
                add_log(&*format!("{} was added", item.get_name()));
                *slot = item;
                added = true;
                break;
            }
        }

        added
    }

    pub fn inspect(&mut self, slot: u8) {
        if (slot <= self.inventory_size - 1) {
            self.inspecting = (true, slot)
        }
    }
    pub fn stop_inspect(&mut self) {
        self.inspecting = (false, 0)
    }
    pub fn get_inspect(&self) -> &(bool, u8) {
        &self.inspecting
    }
    pub fn drop_item_from_inventory(&mut self, index: usize) {
        if (index <= self.inventory.len() - 1
            && !self
                .inventory
                .get(index)
                .unwrap()
                .get_name()
                .to_ascii_lowercase()
                .eq("empty"))
        {
            self.inventory[index] = ItemsTypes::InventorySlot(Inventoryslot::empty());
        } else {
            add_log("Dungeon: You are a funny one aren't you?")
        }
    }

    pub fn is_in_inventory(&self) -> &bool {
        &self.in_inventory
    }

    pub fn set_inventory(&mut self, yes: bool) {
        self.in_inventory = yes;
    }
    pub fn get_inventory(&self) -> &[ItemsTypes] {
        &self.inventory
    }

    pub fn get_player(&self) -> &Player {
        &self
    }

    pub fn get_stats(&self) -> (&str, u8, u8, i8, i8, i8, &Vec<String>) {
        (
            &self.name,
            self.health,
            self.max_hp,
            self.inventory.len() as i8,
            self.armor,
            self.level,
            &self.skills,
        )
    }

    pub fn use_item(&mut self, item_slot: u8) {
        if (item_slot <= &self.inventory_size - 1) {
            match self.inventory.get_mut(item_slot as usize).unwrap() {
                ItemsTypes::ConsumableItem(item) => {
                    if (item.get_name().to_ascii_lowercase().contains("heal")) {
                        let healt_before = self.health;

                        if ((self.health + item.get_buf()) > self.max_hp) {
                            add_log(&*format!("Dungeon: Healed for {} HP", item.get_buf()));
                            self.health = self.max_hp;
                            add_log(&*format!(
                                "Dungeon: {healt_before} HP -> {} HP",
                                self.health
                            ));
                        } else {
                            add_log(&*format!("Dungeon: Healed for {} HP", item.get_buf()));
                            self.health = self.health + item.get_buf();
                            add_log(&*format!(
                                "Dungeon: {healt_before} HP -> {} HP",
                                self.health
                            ));
                        }

                        item.used();

                        if (*item.get_uses() == 0) {
                            self.inventory[item_slot as usize] =
                                ItemsTypes::InventorySlot(Inventoryslot::empty())
                        }
                    }
                }
                _ => add_log("Dungeon: cant use this item"),
            }
        } else {
            add_log("Dungeon: Pls use something that");
            add_log("actually exists")
        }
    }

    pub fn player_ref() -> &'static Mutex<Player> {
        static PLAYER: OnceLock<Mutex<Player>> = OnceLock::new();

        PLAYER.get_or_init(|| {
            let player = Mutex::new(Player::new("Playerholder".to_string()));
            player
        })
    }

    pub fn create_new_player() {
        let mut player = Self::player_ref()
            .lock()
            .expect("That shoudn't happend. Pls create a ticket on Github");
        *player = Self::new("Olaf".into());
    }

    pub fn equip_item(&mut self, item_index: usize, slot: Equipmintslots) {
        if (item_index <= (self.inventory_size - 1) as usize) {
            if (self
                .equipmentslots
                .get(&slot)
                .unwrap()
                .get_name()
                .to_ascii_lowercase()
                .eq("empty")
                && slot == *self.inventory.get(item_index).unwrap().get_equipment_slot()
                && slot != Equipmintslots::None)
            {
                match &self.inventory.get(item_index).unwrap() {
                    ItemsTypes::EquipItem(eq) => {
                        let amrmor_bevor = self.armor;
                        self.armor = self.armor + *eq.get_armor_buff() as i8;
                        add_log(&*format!(
                            "Dungeon: {} AD -> {} AD",
                            amrmor_bevor, self.armor
                        ));

                        self.equipmentslots
                            .insert(slot, self.inventory.get(item_index).unwrap().to_owned());
                    }
                    ItemsTypes::WeaponItem(weapeon) => {
                        let amrmor_bevor = self.attack;
                        self.attack = self.attack + *weapeon.get_bonus_dmg();
                        add_log(&*format!(
                            "Dungeon: {} DMG -> {} DMG",
                            amrmor_bevor, self.attack
                        ));

                        self.equipmentslots
                            .insert(slot, self.inventory.get(item_index).unwrap().to_owned());
                    }
                    _ => {
                        self.equipmentslots
                            .insert(slot, self.inventory.get(item_index).unwrap().to_owned());
                    }
                }

                self.inventory[item_index] = ItemsTypes::InventorySlot(Inventoryslot::empty())
            } else if (!self
                .equipmentslots
                .get(&slot)
                .unwrap()
                .get_name()
                .to_ascii_lowercase()
                .eq("empty"))
            {
                add_log("You already have something equipt ");
                add_log("on that slot");
            } else {
                add_log("You fool :)")
            }
        }
    }

    pub fn has_free_slot(&self) -> bool {
        let mut si = false;
        for slot in &self.inventory {
            if (slot.get_name().to_ascii_lowercase().eq("empty")) {
                si = true
            }
        }
        si
    }

    pub fn get_equipment_from_slot(&self, slot: String) -> &ItemsTypes {
        &self
            .equipmentslots
            .get(&Equipmintslots::from_string(slot))
            .unwrap()
    }

    pub fn unequip(&mut self, slot: Equipmintslots) {
        if (slot != Equipmintslots::None
            && !self
                .equipmentslots
                .get(&slot)
                .unwrap()
                .get_name()
                .to_ascii_lowercase()
                .eq("empty"))
        {
            if (self.has_free_slot()) {
                match self.equipmentslots.get(&slot).unwrap() {
                    ItemsTypes::EquipItem(eq) => {
                        let amrmor_bevor = self.armor;
                        self.armor = self.armor - *eq.get_armor_buff() as i8;
                        add_log(&*format!("Dungeon: {} -> {}", amrmor_bevor, self.armor));
                        self.add_loot(ItemsTypes::EquipItem(eq.to_owned()));
                        self.equipmentslots
                            .insert(slot, ItemsTypes::InventorySlot(Inventoryslot::empty()));
                    }

                    (item) => {
                        self.add_loot(item.to_owned());
                        self.equipmentslots
                            .insert(slot, ItemsTypes::InventorySlot(Inventoryslot::empty()));
                    }

                    _ => {}
                }
            } else {
                add_log("Dungeon: No space in inventory")
            }
        } else {
            add_log("Dungeon: Pls dont be weird")
        }
    }

    pub fn player_died() {
        tdrawer::set_render_queue("death".into());
        //*gamestate_ref().lock().unwrap() = Gamestate::home;
    }
}
