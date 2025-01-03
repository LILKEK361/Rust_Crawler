use std::{io, thread, time::Duration};

use crossterm::{event, execute,terminal::{self, SetSize}};
use ratatui::Terminal;
use gamelogic::{terminaldrawer::tdrawer, postgreshandler};
use std::io::{stderr, stdout, Stdout};


use ratatui::{backend, layout::{Constraint, Direction, Layout}, prelude::CrosstermBackend, DefaultTerminal};

use ratatui::{text::Text, Frame};


use std::{sync::{Mutex, OnceLock}, thread::{sleep, Thread}};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use crossterm::terminal::EnterAlternateScreen;
use ratatui::widgets::Block;
use crate::gamehelper::dbpaths;
use crate::gamelogic::gamehelperfunctions::generate_random_weapon;
use crate::gameobjects::{dungeon::{Dungeon, DungeonHandler}};
use crate::gameobjects::item_handler::Item;

#[path= "Gamelogic.rs"]
mod gamelogic;


#[path="GameObjects.rs"]
mod gameobjects;

#[path="Gamehelper.rs"]
mod gamehelper;

#[path="Gamehelper/tests/test.rs"]
mod test;




pub fn log_ref() -> &'static Mutex<Vec<String>>{
    static LOG: OnceLock<Arc<Mutex<Vec<String>>>> = OnceLock::new();

    LOG.get_or_init(|| {
        Arc::new(Mutex::new(Vec::from([String::from("WELCOME")])))
    })
}

pub fn add_log(message: &str){
    log_ref().lock().unwrap().push(message.into());
}

pub fn read_log() -> Vec<String>{
    log_ref().lock().unwrap().clone()
}


fn main() {


    execute!(
        stdout(),
        SetSize(120, 40) // width, height
    );

    let mut terminal = ratatui::init();


    tdrawer::tdrawer_ref().lock().unwrap().deref_mut().draw(&mut terminal).unwrap();




}

pub enum Gamestate {
    run,
    home,
    end,
}

impl PartialEq for Gamestate {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Gamestate::run, Gamestate::run) => true,
            (Gamestate::home, Gamestate::home) => true,
            (Gamestate::end, Gamestate::end) => true,
            _ => false,
        }
    }
}


pub(crate) fn db_manager_ref() -> &'static Arc<Mutex<postgreshandler::PgHandler>>{
    static MANAGER: OnceLock<Arc<Mutex<postgreshandler::PgHandler>>> = OnceLock::new();

    MANAGER.get_or_init(||
        {
            let manager = Arc::new(Mutex::new( postgreshandler::PgHandler::new()));
            manager
        }
    )

}



pub fn gamestate_ref() -> &'static Mutex<Gamestate>{
    static GAMESTATE: OnceLock<Mutex<Gamestate>> = OnceLock::new();

    GAMESTATE.get_or_init(||{
        let gamestate = Mutex::new(Gamestate::home);
        gamestate
    })
}

