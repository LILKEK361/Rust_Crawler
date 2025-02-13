use crate::gameobjects::item_handler::{Equipmintslots, Item};
use std::any::Any;

use crossterm::event::KeyEventKind::Press;
use crossterm::event::{read, Event};
use crossterm::style::Stylize;
use log::log;
use ratatui::layout::Alignment::Center;
use ratatui::layout::Constraint::Ratio;
use ratatui::layout::Direction::{Horizontal, Vertical};
use ratatui::layout::{Alignment, Direction, Rect, Rows};
use ratatui::prelude::Stylize as OtherStylize;
use ratatui::style::{Color, Style};
use ratatui::widgets::{BorderType, Borders, Cell, Row, Table, Wrap};
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    layout,
    layout::{Constraint, Layout},
    text::{Line, Span, Text},
    widgets::{Block, List, ListItem, Paragraph},
    DefaultTerminal, Frame, TerminalOptions,
};
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt::format;
use std::io::{self};
use std::ops::{Deref, DerefMut};
use std::ptr::null;
use std::sync::{Arc, Mutex, MutexGuard, OnceLock};

use crate::gamelogic::game_screens::{Drawable, MainScreen, WindowContents};
use crate::gamelogic::konst;
use crate::gameobjects::dungeon::{Dungeon, Dungeonroom};
use crate::gameobjects::encounter::Encounter;
use crate::gameobjects::item_handler::ItemsTypes;
use crate::gameobjects::player;
use crate::gameobjects::player::Player;
use crate::{add_log, gamestate_ref, read_log, Gamestate};
use ratatui::widgets::block::Position;

pub struct drawer {
    input_string: String,
    char_start_index: usize,
    character_index: usize,
    terminal: DefaultTerminal,

    home: bool,
    game: bool,
}

impl drawer {
    pub fn new() -> Self {
        Self {
            input_string: String::new(),

            character_index: 0,
            char_start_index: 0,
            terminal: ratatui::init(),

            home: true,
            game: false,
        }
    }

    fn add_message(&mut self) {
        let cmd = &self.input_string.clone();
        self.input_string = String::from("");
        self.character_index = self.char_start_index;
        add_log(cmd);

        if ((cmd).eq(&"start") && self.home) {
            self.home = false;
            self.game = true;
            &self.start_game();
        } else if(cmd).eq(&"spoiler") && self.home {
          let _ = self.draw(WindowContents::new_spoiler());
        }else if(cmd).eq(&"menu") && self.home {
            let _ = self.draw(MainScreen::new());
        }

        else {
            self.handle_input(cmd);
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input_string.chars().count())
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    fn delete_char(&mut self) {
        if (self.input_string.len() > 0) {
            let _ = self.input_string.split_off(self.input_string.len() - 1);
        }
    }

    fn add_char(&mut self, add_char: char) {
        self.input_string.push(add_char)
    }

    pub fn draw<T: Drawable>(&mut self, screen: T) -> io::Result<&str> {
        let _ = self.terminal.clear();
        loop {
            self.terminal
                .draw(|frame: &mut Frame| screen.draw(frame, &self.input_string, read_log()))?;

            if let Ok(Event::Key(key)) = event::read() {
                if (key.kind == Press) {
                    match key.code {
                        KeyCode::Enter => self.add_message(),
                        KeyCode::Char(char) => self.add_char(char),
                        KeyCode::Esc => std::process::exit(0),
                        KeyCode::Backspace => self.delete_char(),
                        _ => {}
                    }
                }
            }
        }
    }

    pub fn start_game(&mut self) {
        Player::create_new_player();
        Dungeon::generate_new_dungeon();
        let _ = self.draw(WindowContents::new_map_screen());
    }

    pub fn drawer_static_ref() -> &'static Mutex<drawer> {
        static DRAWER: OnceLock<Mutex<drawer>> = OnceLock::new();

        DRAWER.get_or_init(|| {
            let drawer = Mutex::new(drawer::new());
            drawer
        })
    }

    pub fn handle_input(&mut self, action: &str) {
        Player::player_ref().lock().unwrap().stop_inspect();

        if *(Dungeon::dungeon_ref().lock().unwrap().is_combat()) {
            if (vec!["defend", "attack", "help"].contains(&action)) {
                if (action.eq("attack")) {
                    let mut dungeon = Dungeon::dungeon_ref().lock().unwrap();

                    let monster = dungeon.get_current_room().get_Monster().unwrap();

                    monster.take_dmg(Player::player_ref().lock().unwrap().attack());

                    add_log(&*format!(
                        "The {} took {} dmg",
                        &monster.get_Name(),
                        Player::player_ref().lock().unwrap().attack()
                    ));

                    if (!monster.is_alive()) {
                        dungeon.set_combat(false);

                        if (dungeon
                            .get_current_room()
                            .get_Type()
                            .to_ascii_lowercase()
                            .eq("goal"))
                        {
                            drop(dungeon);
                            let _ = self.draw(WindowContents::new_vic_screen());
                        } else {
                            dungeon.get_current_room().clearMonsterRoom();
                            drop(dungeon);
                            let _ = self.draw(WindowContents::new_room_screen());
                        }
                    } else {
                        Player::player_ref()
                            .lock()
                            .unwrap()
                            .take_dmg(*monster.get_dmg());

                        if (!Player::player_ref().lock().unwrap().alive) {
                            let _ = self.draw(WindowContents::new_death_screen());
                        }
                    };
                }
                if (action.eq("defend")) {
                    let mut dungeon = Dungeon::dungeon_ref().lock().unwrap();

                    let monster = dungeon.get_current_room().get_Monster().unwrap();

                    let mut player = Player::player_ref().lock().unwrap();

                    player.defend(*monster.get_dmg() as i8);

                    if (!player.alive) {
                        drop(dungeon);
                        let _ = self.draw(WindowContents::new_death_screen());
                    }
                };
                if (action.eq("help")) {
                    let _ = self.draw(WindowContents::new_help_screen());
                }
            } else {
                add_log("Dungeon: You can't use that action during")
            }
        } else {
            if (action.eq("la") || action.eq("look around")) {
                let _ = self.draw(WindowContents::new_room_screen());
            } else if (action.eq("map")) {
                let _ = self.draw(WindowContents::new_map_screen());
            } else if (action.eq("help")) {
                let _ = self.draw(WindowContents::new_help_screen());
            } else if (action.eq("clear")) {
                add_log("Todo: Cleared")
            } else if (vec!["up", "down", "left", "right"].contains(&&**&action)) {
                Dungeon::dungeon_ref().lock().unwrap().move_player(&*action);
                if (*Dungeon::dungeon_ref().lock().unwrap().is_combat()) {
                    let _ = self.draw(WindowContents::new_combat_screen());
                }
            } else if (action.eq("info")) {
                let _ = self.draw(WindowContents::new_info_screen());
            } else if action.eq("i") || action.eq("inventory") {
                let _ = self.draw(WindowContents::new_inventory_screen());
            } else if (drawer::inventory_input(&action)) {
                if (action.contains("drop")) {
                    Player::player_ref().lock().unwrap().handle_drop(&action);
                } else if (action.contains("equip") && !action.contains("unequip")) {
                    Player::player_ref().lock().unwrap().handle_equip(&action)
                } else if (action.contains("unequip")) {
                    Player::player_ref().lock().unwrap().handle_unequip(&action)
                } else if (action.contains("inspect")) {
                    Player::player_ref().lock().unwrap().handle_inspect(&action)
                }
            } else if (action.eq("loot")) {

                let mut dungeon = Dungeon::dungeon_ref().lock().unwrap();
                let _ = dungeon.get_current_room().handleLoot();

            } else if (action.eq("exit")) {
                self.game = false;
                self.home = true;
                let _ = self.draw(MainScreen::new());
            } else {
                add_log("Dungeon: You can't use that action".into())
            }
        }
    }

    pub fn inventory_input(action: &&str) -> (bool) {
        let inventory_cmd = vec!["drop", "equip", "inspect", "unequip"];
        for cmd in inventory_cmd {
            if (action.contains(cmd)) {
                return true;
            }
        }

        false
    }
}
