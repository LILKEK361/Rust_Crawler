use crate::gamelogic;
use crate::gameobjects::encounter::Encounter;
use crate::gameobjects::item_handler::{ItemsTypes, Raritys};
use crate::gameobjects::weaponitem::WeaponItem;
use crossterm::style::Stylize;

#[derive(Clone)]
pub(crate) struct Monster {
    pub name: String,
    m_type: String,
    des: String,
    alive: bool,
    hp: u8,
    dmg: u8,
    max_hp: u8,
    loot: Vec<ItemsTypes>,
}

impl Monster {
    pub fn new(name: String) -> Self {
        Self {
            name: String::from(&name),
            m_type: "Monster".into(),
            des: format!("A {name} is viben in the room. Oh no it attacks").into(),
            alive: true,
            hp: 25,
            max_hp: 25,
            dmg: 6,
            loot: vec![gamelogic::gamehelperfunctions::generate_random_drop()],
        }
    }

    pub fn from_json(name: String, hp: u8, dmg: u8, des: String) -> Self{
        Self {
            name,
            m_type: "Monster".into(),
            des,
            alive: true,
            hp,
            max_hp: hp,
            dmg,
            loot: vec![],
        }
    }

    pub fn new_Boss(name: String) -> Self {
        Self {
            name: String::from(&name),
            m_type: "Boss".into(),
            des: format!("A big, chunky {name} is viben. Looks like its attacking.").into(),
            alive: true,
            hp: 2,
            max_hp: 255,
            dmg: 15,
            loot: vec![],
        }
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }
    pub fn take_dmg(&mut self, dmg: u8) {
        if (!(dmg > self.hp)) {
            self.hp = self.hp - dmg;
            if (self.hp <= 0) {
                self.alive = false;
                self.dead()
            }
        } else {
            self.hp = self.hp - self.hp;
            self.alive = false;
        }
    }
    pub fn get_dmg(&self) -> &u8 {
        &self.dmg
    }

    pub fn get_hp(&self) -> &u8 {
        &self.hp
    }

    pub fn get_max_hp(&self) -> &u8 {
        &self.max_hp
    }

    pub fn dead(&mut self) {
        let monster = &self.name;

        self.des = format!("A {monster} lies on the ground.\n Dead\nYou killed it.",);
        self.name = format!("Dead {monster}")
    }

    pub fn drop(&mut self) -> Vec<ItemsTypes> {
        //todo
        let items = self.loot.to_owned();
        self.loot = vec![];
        items
    }
}

impl Encounter for Monster {
    fn get_Name(&self) -> &str {
        &self.name
    }

    fn get_Type(&self) -> &str {
        &self.m_type
    }

    fn get_description(&self) -> &str {
        &self.des
    }
}
