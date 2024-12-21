use crate::add_log;
use crate::gamelogic::terminaldrawer::tdrawer;
use crate::gameobjects::encounter::{Encounter, EncounterTypes};
use crate::gameobjects::monster::Monster;
use std::any::Any;
use std::ascii::AsciiExt;
use std::collections::{HashMap, VecDeque};
use std::ops::Deref;
use std::ptr::eq;
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, Mutex, OnceLock};
use std::thread;
use rand::Rng;
use crate::gameobjects::player::Player;
use crate::gameobjects::trap::Trap;

pub struct DungeonHandler {

    tx: Sender<()>,
    action_queue: Arc<Mutex<VecDeque<String>>>,

}

impl DungeonHandler {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();

        //Queue for storing the actions
        let action_queue = Arc::new(Mutex::new(VecDeque::<String>::new()));

        let action_queue_clone = Arc::clone(&action_queue);

        let dungeon_clone = Arc::clone(&Dungeon::dungeon_ref());

        let cmd_map_clone = Arc::clone(&Dungeon::dugeon_cmd_ref());

        let handle = thread::spawn(move || {
            loop {
                //Check if there is any action in the queue, blocks until there is an action
                rx.recv().unwrap();
                //match logic for player inputs needs to be added next

                let cmd_map = cmd_map_clone.lock().unwrap();

                let mut action_queue = action_queue_clone.lock().unwrap();

                let action = action_queue.pop_front().unwrap();


                if (dungeon_clone.lock().unwrap().is_combat() == &true) {
                    if(cmd_map.get("combat").unwrap().contains(&action.to_ascii_lowercase())){

                        let combat_action = cmd_map.get("combat").unwrap();
                        let mut dungeon = Dungeon::dungeon_ref().lock().unwrap();
                        let mut dungeonroom = dungeon.get_current_room();
                        let monster = dungeonroom.get_Monster().unwrap();
                        let mut player = Player::player_ref().lock().unwrap();

                        if(action.eq(&combat_action[0]/*attack*/)){
                            monster.take_dmg(*player.attack());
                            player.take_dmg(*monster.get_dmg());
                            if(!monster.is_alive()){
                                dungeonroom.clearMonsterRoom(&player);
                                dungeon.combat = false;
                                tdrawer::set_render_queue("map".into())

                            }
                        }else if(action.eq(&combat_action[1] /*Defend*/)){
                            player.defend(*monster.get_dmg());
                        }

                    }else {
                        add_log("You can't use this action in combat");
                    }


                } else {
                    if (action.to_ascii_lowercase().eq(&String::from("map"))) {
                        tdrawer::set_render_queue(String::from("map"));

                    } else if cmd_map.get("movement").unwrap().contains(&action) {
                        let movment = &cmd_map.get("movement").unwrap();

                        if action.eq(&movment[0]) {
                            Dungeon::dungeon_ref().lock().unwrap().move_player("up");
                        } else if action.eq(&movment[1]) {
                            Dungeon::dungeon_ref().lock().unwrap().move_player("down");
                        } else if action.eq(&movment[2]) {
                            Dungeon::dungeon_ref().lock().unwrap().move_player("left");
                        } else if action.eq(&movment[3]) {
                            Dungeon::dungeon_ref().lock().unwrap().move_player("right");
                        }
                    } else if(action.to_ascii_lowercase().eq(&String::from("inventory"))){
                        tdrawer::set_render_queue("inventory".into());
                    }else if(action.to_ascii_lowercase().eq(&String::from("look around")) || action.to_ascii_lowercase().eq(&String::from("la"))) {
                        tdrawer::set_render_queue("look".into());
                    } else if(action.eq(&String::from("help"))){
                        tdrawer::set_render_queue("help".into())
                    }
                    else {
                        add_log("Unvaild Command")
                    }
                }
            }
        });

        drop(handle);

        Self { tx, action_queue }
    }

    pub fn send_action(&mut self, action: String) {
        if let Ok(mut queue) = self.action_queue.lock() {
            queue.push_back(action);
        } else {
            eprintln!("Failed to lock action queue");
        }

        if self.tx.send(()).is_err() {
            eprintln!("Failed to send message to thread. Receiver might be closed.");
        }
    }

    pub fn dungeon_handler_ref() -> &'static Mutex<DungeonHandler> {
        static DUNGEONHANDLER: OnceLock<Mutex<DungeonHandler>> = OnceLock::new();

        DUNGEONHANDLER.get_or_init(|| {
            let dungeonhandler = Mutex::new(DungeonHandler::new());
            dungeonhandler
        })
    }
}

//This clase will handle the gameloop and all the game mechanics
pub(crate) struct Dungeon {
    rooms: Vec<Vec<Dungeonroom>>,
    player_position: Vec<i8>,
    combat: bool,

}

impl Dungeon {
    pub fn new() -> Self {
        let mut rooms = Self::generat_generate_dungeon_rooms(8,4);
        rooms[0][0].visit_room();


        let dungeon = Self {
            rooms,
            player_position: vec![0, 0],
            combat: false,
        };

        dungeon
    }

    pub fn generat_generate_dungeon_rooms(width: i8, height: i8) -> Vec<Vec<Dungeonroom>> {

        let rooms = (0..height).map(|row|  {
            (0..width).map(|roomnumber|{
                Dungeonroom::randomRoom()
            }).collect::<Vec<Dungeonroom>>()
        }).collect::<Vec<Vec<Dungeonroom>>>();
        rooms
    }

    pub fn dungeon_ref() -> &'static Arc<Mutex<Dungeon>> {
        static DUNGEON: OnceLock<Arc<Mutex<Dungeon>>> = OnceLock::new();

        DUNGEON.get_or_init(|| {
            let dungeon = Arc::new(Mutex::new(Dungeon::new()));
            dungeon
        })
    }

    pub fn dugeon_cmd_ref() -> &'static Arc<Mutex<HashMap<String, Vec<String>>>> {
        static CMD_MAP: OnceLock<Arc<Mutex<HashMap<String, Vec<String>>>>> = OnceLock::new();

        CMD_MAP.get_or_init(|| {
            let cmd_map = Arc::new(Mutex::new(HashMap::from([
                (
                   "movement".to_string(),
                    vec![
                        "up".to_string(),
                        "down".to_string(),
                        "left".to_string(),
                        "right".to_string(),
                    ],
                ),
                (
                    "combat".to_string(),
                    vec![
                        "attack".to_string(),
                        "defend".to_string(),

                    ]

                ),

            ])));
            cmd_map
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
                    add_log("wall");
                }
            } else {
                add_log("wall");
            }
        } else if (direction.eq("down")) {
            if pp[0] + 1 <= (self.rooms.len() - 1) as i8 {
                let next_room: &Dungeonroom = &self.rooms[(pp[0] + 1) as usize][pp[1] as usize];

                if (next_room.enterable) {
                    self.player_position = vec![pp[0] + 1, pp[1]];
                    self.check_room();
                } else {
                    add_log("wall");
                }
            } else {
                add_log("wall");
            }
        } else if (direction.eq("left")) {
            if let Some(index) = (pp[1] as usize).checked_sub(1) {
                let next_room: &Dungeonroom = &self.rooms[pp[0] as usize][(pp[1] - 1) as usize];

                if (next_room.enterable) {
                    self.player_position = vec![pp[0], pp[1] - 1];
                    self.check_room();
                } else {
                    add_log("wall");
                }
            } else {
                add_log("wall");
            }
        } else if (direction.eq("right")) {
            if pp[1] + 1 <= ((&self.rooms[pp[0] as usize] as &Vec<Dungeonroom>).len() - 1) as i8 {
                let next_room: &Dungeonroom = &self.rooms[pp[0] as usize][(pp[1] + 1) as usize];

                if (next_room.enterable) {
                    self.player_position = vec![pp[0], pp[1] + 1];
                    self.check_room();
                } else {
                    add_log("wall");
                }
            } else {
                add_log("wall");
            }
        }
    }
    pub fn check_room(&mut self) {

        let room = self.get_current_room();
        match &room.encoutner  {
            EncounterTypes::Monster(monster) => {
                if(monster.is_alive()){
                    self.combat = true;
                    tdrawer::set_render_queue("combat".parse().unwrap());
                }
            }
            _ => {}
        }
        self.get_current_room().visited = true
    }
}

pub struct Dungeonroom {
    encoutner: EncounterTypes,
    visited: bool,
    enterable: bool,
}

impl Dungeonroom {
    pub(crate) fn get_room_title(&self) -> String {
        if (!&self.visited) {
            "???".to_string()
        } else {
            self.encoutner.get_Name().to_string()
        }
    }
    pub fn randomRoom() -> Self{
        let random_number = rand::rng().random_range(0..=3);
        add_log(&*random_number.to_string());
        match random_number {
            0 => {Dungeonroom::MonsterRoom("Nigga".into())},
            1 => {Dungeonroom::EmptyRoom("E")},
            2 => {Dungeonroom::MonsterRoom("Nigga".into())},
            3 => {Dungeonroom::EmptyRoom("E")},
            _ => {Dungeonroom::None()},
        }
    }
    pub fn MonsterRoom(name: String) -> Self {
        Self {
            encoutner: EncounterTypes::Monster(Monster::new(name)),
            visited: false,
            enterable: true,
        }
    }

    pub fn TrapRoom() -> Self {
        Self {
            enterable: true,
            visited: false,
            encoutner: EncounterTypes::Trap(Trap::new())
        }
    }

    pub fn EmptyRoom(name: &str) -> Self {
        Self {
            enterable: true,
            encoutner: EncounterTypes::Empty,
            visited: true, //todo! change after testing
        }
    }

    pub fn None() -> Self {
        Self{
            enterable: false,
            encoutner: EncounterTypes::None,
            visited: false,
        }
    }

    pub fn visit_room(&mut self) {
        self.visited = true;
    }

    pub fn is_enterable(&self) -> bool {
        self.enterable
    }

    pub fn get_Type(&self) -> &str {
        &self.encoutner.get_Type()
    }

    pub fn get_des(&self) -> &str{
        &self.encoutner.get_description()
    }



    pub fn get_dmg_from_Monster(&self) -> &i8{
        match &self.encoutner {
            EncounterTypes::Monster(Monster) => {
                Monster.get_dmg()
            },
            _ => {
                &0
            }
        }
    }

    pub fn get_Monster(&mut self) -> Option<&mut Monster>{
        match &mut self.encoutner {
            EncounterTypes::Monster(monster) => {
                Some(monster)
            }
            _ => None
        }
    }

    pub fn clearMonsterRoom(&mut self, player: &Player){
        match &mut self.encoutner{
            EncounterTypes::Monster(monster) => {

                monster.dead();

            }
            _ => {}
        }


    }

}
