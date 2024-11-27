use std::{io, thread, time::Duration};

use crossterm::event;
use ratatui::Terminal;
use terminaldrawer::tdrawer;
use std::io::{stderr, stdout, Stdout};

use crossterm::terminal;
use ratatui::{backend, layout::{Constraint, Direction, Layout}, prelude::CrosstermBackend, DefaultTerminal};
use crossterm::event::{Event};
use ratatui::{text::Text, Frame};



#[path ="./Gamehelper/player.rs"]
mod player;
#[path= "Gamehelper/Gamelogic/uidrawer.rs"]
mod uidrawer;
#[path= "Gamehelper/GameObjects/item_handler.rs"]
mod item_handler;
#[path= "Gamehelper/GameObjects/Items/weaponitem.rs"]
mod weaponitem;
#[path= "Gamehelper/GameObjects/Items/equip_item.rs"]
mod equip_item;
#[path="Gamehelper/Gamelogic/terminaldrawer.rs"]
mod terminaldrawer;
#[path="Gamehelper/Gamelogic/Story.rs"]
mod story;

fn main() {
   

   let mut tdrawer = tdrawer::new();
    tdrawer.draw_ui();
 
}
   
