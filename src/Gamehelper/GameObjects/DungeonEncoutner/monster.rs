use crate::gameobjects::encounter::Encounter;
use crate::gameobjects::item_handler::ItemsTypes;

#[derive(Clone)]
pub(crate) struct Monster {
    pub name: String,
    m_type: String,
    des: String,
    alvie: bool,
    hp: u8,
    dmg: i8,
    max_hp: i8,
    loot: Vec<ItemsTypes>,



}

impl Monster {
    pub fn new(name: String) -> Self {
        Self {
            name: String::from(&name),
            m_type: "Monster".into(),
            des: format!("A {name} is viben in the room. Oh no it attacks").into(),
            alvie: true,
            hp: 100,
            max_hp: 100,
            dmg: 10,
            loot: vec![]
        }
    }




    pub fn is_alive(&self) -> bool {
        self.alvie
    }
    pub fn take_dmg(&mut self, dmg: i8) {
        self.hp = self.hp - dmg as u8;
        if(self.hp <= 0){
            self.alvie = false;
        }
    }
    pub fn get_dmg(&self) -> &i8{
        &self.dmg
    }

    pub fn get_hp(&self) -> &u8{
        &self.hp
    }

    pub fn get_max_hp(&self) -> &i8 {
        &self.max_hp
    }

    pub fn dead(&mut self){
        let monster = &self.name;
        self.des = format!("A dead {monster} lies on the ground.\nDead.\nYou killed it.");
        self.name = format!("Dead {monster}")
    }

}


impl Encounter for Monster{
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