use crate::gamelogic::konst;
use crate::gameobjects::dungeon::Dungeonroom;
use crate::gameobjects::encounter::EncounterTypes;
use crate::gameobjects::equip_item::EquipItem;
use crate::gameobjects::item_handler::{Equipmintslots, Item, ItemsTypes, Raritys};
use colored::Colorize;
use log::trace;
use rand::Rng;
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{Error, Read};
use terminal_link::Link;

struct JSONFILESTRUCT {
    items: u8,
    monster: u8,
    traps: u8,
    //Head, Torso, pants, shoes,
    armorpieces: u8,
    treasures: u8,
    consumables: u8,
    weapons: u8,
    randomroom: u8,
}

impl JSONFILESTRUCT {
    pub fn new(
        monster: u8,
        trap: u8,
        armorpieces: u8,
        treasures: u8,
        consumables: u8,
        weapons: u8,
        randomrom: u8,
    ) -> Self {
        let mut all_armor: u8 = 0;

        Self {
            items: treasures + consumables + weapons + all_armor,
            monster,
            traps: trap,
            randomroom: randomrom,
            armorpieces,
            treasures,
            consumables,
            weapons,
        }
    }
}

pub fn read_json_file() {}
pub fn check_file(path: &str) -> anyhow::Result<&str, String> {
    match File::open(path) {
        Ok(mut file) => {
            println!("File found :)");

            let json: serde_json::Value =
                serde_json::from_reader(file).expect("file should be proper JSON");

            let monsters = json.get("monsters").unwrap().as_object().unwrap();

            let traps = json.get("traps").unwrap().as_object().unwrap();

            let items = json.get("items").unwrap().as_object().unwrap();

            let weapon = items.get("weapons").unwrap().as_object().unwrap();



            let armorpieces = items.get("armor").unwrap().as_object().unwrap();

            let randomrooms = json.get("random_rooms").unwrap().as_object().unwrap();

            let mut amor_count = 0;

            for (k, v) in armorpieces {
                for (k2, v2) in v.as_object().unwrap() {
                    amor_count += 1;
                }
            }

            let JSONFILESTRUCT = JSONFILESTRUCT::new(
                monsters.len() as u8,
                traps.iter().len() as u8,
                amor_count,
                0,
                0,
                weapon.len() as u8,
                randomrooms.len() as u8,
            );

            println!("{}", konst::JSONINFO(monsters.len() as u8,
                                           traps.iter().len() as u8,
                                           amor_count,
                                           0,
                                           0,
                                           weapon.len() as u8,
                                           randomrooms.len() as u8,));
            return Ok("loading complete");
        }
        Err(_) => {
            return Err(konst::JSONFILEERROR(Link::new(
                konst::GAMENAME,
                konst::GITHUBLINK,
            )))
        }
    }
}

pub fn generate_armor_piece(armordata: &Map<String, Value>) -> ItemsTypes {
    let mut keys: Vec<_> = armordata.keys().into_iter().map(|key| key).collect();

    let armor_category = armordata
        .get(keys[rand::rng().random_range(0..armordata.len())])
        .unwrap()
        .as_object()
        .unwrap();

    keys = armor_category.keys().into_iter().map(|key| key).collect();

    let random_armor_piece = rand::rng().random_range(0..armor_category.len());

    let armor_piece = armor_category
        .get(keys[random_armor_piece])
        .unwrap()
        .as_object()
        .unwrap();

    ItemsTypes::EquipItem(EquipItem::new(
        keys[random_armor_piece].to_owned(),
        keys[random_armor_piece].to_owned(),
        Equipmintslots::from_string(armor_piece.get("slot").unwrap().to_string()),
        armor_piece.get("def").unwrap().as_u64().unwrap() as u8,
        Raritys::COMMON,
        0,
    ))
}
/*
pub fn generate_weapon() -> ItemsTypes {}

pub fn generate_monster() -> EncounterTypes {}

pub fn generate_room() -> Dungeonroom {}

*/
