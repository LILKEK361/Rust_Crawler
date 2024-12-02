use std::{io, thread, time::Duration};

use crossterm::event;
use ratatui::Terminal;
use terminaldrawer::tdrawer;
use std::io::{stderr, stdout, Stdout};

use crossterm::terminal;
use ratatui::{backend, layout::{Constraint, Direction, Layout}, prelude::CrosstermBackend, DefaultTerminal};
use crossterm::event::{Event};
use ratatui::{text::Text, Frame};


use std::{sync::{Mutex, OnceLock}, thread::{sleep, Thread}};
use std::ops::Deref;
use std::sync::Arc;
use crate::gameobjects::dungeon::{Dungeon, DungeonHandler};

#[path= "Gamehelper/Gamelogic/uidrawer.rs"]
mod uidrawer;

#[path="Gamehelper/Gamelogic/terminaldrawer.rs"]
mod terminaldrawer;
#[path="Gamehelper/Gamelogic/Story.rs"]
mod story;


#[path="GameObjects.rs"]
mod gameobjects;

pub fn log_ref() -> &'static Mutex<Vec<String>>{
    static LOG: OnceLock<Arc<Mutex<Vec<String>>>> = OnceLock::new();

    LOG.get_or_init(|| {
        Arc::new(Mutex::new(Vec::from(["First message".into()])))
    })
}



fn main() {
   

    let mut terminal = ratatui::init();

    let mut tdrawer = tdrawer::new();
    tdrawer.draw(&mut terminal).unwrap();

    


}

pub enum Gamestate {
    run,
    home,
    end,
}

pub fn tdrawer_ref() -> &'static Mutex<tdrawer>{
    static TDRAWER: OnceLock<Mutex<tdrawer>> = OnceLock::new();

    TDRAWER.get_or_init(||{

        let tdrawer = Mutex::new(tdrawer::new());
        tdrawer
    })
}

pub fn dungeon_ref() -> &'static Mutex<DungeonHandler>{
    static DUNGEON: OnceLock<Mutex<DungeonHandler>> = OnceLock::new();

    DUNGEON.get_or_init(||{
        let dungeon = Mutex::new(DungeonHandler::new());
        dungeon
    })
}

pub fn gamestate_ref() -> &'static Mutex<Gamestate>{
    static GAMESTATE: OnceLock<Mutex<Gamestate>> = OnceLock::new();

    GAMESTATE.get_or_init(||{
        let gamestate = Mutex::new(Gamestate::home);
        gamestate
    })
}

