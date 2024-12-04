use crate::gameobjects::monster::{Monster};
use std::any::Any;
use std::collections::VecDeque;
use std::io::Read;
use std::iter::Once;
use std::ops::{Deref, DerefMut};
use std::sync::{mpsc, Arc, Mutex, OnceLock};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;
use std::time::Duration;
use ratatui::DefaultTerminal;

use ratatui::widgets::{Block, Borders};
use serde::Serialize;
use crate::gameobjects::encounter::{Encounter, EncounterTypes};

use crate::gameobjects::player::Player;
use crate::{add_log, log_ref, tdrawer_ref};
use crate::terminaldrawer::tdrawer;

enum DungeonCommands{
    Lookaround,
}

//Returns the possible command in long and short form
impl DungeonCommands{
    pub fn to_string(&self) -> (String, String){
        match self {
            DungeonCommands::Lookaround => ("Look around".into(), "la".into()),
        }
    }
}

pub struct DungeonHandler{

        tx: Sender<()>,

        action_queue: Arc<Mutex<VecDeque<String>>>,
        //dungeon: &'static Dungeon<'static>,
        //player: &'static Player<'static>,



}

impl DungeonHandler{



    pub fn new() -> Self {
        
        let (tx, rx) = mpsc::channel();

        //Queue for storing the actions
        let action_queue = Arc::new(Mutex::new(VecDeque::<String>::new()));

        let action_queue_clone = Arc::clone(&action_queue);

        let dungeon_clone = Arc::clone(&Dungeon::dungeon_ref());


        let handle = std::thread::spawn(move || {

            let dungeon = dungeon_clone.lock().unwrap();


            loop {

                //Check if there is any action in the queue, blocks until there is an action
                rx.recv().unwrap();
                //match logic for player inputs needs to be added next
                let mut action_queue = action_queue_clone.lock().unwrap();

                match let Some(action) = action_queue.pop_front().unwrap() {

                }

            }
        });

        drop(handle);

        Self {
            tx,
            action_queue,
        }


    }

    pub fn get_dungeon_commands() -> &'static Mutex<Vec<String>> {
        static  DUNGEONCOMMANDS: OnceLock<Mutex<Vec<String>>> = OnceLock::new();

        DUNGEONCOMMANDS.get_or_init( || {
            let dungeoncommands = Mutex::new(Vec::from([
                
            ]));

            dungeoncommands
        })
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

    pub fn dungeon_handler_ref() -> &'static Mutex<DungeonHandler>{

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
    player_position: i8,
    //current_room: Arc<Mutex<Dungeonroom>>,

    //Will later be changed

}


impl Dungeon {

    pub fn new( )-> Self{
        let rooms= Self::generat_generate_dungeon_rooms(0);
        let dungeon = Self{

            rooms,
            player_position: 0,
        };
        dungeon
    }
    pub fn generat_generate_dungeon_rooms(nofrooms: i8) -> Vec<Vec<Dungeonroom>> {

       vec![vec![Dungeonroom::MonsterRoom("Goblin")]]
    }


    pub fn Dungeon_run(self, )-> bool {

        true
    }

    pub fn dungeon_ref() -> &'static Arc<Mutex<Dungeon>> {
        static DUNGEON: OnceLock<Arc<Mutex<Dungeon>>>  = OnceLock::new();

        DUNGEON.get_or_init(||{
            let dungeon = Arc::new(Mutex::new(Dungeon::new()));
            dungeon
        })

    }

    pub fn get_current_room(&self) -> &Dungeonroom {
        &self.rooms[0][0]
    }


}


pub struct Dungeonroom {
    encoutner: EncounterTypes ,

}

impl Dungeonroom{


    pub fn MonsterRoom(name: &str) -> Self{

        Self{
            encoutner: EncounterTypes::Monster(Monster::new("Goblin".to_string())),
        }
    }

    pub fn display_room() -> Block<'static>{
        let room = Block::new().title("Room").borders(Borders::ALL);
        room
    }



    pub fn get_Type(&self) -> &str {
        &self.encoutner.get_Name()
    }

}


