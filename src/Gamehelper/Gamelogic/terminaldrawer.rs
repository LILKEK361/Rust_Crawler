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
use crate::gameobjects::dungeon::{Dungeon, DungeonHandler, Dungeonroom};
use crate::gameobjects::encounter::Encounter;
use crate::gameobjects::item_handler::ItemsTypes;
use crate::gameobjects::player;
use crate::gameobjects::player::Player;
use crate::{add_log, gamestate_ref, read_log, Gamestate};
use ratatui::widgets::block::Position;
/*
    This file will handle the ui drawing for the game

*/

pub struct tdrawer {
    input_string: String,
    character_index: usize,
    input_mode: InputMode,
    dislay: Block<'static>,
    log_index: i8,
    show_spoiler: bool,
}

enum InputMode {
    Normal,
    Editing,
}

impl tdrawer {
    pub fn new() -> tdrawer {
        tdrawer {
            input_string: String::new(),
            input_mode: InputMode::Editing,
            character_index: 0,
            dislay: Block::default()
                .borders(Borders::ALL)
                .title("Placeholder")
                .title_position(ratatui::widgets::block::Position::Top),
            log_index: 30,
            show_spoiler: false,
        }
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input_string.insert(index, new_char);
        self.move_cursor_right();
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input_string.chars().count())
    }

    fn byte_index(&self) -> usize {
        self.input_string
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input_string.len())
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input_string.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input_string.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input_string = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    pub fn submit_message(&mut self) {
        add_log(&*format!("Player: {}", &self.input_string));

        if (self.input_string.clone() == "start"
            && *gamestate_ref().lock().unwrap() == Gamestate::home)
        {
            Player::create_new_player();
            Dungeon::generate_new_dungeon();
            *gamestate_ref().lock().unwrap() = Gamestate::run;
            /*
            DungeonHandler::dungeon_handler_ref()
                .lock()
                .unwrap()
                .send_action("map".into());
                Self::set_render_queue("map".into());*/
        } else if ((self.input_string.clone() == "exit" || self.input_string.clone() == "end")
            && *gamestate_ref().lock().unwrap() == Gamestate::run)
        {
            *gamestate_ref().lock().unwrap() = Gamestate::home;
        } else if (*gamestate_ref().lock().unwrap() == Gamestate::run) {
            /*
            DungeonHandler::dungeon_handler_ref()
                .lock()
                .unwrap()
                .send_action(self.input_string.clone());*/
        } else if (self.input_string.clone().to_ascii_lowercase().eq("spoiler")
            && !(*gamestate_ref().lock().unwrap() == Gamestate::run))
        {
            self.show_spoiler = true;
        } else if (self.input_string.clone().to_ascii_lowercase().eq("menu")
            && !(*gamestate_ref().lock().unwrap() == Gamestate::run))
        {
            self.show_spoiler = false;
        }

        self.input_string.clear();
        self.reset_cursor();
    }

    pub fn draw(&mut self, terminal: &mut DefaultTerminal) -> io::Result<&str> {
        loop {
            terminal.draw(|frame: &mut Frame| {
                &self.screen(frame);
            })?;

            if let event::Event::Key(key) = event::read()? {
                if (key.kind == KeyEventKind::Press) {
                    match key.code {
                        KeyCode::Esc => return Ok("exit"),
                        KeyCode::Enter => {
                            if (self.input_string != "") {
                                self.submit_message();
                            }
                        }
                        KeyCode::Char(to_insert) => self.enter_char(to_insert),
                        KeyCode::Backspace => self.delete_char(),
                        KeyCode::Left => self.move_cursor_left(),
                        KeyCode::Right => self.move_cursor_right(),
                        KeyCode::Down => {
                            if (read_log().len() >= (self.log_index + 1) as usize) {
                                self.log_index += 1;
                            }
                        }
                        KeyCode::Up => {
                            if (29 < (self.log_index - 1)) {
                                self.log_index -= 1;
                            }
                        }

                        _ => {}
                    }
                }
            }
        }
    }

    pub fn screen(&self, frame: &mut Frame) {
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(90), Constraint::Percentage(10)])
            .split(frame.area());

        let big_screen = layout::Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(75), Constraint::Percentage(25)])
            .split(main_layout[0]);

        let input = Paragraph::new(self.input_string.as_str());

        let input_block = Block::default()
            .borders(Borders::ALL)
            .title("Input")
            .title_position(ratatui::widgets::block::Position::Top);

        frame.render_widget(&input_block, main_layout[1]);

        frame.render_widget(input, input_block.inner(main_layout[1]));
        if (read_log().len() as i8 > 30i8) {
            frame.render_widget(
                Self::get_specific_log(
                    (read_log().len() - self.log_index as usize) as i8,
                    read_log().len() as i8,
                ),
                big_screen[1],
            )
        } else {
            frame.render_widget(Self::get_log(), big_screen[1]);
        }

        if (*gamestate_ref().lock().unwrap() == Gamestate::run) {
            let big_container = Block::default()
                .borders(Borders::ALL)
                .title("Dungeon")
                .title_position(ratatui::widgets::block::Position::Top);

            frame.render_widget(&big_container, big_screen[0]);
            tdrawer::display_dungeon_context(frame, &big_container, &big_screen[0]);
        } else {
            let big_container = Block::default()
                .borders(Borders::ALL)
                .title("Menu")
                .title_position(ratatui::widgets::block::Position::Top);
            let main_menu_text = if (self.show_spoiler) {
                Paragraph::new(konst::SPOILER)
                    .block(big_container)
                    .alignment(Center)
                    .wrap(Wrap { trim: true })
            } else {
                Paragraph::new(konst::MAINMENU)
                    .block(big_container)
                    .alignment(Center)
                    .wrap(Wrap { trim: true })
            };
            frame.render_widget(main_menu_text, big_screen[0])
        };
    }

    pub fn display_dungeon_context(frame: &mut Frame, container: &Block, area: &Rect) {
        let command = Self::render_queue().lock().unwrap();
        if command.eq("map".into()) {
            //Self::draw_map(frame, container, area);
        } else if command.eq("combat".into()) {
            Self::draw_combat(frame, container, area);
        } else if command.eq("inventory".into()) {
            // Self::draw_inventory(frame, container, area);
        } else if command.eq("look".into()) {
            //Self::draw_room(frame, container, area);
        } else if command.eq("help".into()) {
            Self::draw_help(frame, container, area);
        } else if (command.eq("info".into())) {
            Self::draw_character_sheet(frame, container, area)
        } else if (command.eq("death".into())) {
            Self::draw_death_screen(frame, container, area)
        } else if (command.eq("victory")) {
            Self::draw_victory_screen(frame, container, area)
        }
    }
    pub fn draw_help(frame: &mut Frame, container: &Block, area: &Rect) {
        frame.render_widget(Paragraph::new(konst::HELP), container.inner(*area))
    }

    pub fn draw_character_sheet(frame: &mut Frame, container: &Block, area: &Rect) {
        let player = Player::player_ref().lock().unwrap();

        let (name, health, max_health, inv_size, armor, level, skills) = player.get_stats();
        frame.render_widget(
            Paragraph::new(konst::PLAYERINFO(
                name, level, health, max_health, armor, inv_size,
            )),
            container.inner(*area),
        )
    }

    pub fn draw_combat(frame: &mut Frame, container: &Block, area: &Rect) {
        let mut dungeon = Dungeon::dungeon_ref().lock().unwrap();

        let monster = dungeon.get_current_room().get_Monster().unwrap();

        let player = player::Player::player_ref().lock().unwrap();

        let mapLayout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ])
            .split(container.inner(*area));

        let helper_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
            .split(mapLayout[1]);

        let playercard = generate_Card(
            String::from(&player.name),
            *player.get_hp(),
            *player.get_max_hp(),
        );
        let monstercard = generate_Card(
            String::from(&monster.name),
            *monster.get_hp(),
            *monster.get_max_hp(),
        );

        frame.render_widget(playercard, mapLayout[0]);

        frame.render_widget(monstercard, mapLayout[2]);
    }

    pub fn draw_death_screen(frame: &mut Frame, container: &Block, area: &Rect) {
        frame.render_widget(
            Paragraph::new(konst::DEATHMESSAGE)
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true }),
            container.inner(*area),
        )
    }

    pub fn draw_victory_screen(frame: &mut Frame, container: &Block, area: &Rect) {
        frame.render_widget(
            Paragraph::new(konst::VICOTRYMESSAGE)
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true }),
            container.inner(*area),
        )
    }

    pub fn get_log() -> List<'static> {
        let mut messages: Vec<ListItem> = read_log()
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let content = Line::from(Span::raw(format!("{m}")));
                ListItem::new(content)
            })
            .collect();
        let log_block = List::new(messages).block(Block::bordered().title("Log"));
        log_block
    }

    pub fn render_queue() -> &'static Mutex<String> {
        static QUEUE: OnceLock<Mutex<String>> = OnceLock::new();
        QUEUE.get_or_init(|| {
            let queue = Mutex::new(String::new());
            queue
        })
    }
    pub fn set_render_queue(name: String) {
        if let Ok(mut queue) = Self::render_queue().lock() {
            *queue = name
        } else {
            eprintln!("Failed to lock render");
        }
    }

    pub fn get_specific_log(start: i8, end: i8) -> List<'static> {
        let logs = read_log();

        let mut messages: Vec<ListItem> = Vec::new();

        if (logs.len() as i8 <= end) {
            for i in start..end {
                let log = logs.get(i as usize).unwrap();
                messages.push(ListItem::new(Line::from(Span::raw(format!("{log}")))));
            }
            let log_block = List::new(messages).block(Block::bordered().title("Log"));
            log_block
        } else {
            Self::get_log()
        }
    }

    pub fn tdrawer_ref() -> &'static Mutex<tdrawer> {
        static TDRAWER: OnceLock<Mutex<tdrawer>> = OnceLock::new();

        TDRAWER.get_or_init(|| {
            let tdrawer = Mutex::new(tdrawer::new());
            tdrawer
        })
    }
}

pub fn generate_Card(name: String, hp: u8, max_hp: u8) -> Paragraph<'static> {
    let card = Block::default()
        .title(name)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let info = Paragraph::new(format!("HP: {hp}/{max_hp}")).block(card);
    info
}

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
        } else {
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
            } else if(action.eq("loot")){

            } else if(action.eq("exit")){
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
