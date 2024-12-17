use std::any::Any;
use crate::add_log;
use crate::gameobjects::encounter::{Encounter, EncounterTypes};
use crate::gameobjects::monster::Monster;
use std::collections::{HashMap, VecDeque};
use std::ptr::eq;
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, Mutex, OnceLock};
use std::thread;
use crate::gamelogic::terminaldrawer::tdrawer;


pub struct DungeonHandler {
    tx: Sender<()>,

    action_queue: Arc<Mutex<VecDeque<String>>>,
    //dungeon: &'static Dungeon<'static>,
    //player: &'static Player<'static>,
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


                if( dungeon_clone.lock().unwrap().is_combat() == &true){
                    add_log("a");

                }else {
                    if (action.to_ascii_lowercase().eq(&String::from("map"))) {


                        tdrawer::set_render_queue(String::from("map"));

                    } else if (action.eq(&String::from("la"))) {
                        tdrawer::set_render_queue(String::from("room"));
                    } else if cmd_map.get("movement").unwrap().contains(&action) {
                        let m = &cmd_map.get("movement").unwrap();

                        if action.eq(&m[0]) {
                            Dungeon::dungeon_ref().lock().unwrap().move_player("up");
                        } else if action.eq(&m[1]) {
                            Dungeon::dungeon_ref().lock().unwrap().move_player("down");
                        } else if action.eq(&m[2]) {
                            Dungeon::dungeon_ref().lock().unwrap().move_player("left");
                        } else if action.eq(&m[3]) {
                            Dungeon::dungeon_ref().lock().unwrap().move_player("right");

                        }
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
    combat: bool
}

impl Dungeon {
    pub fn new() -> Self {
        let mut rooms = Self::generat_generate_dungeon_rooms(0);
        rooms[0][0].visit_room();


        let dungeon = Self {
            rooms,
            player_position: vec![0,0],
            combat: false,
        };


        dungeon
    }

    pub fn generat_generate_dungeon_rooms(nu_of_rooms: i8) -> Vec<Vec<Dungeonroom>> {
        vec![
            vec![
                Dungeonroom::MonsterRoom("Goblin"),
                Dungeonroom::MonsterRoom("Goblin"),
                Dungeonroom::MonsterRoom("Goblin"),
                Dungeonroom::MonsterRoom("Goblin"),
            ],
            vec![
                Dungeonroom::MonsterRoom("Goblin"),
                Dungeonroom::EmptyRoom("Empty"),
                Dungeonroom::MonsterRoom("Goblin"),
                Dungeonroom::EmptyRoom("Empty"),
            ],
            vec![
                Dungeonroom::MonsterRoom("Goblin"),
                Dungeonroom::MonsterRoom("Goblin"),
                Dungeonroom::MonsterRoom("Goblin"),
                Dungeonroom::EmptyRoom("Empty"),
            ],
            vec![
                Dungeonroom::MonsterRoom("Goblin"),
                Dungeonroom::MonsterRoom("Goblin"),
                Dungeonroom::MonsterRoom("Goblin"),
                Dungeonroom::MonsterRoom("Goblin"),
            ],
            vec![
                Dungeonroom::MonsterRoom("Goblin"),
                Dungeonroom::MonsterRoom("Goblin"),
                Dungeonroom::EmptyRoom("Empty"),
                Dungeonroom::MonsterRoom("Goblin"),
            ],
        ]
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
            let a = Arc::new(Mutex::new(HashMap::from([(
                "movement".to_string(),
                vec![
                    "move up".to_string(),
                    "move down".to_string(),
                    "move left".to_string(),
                    "move right".to_string(),
                ],
            )])));
            a
        })
    }

    pub fn get_player_position(&self) -> &Vec<i8>{
        &self.player_position
    }
    pub fn get_current_room(&self) -> &Dungeonroom {
        &self.rooms[0][0]
    }
    pub fn get_all_rooms(&self) -> &Vec<Vec<Dungeonroom>> {
        &self.rooms
    }

    pub fn is_combat(&self) -> &bool {
        &self.combat
    }

    pub fn move_player(&mut self, direction: &str) {
        let pp = &self.player_position;

        if(direction.eq("up") ){

            if let Some(index) = (pp[0] as usize).checked_sub(1) {
                let next_room:&Dungeonroom = &self.rooms[(pp[0] - 1) as usize][pp[1] as usize];

                if(next_room.enterable){
                    self.player_position = vec![pp[0] - 1, pp[1]];
                    self.check_room();
                }else {
                    add_log("wall");
                }

            }else {
                add_log("wall");
            }
        } else if(direction.eq("down") ){

            if pp[0] + 1 <= (self.rooms.len() - 1) as i8 {
                let next_room: &Dungeonroom = &self.rooms[(pp[0] + 1) as usize][pp[1] as usize];

                if(next_room.enterable){
                    self.player_position = vec![pp[0] + 1, pp[1]];
                    self.check_room();
                }else {
                    add_log("wall");
                }

            }else {
                add_log("wall");
            }
        }else if(direction.eq("left") ){

            if let Some(index) = (pp[1] as usize).checked_sub(1) {
                let next_room: &Dungeonroom = &self.rooms[pp[0]  as usize][(pp[1] - 1) as usize];

                if(next_room.enterable){
                    self.player_position = vec![pp[0] , pp[1] - 1];
                    self.check_room();
                }else {
                    add_log("wall");
                }

            }else {
                add_log("wall");
            }
        }else if(direction.eq("right")) {

            if pp[1] + 1 <= ((&self.rooms[pp[0] as usize] as &Vec<Dungeonroom>).len() - 1) as i8 {


                let next_room: &Dungeonroom = &self.rooms[pp[0]  as usize][(pp[1] + 1) as usize];

                if(next_room.enterable){
                    self.player_position = vec![pp[0] , pp[1] + 1];
                    self.check_room();
                }else {
                    add_log("wall");
                }

            }else {
                add_log("wall");
            }
        }


    }
    pub fn check_room(&mut self){
        if(self.get_current_room().encoutner.get_Type().eq("Monster")){
            self.combat = true;
            tdrawer::set_render_queue("combat".parse().unwrap());
        }

    }

}



pub struct Dungeonroom {
    encoutner: EncounterTypes,
    visited: bool,
    enterable: bool,
}

impl Dungeonroom {
    pub(crate) fn get_room_title(&self) -> String {
        if (!*&self.visited) {
            "???".to_string()
        } else {
            self.encoutner.get_Type().to_string()
        }
    }
}

impl Dungeonroom {
    pub fn MonsterRoom(name: &str) -> Self {
        Self {
            encoutner: EncounterTypes::Monster(Monster::new("Goblin".to_string())),
            visited: false,
            enterable: true,
        }
    }

    pub fn EmptyRoom(name: &str) -> Self {
        Self {
            enterable: false,
            encoutner: EncounterTypes::Empty,
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
}
