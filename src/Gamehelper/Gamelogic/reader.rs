use crate::gamelogic::konst;
use crate::gameobjects::dungeon::Dungeonroom;
use crate::gameobjects::encounter::EncounterTypes;
use crate::gameobjects::equip_item::EquipItem;
use crate::gameobjects::item_handler::{Equipmintslots, Item, ItemsTypes, Raritys};
use crate::gameobjects::monster::Monster;
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
use crate::gameobjects::trap::Trap;

pub fn read_json_file() {}
pub fn check_file(path: &str) -> anyhow::Result<&str, String> {
    match File::open(path) {
        Ok(mut file) => {
            println!("File found :)");

            let json: serde_json::Value =
                serde_json::from_reader(file).expect("file should be proper JSON");

            let monsters = read_encounter_category("monsters".parse().unwrap()).unwrap().len();
            let traps = read_encounter_category("traps".parse().unwrap()).unwrap().len();
            let mut armorpieces = 0;

            for (key, v) in read_category_item("armor".to_string()).unwrap() {
                for (keys, values) in v.as_object().unwrap() {
                    armorpieces += 1;
                }
            }

            let treasures = read_category_item("treasures".into()).unwrap().len();
            let consumables = read_category_item("consumables".into()).unwrap().len();
            let weapons =  read_category_item("weapons".into()).unwrap().len();

            let random_rooms = json.get("random_rooms").unwrap().as_object().unwrap().len();


            println!("{}", konst::JSONINFO(monsters,traps,armorpieces,treasures,consumables,weapons,random_rooms));

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

    let armor_pieces: Vec<_> = armor_category.keys().into_iter().map(|key| key).collect();

    let random_armor_piece = rand::rng().random_range(0..armor_category.len());

    let armor_piece = armor_category
        .get(armor_pieces[random_armor_piece])
        .unwrap()
        .as_object()
        .unwrap();

    ItemsTypes::EquipItem(EquipItem::new(
        armor_pieces[random_armor_piece].to_owned(),
        armor_pieces[random_armor_piece].to_owned(),
        Equipmintslots::from_string(
            armor_piece
                .get("slot")
                .unwrap()
                .as_str()
                .unwrap()
                .parse()
                .unwrap(),
        ),
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
        random_weapon
            .get("des")
            .unwrap()
            .as_str()
            .unwrap()
            .parse()
            .unwrap(),
        Raritys::COMMON,
        random_weapon.get("dmg").unwrap().as_u64().unwrap() as u8,
        0,
    ))
}

pub fn generate_trap(trapdata : &Map<String, Value>) -> EncounterTypes {

    let traps: Vec<_> = trapdata.keys().map(|key| key).collect();

    let rand = rand::rng().random_range(0..(traps.len()));

    let randomtrap = trapdata.get(traps[rand]).unwrap();

    EncounterTypes::Trap(Trap::new(
        traps[rand].to_owned(),
        randomtrap.get("des").unwrap().as_str().unwrap().to_string(),
        randomtrap.get("dmg").unwrap().as_u64().unwrap() as u8,

    ))
}

pub fn generate_monster(monsterdata: &Map<String, Value>) -> EncounterTypes {
    let mut keys: Vec<_> = monsterdata.keys().into_iter().map(|key| key).collect();
    let rand = rand::rng().random_range(0..keys.len());

    let randommonster = monsterdata.get(keys[rand]).unwrap().as_object().unwrap();

    EncounterTypes::Monster(Monster::from_json(
        keys[rand].to_owned(),
        randommonster.get("hp").unwrap().as_u64().unwrap() as u8,
        randommonster.get("dmg").unwrap().as_u64().unwrap() as u8,
        randommonster.get("description").unwrap().as_str().unwrap().to_string(),
    ))
}

pub fn read_category_item(key: String) -> io::Result<Map<String, Value>> {
    match File::open(konst::TEST_FIlE_PATH) {
        Ok(file) => {
            let json: serde_json::Value =
                serde_json::from_reader(file).expect("file should be proper JSON");

            Ok(json
                .get("items")
                .unwrap()
                .get(key)
                .unwrap()
                .as_object()
                .unwrap()
                .to_owned())
        }
        Err(e) => Err(e),
    }
}

pub fn read_encounter_category(key: String) -> io::Result<Map<String, Value>> {
    match File::open(konst::TEST_FIlE_PATH) {
        Ok(file) => {
            let json: serde_json::Value =
                serde_json::from_reader(file).expect("file should be proper JSON");

            Ok(json
                .get("encounter")
                .unwrap()
                .get(key)
                .unwrap()
                .as_object()
                .unwrap()
                .to_owned())
        }
        Err(e) => Err(e),
    }
}

pub fn generate_random_room() -> io::Result<Dungeonroom> {
    match File::open(konst::TEST_FIlE_PATH) {
        Ok(file) => {
            let json: serde_json::Value =
                serde_json::from_reader(file).expect("file should be proper JSON");

            let randomroomsdata = json.get("random_rooms").unwrap().as_object().unwrap();
            let keys: Vec<_> = randomroomsdata.keys().map(|roomname| roomname).collect();
            let rand = rand::rng().random_range(0..keys.len());
            let room = randomroomsdata
                .get(keys[rand])
                .unwrap()
                .as_object()
                .unwrap();

            Ok(Dungeonroom::fillerRoom(
                keys[rand].to_owned(),
                room.get("description")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string(),
                String::new(),
            ))
        }
        Err(e) => Err(e),
    }
}
