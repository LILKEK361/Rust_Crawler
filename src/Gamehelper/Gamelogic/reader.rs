use crate::gamelogic::konst;
use crate::gameobjects::dungeon::Dungeonroom;
use crate::gameobjects::encounter::EncounterTypes;
use crate::gameobjects::equip_item::EquipItem;
use crate::gameobjects::item_handler::{Equipmintslots, Item, ItemsTypes, Raritys};
use crate::gameobjects::weaponitem::WeaponItem;
use colored::Colorize;
use log::trace;
use rand::Rng;
use serde_json::map::Values;
use serde_json::{json, Map, Value};
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{Error, ErrorKind, Read};
use terminal_link::Link;
use crate::gameobjects::monster::Monster;

pub fn read_json_file() {}
pub fn check_file(path: &str) -> anyhow::Result<&str, String> {
    match File::open(path) {
        Ok(mut file) => {
            println!("File found :)");

            let json: serde_json::Value =
                serde_json::from_reader(file).expect("file should be proper JSON");

            let monsters = match_json(json.get("monsters"));

            let traps = match_json(json.get("traps"));

            let items = match_json(json.get("items"));

            let weapon = match_json(json.get("items").unwrap().get("weapons"));

            let armorpieces = match_json(json.get("items").unwrap().get("armor"));

            let randomrooms = match_json(json.get("random_rooms"));

            let treasures = match_json(json.get("items").unwrap().get("treasures"));

            let consumables = match_json(json.get("items").unwrap().get("consumables"));

            let mut amor_count = 0;

            for (k, v) in json
                .get("items")
                .unwrap()
                .get("armor")
                .unwrap()
                .as_object()
                .unwrap()
            {
                for (k2, v2) in v.as_object().unwrap() {
                    amor_count += 1;
                }
            }

            println!(
                "{}",
                konst::JSONINFO(
                    monsters,
                    traps,
                    armorpieces,
                    treasures,
                    consumables,
                    weapon,
                    randomrooms
                )
            );
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

pub fn match_json(value: Option<&Value>) -> usize {
    match value {
        Some(v) => match v.as_object() {
            Some(va) => va.len(),
            _ => 0,
        },
        _ => 0,
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

pub fn generate_weapon(weapondata: &Map<String, Value>) -> ItemsTypes {
    let mut keys: Vec<_> = weapondata.keys().into_iter().map(|key| key).collect();

    let rand = rand::rng().random_range(0..keys.len());

    let random_weapon = weapondata.get(keys[rand]).unwrap().as_object().unwrap();

    ItemsTypes::WeaponItem(WeaponItem::new(
        keys[rand].to_owned(),
        random_weapon.get("des").unwrap().to_string(),
        Raritys::COMMON,
        random_weapon.get("dmg").unwrap().as_u64().unwrap() as u8,
        0,
    ))
}

pub fn generate_monster(monsterdata: &Map<String, Value>) -> EncounterTypes {

    let mut keys: Vec<_> = monsterdata.keys().into_iter().map(|key| key).collect();
    let rand = rand::rng().random_range(0..keys.len());

    let randommonster = monsterdata.get(keys[rand]).unwrap().as_object().unwrap();

    EncounterTypes::Monster(Monster::from_json(
        keys[rand].to_owned(),
        randommonster.get("hp").unwrap().as_u64().unwrap() as u8 ,
        randommonster.get("dmg").unwrap().as_u64().unwrap() as u8 ,
        randommonster.get("des").unwrap().to_string(),
    ))



}

pub fn read_with_item_category(key: String) -> io::Result<Map<String, Value>> {
    match File::open(konst::TEST_FIlE_PATH) {
        Ok(file) => {
            let json: serde_json::Value =
                serde_json::from_reader(file).expect("file should be proper JSON");;

            return Ok(json.get("items").unwrap().get(key).unwrap().as_object().unwrap().to_owned())

        }
        Err(e) => {
            Err(e)
        }
    }


}


/*
pub fn generate_room() -> Dungeonroom {}

*/
