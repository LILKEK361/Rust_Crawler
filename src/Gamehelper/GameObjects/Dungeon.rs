use crate::gameobjects::monster_handler::{Monster};
use std::any::Any;
use std::collections::VecDeque;
use std::io::Read;
use std::ops::{Deref, DerefMut};
use std::sync::{mpsc, Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;
use std::time::Duration;
use ratatui::DefaultTerminal;
use crate::gameobjects::encounter::Encounter;

use crate::gameobjects::player::Player;
use crate::log_ref;
use crate::terminaldrawer::tdrawer;



pub struct DungeonHandler{

        tx: Sender<()>,

        action_queue: Arc<Mutex<VecDeque<String>>>



}

impl DungeonHandler{
    pub fn new() -> Self {
        
        let (tx, rx) = mpsc::channel();

        //Queue for storing the actions
        let action_queue = Arc::new(Mutex::new(VecDeque::<String>::new()));

        let action_queue_clone = Arc::clone(&action_queue);

        let handle = std::thread::spawn(move || {
            loop {

                //Check if there is any action in the queue
                rx.recv().unwrap();
                log_ref().lock().unwrap().deref_mut().push("Recived action".into());

            }
        });

        drop(handle);

        Self {
            tx,
            action_queue
        }
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


}


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


    pub fn Dungeon_run(self, tdrawer: &mut tdrawer)-> bool {




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


