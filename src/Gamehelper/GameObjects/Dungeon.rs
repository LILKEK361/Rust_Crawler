use crate::gameobjects::monster_handler::{Monster};
use std::any::Any;
use std::io::Read;
use crate::gameobjects::encounter::Encounter;

use crate::gameobjects::player::Player;

//This clase will handle the gameloop and all the game mechanics
pub(crate) struct Dungeon<'a> {
    player: &'a Player<'a>,
    rooms: &'a [Dungeonroom],
    player_position: i8,
    //Will later be changed

}


impl<'a> Dungeon<'a> {

    pub fn new(player: &'a Player<'a>, )-> Self{

        let dungeon = Self{
            player,
            rooms: Self::generat_generate_dungeon_rooms(),

            player_position: 0,
        };
        dungeon
    }
    pub fn generat_generate_dungeon_rooms() -> &'a [Dungeonroom] {

        &[]
    }


    pub fn Dungeon_run(self)-> bool {
        let mut input = String::new();

        while (self.player.alive) {
            std::io::stdin().read_line(&mut input).unwrap();

            if input == "" {

            } else {
                return false
            }

        }

        true
    }

}



pub struct Dungeonroom {
    encoutner: Box<dyn Encounter> ,

}

impl Dungeonroom{


    pub fn MonsterRoom(name: &str) -> Self{

        Self{
            encoutner: Box::new(Monster::new(name))
        }
    }



    pub fn get_Type<T: Encounter>(encounter: &T, ) -> &str {
        return &encounter.get_Name();
    }

}


