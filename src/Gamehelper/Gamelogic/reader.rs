use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use colored::Colorize;
use log::trace;
use terminal_link::Link;
use crate::gamelogic::konst;



struct JSONFILESTRUCT {
    items: u8,
    monster: u8,
    traps: u8,
    //Head, Torso, pants, shoes,
    armorpieces: Vec<u8>,
    treasures: u8,
    consumables: u8,
    weapons: u8,
    randomroom: u8,

}

impl JSONFILESTRUCT {
    pub fn new(monster: u8, trap: u8, armorpieces: Vec<u8>, treasures: u8, consumables: u8, weapons: u8, randomrom: u8 ) -> Self {

        let mut all_armor: u8 = 0;

        armorpieces.iter().for_each(|amount| {
            all_armor += amount
        });

        Self{
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


pub fn read_json_file(){

}
pub fn check_file(path: &str) {
    match File::open(path) {
        Ok(mut file) => {

            println!("File found :)");

            let json: serde_json::Value = serde_json::from_reader(file)
                .expect("file should be proper JSON");

            let monsters = json.get("monsters").unwrap();
            let traps = json.get("traps").unwrap();
            let items = json.get("items").unwrap();
            let randomrooms = json.get("random_rooms").unwrap();

            println!("{}", monsters);
            println!("{}", traps);
            println!("{}", items);
            println!("{}", randomrooms);




        }
        Err(_) => {

            eprintln!("{}", konst::JSONFILEERROR(Link::new(konst::GAMENAME, konst::GITHUBLINK)))
        }
    }

}



