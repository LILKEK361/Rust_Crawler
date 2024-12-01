use std::{io, thread, time::Duration};

use crossterm::event;
use ratatui::Terminal;
use terminaldrawer::tdrawer;
use std::io::{stderr, stdout, Stdout};

use crossterm::terminal;
use ratatui::{backend, layout::{Constraint, Direction, Layout}, prelude::CrosstermBackend, DefaultTerminal};
use crossterm::event::{Event};
use ratatui::{text::Text, Frame};




#[path= "Gamehelper/Gamelogic/uidrawer.rs"]
mod uidrawer;

#[path="Gamehelper/Gamelogic/terminaldrawer.rs"]
mod terminaldrawer;
#[path="Gamehelper/Gamelogic/Story.rs"]
mod story;


#[path="GameObjects.rs"]
mod gameobjects;

fn main() {
   

   let mut tdrawer = tdrawer::new();
    let terminal = ratatui::init();
    tdrawer.draw_ui(terminal);

    /*
    let player = gameobjects::player::Player::new();
    let dungeon = gameobjects::dungeon::Dungeon::new(&player);
    let _ = dungeon.Dungeon_run();
 */
}

   
