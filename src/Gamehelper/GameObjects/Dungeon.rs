use crate::gamelogic::game_screens::WindowContents;


use crate::gamelogic::reader::{generate_monster, generate_random_room, generate_trap, read_encounter_category};
use crate::gamelogic::terminaldrawer::drawer;
use crate::gameobjects::encounter::EncounterTypes::Empty;
use crate::gameobjects::encounter::{Encounter, EncounterTypes};
use crate::gameobjects::item_handler::{Equipmintslots, Item, ItemsTypes, Raritys};
use crate::gameobjects::monster::Monster;
use crate::gameobjects::player::Player;
use crate::gameobjects::trap::Trap;
use crate::gameobjects::treasure::Treasure;
use crate::gameobjects::weaponitem::WeaponItem;
use crate::{add_log, gamestate_ref, Gamestate};
use colored::Colorize;
use crossterm::style::Stylize;
use rand::Rng;
use ratatui::DefaultTerminal;
use std::any::Any;
use std::collections::{HashMap, VecDeque};
use std::fmt::format;
use std::ops::{Deref, DerefMut};
use std::ptr::eq;
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, Mutex, OnceLock};
use std::thread;
use crate::gamelogic::konst;

//This clase will handle the gameloop and all the game mechanics
pub(crate) struct Dungeon {
    rooms: Vec<Vec<Dungeonroom>>,
    player_position: Vec<i8>,
    combat: bool,
}

impl Dungeon {
    pub fn new() -> Self {
        let testing = false;
        let mut rooms = if !testing {
            Self::generator_maze(10, 15)
        } else {
            vec![vec![
                generate_random_room().unwrap(),
                Dungeonroom::MonsterRoom(),
                Dungeonroom::GoalRoom(),
            ]]
        };

        let dungeon = Self {
            rooms,
            player_position: vec![0, 0],
            combat: false,
        };

        dungeon
    }

    pub fn generator_maze(w: i8, h: i8) -> Vec<Vec<Dungeonroom>> {
        let mut maze: Vec<Vec<Dungeonroom>> = (0..h)
            .map(|height| {
                let Dungeonrow = (0..w)
                    .map(|width| Dungeonroom::None())
                    .collect::<Vec<Dungeonroom>>();
                Dungeonrow
            })
            .collect::<Vec<Vec<Dungeonroom>>>();

        let starting_point = (0, 0);

        for i in 0..maze.len() {
            for j in 0..maze[i].len() {
                maze[i][j] = Dungeonroom::randomRoom();
            }
        }

        let gh = rand::rng().random_range((h / 2) as usize..=maze.len());
        let gw = rand::rng().random_range((w / 2) as usize..=maze[0].len());
        maze[starting_point.0][starting_point.1] = Dungeonroom::StartingRoom();
        maze[gh - 1][gw - 1] = Dungeonroom::GoalRoom();

        for i in 0..maze.len() {
            for j in 0..maze[i].len() {
                let mut counter = 0;
                let mut none_rooms = vec![];
                if (j + 1 < maze[i].len() && maze[i][j + 1].encoutner.get_Type().eq("None")) {
                    counter = counter + 1;
                    none_rooms.push((i, j + 1))
                }
                if (j as i8 - 1 >= 0 && maze[i][j - 1].encoutner.get_Type().eq("None")) {
                    counter = counter + 1;
                    none_rooms.push((i, j - 1))
                }
                if (i + 1 < maze.len() && maze[i + 1][j].encoutner.get_Type().eq("None")) {
                    counter = counter + 1;
                    none_rooms.push((i + 1, j))
                }
                if (i as i8 - 1 >= 0 && maze[i - 1][j].encoutner.get_Type().eq("None")) {
                    counter = counter + 1;
                    none_rooms.push((i - 1, j))
                }
                if (j == 0 || i == 0 || j == maze[i].len() || i == maze.len()) {
                    if (counter >= 2) {
                        let random_number = rand::rng().random_range(0..=(counter - 1));
                        maze[none_rooms[random_number].0][none_rooms[random_number].1] =
                            generate_random_room().unwrap()
                    }
                }
                if (counter >= 3) {
                    let random_number = rand::rng().random_range(0..=(counter - 1));
                    maze[none_rooms[random_number].0][none_rooms[random_number].1] =
                        generate_random_room().unwrap()
                }
            }
        }

        maze
    }

    pub fn set_combat(&mut self, combat: bool) {
        self.combat = combat
    }
    pub fn generat_generate_dungeon_rooms(width: i8, height: i8) -> Vec<Vec<Dungeonroom>> {
        let rooms = (0..height)
            .map(|row| {
                (0..width)
                    .map(|roomnumber| Dungeonroom::randomRoom())
                    .collect::<Vec<Dungeonroom>>()
            })
            .collect::<Vec<Vec<Dungeonroom>>>();
        rooms
    }

    pub fn dungeon_ref() -> &'static Arc<Mutex<Dungeon>> {
        static DUNGEON: OnceLock<Arc<Mutex<Dungeon>>> = OnceLock::new();

        DUNGEON.get_or_init(|| {
            let dungeon = Arc::new(Mutex::new(Dungeon::new()));
            dungeon
        })
    }

    pub fn get_player_position(&self) -> &Vec<i8> {
        &self.player_position
    }
    pub fn get_current_room(&mut self) -> &mut Dungeonroom {
        let pp = &self.player_position;
        &mut self.rooms[pp[0] as usize][pp[1] as usize]
    }
    pub fn get_all_rooms(&self) -> &Vec<Vec<Dungeonroom>> {
        &self.rooms
    }

    pub fn is_combat(&self) -> &bool {
        &self.combat
    }

    pub fn move_player(&mut self, direction: &str) {
        let pp = &self.player_position;

        if (direction.eq("up")) {
            if let Some(index) = (pp[0] as usize).checked_sub(1) {
                let next_room: &Dungeonroom = &self.rooms[(pp[0] - 1) as usize][pp[1] as usize];

                if (next_room.enterable) {
                    self.player_position = vec![pp[0] - 1, pp[1]];
                    self.check_room();
                } else {
                    add_log("Dungeon: You ran into a wall")
                }
            } else {
                add_log("Dungeon: You ran into a wall");
            }
        } else if (direction.eq("down")) {
            if pp[0] + 1 <= (self.rooms.len() - 1) as i8 {
                let next_room: &Dungeonroom = &self.rooms[(pp[0] + 1) as usize][pp[1] as usize];

                if (next_room.enterable) {
                    self.player_position = vec![pp[0] + 1, pp[1]];
                    self.check_room();
                } else {
                    add_log("Dungeon: You ran into a wall");
                }
            } else {
                add_log("Dungeon: You ran into a wall");
            }
        } else if (direction.eq("left")) {
            if let Some(index) = (pp[1] as usize).checked_sub(1) {
                let next_room: &Dungeonroom = &self.rooms[pp[0] as usize][(pp[1] - 1) as usize];

                if (next_room.enterable) {
                    self.player_position = vec![pp[0], pp[1] - 1];
                    self.check_room();
                } else {
                    add_log("Dungeon: You ran into a wall");
                }
            } else {
                add_log("Dungeon: You ran into a wall");
            }
        } else if (direction.eq("right")) {
            if pp[1] + 1 <= ((&self.rooms[pp[0] as usize] as &Vec<Dungeonroom>).len() - 1) as i8 {
                let next_room: &Dungeonroom = &self.rooms[pp[0] as usize][(pp[1] + 1) as usize];

                if (next_room.enterable) {
                    self.player_position = vec![pp[0], pp[1] + 1];
                    self.check_room();
                } else {
                    add_log("Dungeon: You ran into a wall");
                }
            } else {
                add_log("Dungeon: You ran into a wall");
            }
        }
    }
    pub fn check_room(&mut self) {
        let room = self.get_current_room();

        match &mut room.encoutner {
            EncounterTypes::Monster(monster) => {
                if (monster.is_alive()) {
                    self.combat = true;
                }
            }
            EncounterTypes::Trap(trap) => {
                if trap.make_skillcheck(*Player::player_ref().lock().unwrap().get_skill()) {
                    trap.is_spotted();
                } else {
                    Player::player_ref()
                        .lock()
                        .unwrap()
                        .take_true_dmg(*trap.get_dmg());
                }
            }
            EncounterTypes::Goal(monster) => {
                if (monster.is_alive()) {
                    self.combat = true;
                }
            }
            _ => {}
        }

        self.get_current_room().visited = true;
    }

    pub fn generate_new_dungeon() {
        let mut dungeon = Self::dungeon_ref().lock().unwrap();
        *dungeon = Self::new();
    }
}

pub struct Dungeonroom {
    pub(crate) encoutner: EncounterTypes,
    visited: bool,
    enterable: bool,
    note: String,
}

impl Dungeonroom {
    pub(crate) fn get_room_title(&self) -> String {
        if (!&self.visited) {
            "???".to_string()
        } else {
            self.encoutner.get_Name().to_string()
        }
    }
    pub fn randomRoom() -> Self {
        let random_number = rand::rng().random_range(0..=4);

        match random_number {
            0 => Dungeonroom::MonsterRoom(),
            1 => generate_random_room().unwrap(),
            2 => Dungeonroom::TrapRoom(),
            3 => Dungeonroom::None(),
            4 => Dungeonroom::TreaureRoom(),
            _ => Dungeonroom::None(),
        }
    }
    pub fn MonsterRoom() -> Self {
        Self {
            encoutner: generate_monster(&read_encounter_category(konst::MONSTERCAT.into()).unwrap()),
            visited: true, //todo change
            enterable: true,
            note: String::new(),
        }
    }

    pub fn GoalRoom() -> Self {
        Self {
            encoutner: EncounterTypes::Goal(Monster::new_Boss("Olaf".into())),
            visited: false, //todo change
            enterable: true,
            note: String::new(),
        }
    }

    pub fn TrapRoom() -> Self {
        Self {
            enterable: true,
            visited: false, //change
            encoutner: generate_trap(&read_encounter_category("traps".into()).unwrap()),
            note: String::new(),
        }
    }

    pub fn StartingRoom() -> Self {
        generate_random_room().unwrap()
    }

    pub fn fillerRoom(name: String, des: String, note: String) -> Self {
        Self {
            encoutner: EncounterTypes::Empty(crate::gameobjects::empty::Empty::new(name, des)),
            enterable: true,
            visited: true,
            note,
        }
    }

    pub fn TreaureRoom() -> Self {
        Self {
            enterable: true,
            encoutner: EncounterTypes::Treasure(Treasure::new()),
            note: String::new(),
            visited: false,
        }
    }

    pub fn None() -> Self {
        Self {
            enterable: false,
            encoutner: EncounterTypes::None,
            visited: false,
            note: String::new(),
        }
    }

    pub fn visit_room(&mut self) {
        self.visited = true;
    }

    pub fn set_note(&mut self, note: String) {
        self.note = note
    }

    pub fn get_note(&self) -> &str {
        &self.note
    }

    pub fn is_enterable(&self) -> bool {
        self.enterable
    }

    pub fn get_Type(&self) -> &str {
        &self.encoutner.get_Type()
    }

    pub fn get_des(&self) -> &str {
        &self.encoutner.get_description()
    }

    pub fn get_Monster(&mut self) -> Option<&mut Monster> {
        match &mut self.encoutner {
            EncounterTypes::Monster(monster) => Some(monster),
            EncounterTypes::Goal(monster) => Some(monster),
            _ => None,
        }
    }

    pub fn clearMonsterRoom(&mut self) {
        self.note = "A unlooted corpse lays on the ground.\n Looks yummy\n".into();
        match &mut self.encoutner {
            EncounterTypes::Monster(monster) => {
                monster.dead();
            }
            _ => {}
        }
    }

    pub fn handleLoot(&mut self) {
        let mut player = Player::player_ref().lock().unwrap();
        if(player.has_free_inventory_slot()) {
            match &mut self.encoutner {
                EncounterTypes::Monster(monster) => {
                    for item in monster.drop() {
                        if (!player.add_loot(item)) {
                            add_log("Your inventory is full")
                        } else {
                            self.note = "".parse().unwrap();
                        }
                    }
                }
                EncounterTypes::Treasure(treaure) => {
                    for item in treaure.take() {
                        if (!player.add_loot(item)) {
                            add_log("Your inventory is full")
                        } else {
                            self.note = "".parse().unwrap();
                        }
                    }
                }

                _ => {}
            }
        } else {
            add_log("Dungeon: Your inventory is full")
        }
    }
}
