use crate::{add_log};
use crate::gamelogic::terminaldrawer::tdrawer;
use crate::gameobjects::encounter::{Encounter, EncounterTypes};
use crate::gameobjects::monster::Monster;
use std::any::Any;
use std::collections::{HashMap, VecDeque};
use std::fmt::format;
use std::ops::{Deref, DerefMut};
use std::ptr::eq;
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, Mutex, OnceLock};
use std::thread;
use rand::Rng;
use crate::gameobjects::encounter::EncounterTypes::Empty;
use crate::gameobjects::item_handler::{Item, ItemsTypes, Raritys};
use crate::gameobjects::player::Player;
use crate::gameobjects::trap::Trap;
use crate::gameobjects::weaponitem::WeaponItem;

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

                        let mut player = Player::player_ref().lock().unwrap();

                        if(action.eq(&combat_action[0]/*attack*/)){
                            dungeonroom.get_Monster().unwrap().take_dmg(*player.attack());

                            if(!dungeonroom.get_Monster().unwrap().is_alive()){
                                dungeonroom.clearMonsterRoom(&player);
                                &dungeon.set_combat(false);
                                tdrawer::set_render_queue("look".into())

                            }else {
                                player.take_dmg(*(dungeonroom.get_Monster().unwrap().get_dmg()));
                            }

                        }else if(action.eq(&combat_action[1] /*Defend*/)){
                            player.defend(*dungeonroom.get_Monster().unwrap().get_dmg());
                        }

                    }else {
                        add_log("You can't use this action in combat");
                    }


                } else if(*Player::player_ref().lock().unwrap().is_in_inventory()){

                    if(action.eq("close".into())){
                        Player::player_ref().lock().unwrap().set_inventory(false);
                        tdrawer::set_render_queue("look".into())
                    }else {
                        add_log("Dungeon: Type close to leave")
                    }
                }

                else {
                    if (action.to_ascii_lowercase().eq("map".into())) {
                        tdrawer::set_render_queue("map".into());

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
                    } else if(action.to_ascii_lowercase().eq("inventory".into())){
                        tdrawer::set_render_queue("inventory".into());
                        Player::player_ref().lock().unwrap().set_inventory(true);
                    }else if(action.to_ascii_lowercase().eq("look around".into()) || action.to_ascii_lowercase().eq("la".into())) {
                        tdrawer::set_render_queue("look".into());
                    } else if(action.eq("help".into())){
                        tdrawer::set_render_queue("help".into())
                    } else if(action.eq("info".into())){
                        tdrawer::set_render_queue("info".into())
                    }else if(action.eq("clear".into()) && Dungeon::dungeon_ref().lock().unwrap().get_current_room().get_Type().eq("Goal")){
                        add_log("Dungeon: Yppi you found the goal.");
                        add_log("If you type clear again you will");
                        add_log("procced...")
                    } else if (action.eq("loot".into()) && !Dungeon::dungeon_ref().lock().unwrap().get_current_room().get_Monster().unwrap().is_alive()){

                        //todo
                        let mut dungeon = Dungeon::dungeon_ref().lock().unwrap();
                        let mut player = Player::player_ref().lock().unwrap();
                        dungeon.get_current_room().set_note("".into());
                        let monster =  dungeon.get_current_room().get_Monster().unwrap();
                        if(!player.add_loot(monster.drop())){
                            add_log("Your inventory is full")
                        }
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
        let testing = false;
        let mut rooms = if !testing {
            Self::generator_maze(10,10)
        } else {
            vec![vec![Dungeonroom::EmptyRoom("E"), Dungeonroom::MonsterRoom("Skeleton".into()),Dungeonroom::TrapRoom()]]
        };



        let dungeon = Self {
            rooms,
            player_position: vec![0, 0],
            combat: false,

        };

        dungeon
    }


    pub fn generator_maze(w: i8, h: i8) -> Vec<Vec<Dungeonroom>>{


        let mut maze: Vec<Vec<Dungeonroom>> = (0..h).map(|height|{
            let Dungeonrow = (0..w).map(|width|{
                Dungeonroom::None()
            }).collect::<Vec<Dungeonroom>>();
            Dungeonrow
        }).collect::<Vec<Vec<Dungeonroom>>>();

        let starting_point = (0,0);






        for i in 0..maze.len() {
            for j in 0..maze[i].len() {
                maze[i][j] = Dungeonroom::randomRoom();

            }
        }

        let gh = rand::rng().random_range((h / 2) as usize..=maze.len());
        let gw =  rand::rng().random_range((w / 2) as usize..=maze.len());
        maze[starting_point.0][starting_point.1] = Dungeonroom::StartingRoom();
        maze[gh - 1][gw - 1] = Dungeonroom::GoalRoom();

        for i in 0..maze.len() {
            for j in 0..maze[i].len() {
                let mut counter = 0;
                let mut none_rooms = vec![];
                if( j + 1 < maze[i].len() && maze[i][j+1].encoutner.get_Type().eq("None")){
                    counter = counter + 1;
                    none_rooms.push((i,j+1))
                }
                if( j as i8 - 1 >= 0 && maze[i][j -1].encoutner.get_Type().eq("None")){
                    counter = counter + 1;
                    none_rooms.push((i,j-1))
                }
                if( i + 1 < maze.len() && maze[i + 1][j].encoutner.get_Type().eq("None")){
                    counter = counter + 1;
                    none_rooms.push((i+1,j))
                }
                if( i as i8 -1 >= 0 && maze[i - 1][j].encoutner.get_Type().eq("None")){
                    counter = counter + 1;
                    none_rooms.push((i-1,j))
                }
                if(i==0 && j==0 || i== 0 || i == maze.len()){
                    if(counter >= 2){
                        let random_number = rand::rng().random_range(0..=(counter - 1));
                        maze[none_rooms[random_number].0][none_rooms[random_number].1] = Dungeonroom::EmptyRoom("Empty")

                    }
                }
                if(counter >= 3){
                    let random_number = rand::rng().random_range(0..=(counter - 1));
                    maze[none_rooms[random_number].0][none_rooms[random_number].1] = Dungeonroom::EmptyRoom("Empty")

                }

            }
        }



        maze


    }

    pub fn set_combat(&mut self, combat:bool){
        self.combat = combat
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
                    add_log("Dungeon: You ran into a wall");                }
            } else {
                add_log("Dungeon: You ran into a wall");            }
        } else if (direction.eq("down")) {
            if pp[0] + 1 <= (self.rooms.len() - 1) as i8 {
                let next_room: &Dungeonroom = &self.rooms[(pp[0] + 1) as usize][pp[1] as usize];

                if (next_room.enterable) {
                    self.player_position = vec![pp[0] + 1, pp[1]];
                    self.check_room();
                } else {
                    add_log("Dungeon: You ran into a wall");("wall");
                }
            } else {
                add_log("Dungeon: You ran into a wall");            }
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
        match &mut room.encoutner  {
            EncounterTypes::Monster(monster) => {
                if(monster.is_alive()){
                    self.combat = true;
                    tdrawer::set_render_queue("combat".parse().unwrap());
                }
            },
            EncounterTypes::Trap(trap) => {
                if trap.make_skillcheck(*Player::player_ref().lock().unwrap().get_skill()){
                    trap.is_spotted()


                } else {
                    Player::player_ref().lock().unwrap().take_dmg(*trap.get_dmg());
                    add_log("Dungeon: You feel a sudden sting.");
                    add_log("Are thoose the bug they talked about?")
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
    note: String
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

            match random_number {
                0 => { Dungeonroom::MonsterRoom("Goblin".into()) }
                1 => { Dungeonroom::EmptyRoom("E") }
                2 => { Dungeonroom::TrapRoom() }
                3 => { Dungeonroom::None() }
                _ => { Dungeonroom::None() }
            }

    }
    pub fn MonsterRoom(name: String) -> Self {
        Self {
            encoutner: EncounterTypes::Monster(Monster::new(name)),
            visited: false, //todo change
            enterable: true,
            note: String::new()
        }
    }

    pub fn GoalRoom() -> Self {
        Self {
            encoutner: EncounterTypes::Goal,
            visited: true, //todo change
            enterable: true,
            note: String::new()
        }
    }

    pub fn TrapRoom() -> Self {
        Self {
            enterable: true,
            visited: false, //change
            encoutner: EncounterTypes::Trap(Trap::new()),
            note: String::new()
        }
    }

    pub fn StartingRoom() -> Self {
        Self{
            encoutner: EncounterTypes::Empty,
            visited:true,
            enterable: true,
            note: String::new()
        }
    }

    pub fn EmptyRoom(name: &str) -> Self {
        Self {
            enterable: true,
            encoutner: EncounterTypes::Empty,
            visited: false, //todo! change after testing
            note: String::new()
        }
    }

    pub fn None() -> Self {
        Self{
            enterable: false,
            encoutner: EncounterTypes::None,
            visited: true,
            note: String::new()
        }
    }

    pub fn visit_room(&mut self) {
        self.visited = true;
    }

    pub fn set_note(&mut self, note: String){
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
        self.note = "A unlooted corpse lays on the ground.\n Looks yummy\n".into();
        match &mut self.encoutner{
            EncounterTypes::Monster(monster) => {

                monster.dead();

            }
            _ => {}
        }


    }

}
