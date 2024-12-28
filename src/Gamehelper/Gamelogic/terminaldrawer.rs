use crate::gameobjects::item_handler::Item;
use std::any::Any;

use ratatui::layout::{Alignment, Direction, Rect, Rows};
use ratatui::{crossterm::event::{self, KeyCode, KeyEventKind}, layout, layout::{Constraint, Layout}, text::{Line, Span, Text}, widgets::{Block, List, ListItem, Paragraph}, DefaultTerminal, Frame};
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt::format;
use std::io::{self};
use std::ops::{Deref, DerefMut};
use std::sync::{Mutex, MutexGuard, OnceLock};
use crossterm::event::read;

use ratatui::widgets::{BorderType, Borders, Row, Table, Cell, Wrap};
use log::log;
use ratatui::layout::Alignment::Center;
use ratatui::layout::Constraint::Ratio;
use ratatui::layout::Direction::{Horizontal, Vertical};
use ratatui::prelude::Stylize;
use ratatui::style::{Color, Style};

use ratatui::widgets::block::Position;
use crate::gameobjects::dungeon::{Dungeon, DungeonHandler, Dungeonroom};
use crate::{add_log, gamestate_ref, read_log, Gamestate};

use crate::gameobjects::player;
use crate::gameobjects::player::Player;
use crate::gamelogic::konst;
/*
    This file will handle the ui drawing for the game

*/


pub struct tdrawer {


    input_string: String,
    character_index:usize,
    input_mode: InputMode,
    dislay: Block<'static>,
    log_index: i8,






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
            dislay: Block::default().borders(Borders::ALL).title("Placeholder").title_position(ratatui::widgets::block::Position::Top),
            log_index: 30,


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

        if(self.input_string.clone() == "start" && *gamestate_ref().lock().unwrap() == Gamestate::home){
            *gamestate_ref().lock().unwrap() = Gamestate::run;
            DungeonHandler::dungeon_handler_ref().lock().unwrap().send_action("map".into());
        }else if((self.input_string.clone() == "exit" || self.input_string.clone() == "end") && *gamestate_ref().lock().unwrap() == Gamestate::run){
            *gamestate_ref().lock().unwrap() = Gamestate::home;
        }
        else if(*gamestate_ref().lock().unwrap() == Gamestate::run){
            DungeonHandler::dungeon_handler_ref().lock().unwrap().send_action(self.input_string.clone());
        }


        self.input_string.clear();
        self.reset_cursor();
    }



    pub fn draw(&mut self, terminal: &mut DefaultTerminal) -> io::Result<&str> {
        loop{

            terminal.draw( |frame: &mut Frame| {

                &self.screen(frame);


            })?;
            
            if let event::Event::Key(key) = event::read()? {
               if (key.kind == KeyEventKind::Press) {
                    match key.code {
                        KeyCode::Esc => return Ok("exit"),
                        KeyCode::Enter => {
                            if(self.input_string != "") {
                                self.submit_message();
                            }
                        },
                        KeyCode::Char(to_insert) => self.enter_char(to_insert),
                        KeyCode::Backspace => self.delete_char(),
                        KeyCode::Left => self.move_cursor_left(),
                        KeyCode::Right => self.move_cursor_right(),
                        KeyCode::Down => {
                            if(read_log().len() >= (self.log_index + 1) as usize) {
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
            .constraints(
                [
                    Constraint::Percentage(90),
                    Constraint::Percentage(10),
                ]
                    .as_ref(),
            )
            .split(frame.area());


        let big_screen = layout::Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(75),
                    Constraint::Percentage(25),
                ]
                    .as_ref(),
            )
            .split(main_layout[0]);

        let input = Paragraph::new(self.input_string.as_str());

        let input_block = Block::default().borders(Borders::ALL).title("Input").title_position(ratatui::widgets::block::Position::Top);


        frame.render_widget(&input_block, main_layout[1]);
        frame.render_widget(input, input_block.inner(main_layout[1]));
        if(read_log().len() as i8 > 30i8){
            frame.render_widget(Self::get_specific_log((read_log().len() - self.log_index as usize) as i8, read_log().len() as i8), big_screen[1])
        }else {
            frame.render_widget(Self::get_log(  ),big_screen[1]);
        }

        if(*gamestate_ref().lock().unwrap() == Gamestate::run){

            let big_container = Block::default().borders(Borders::ALL).title("Dungeon").title_position(ratatui::widgets::block::Position::Top);

            frame.render_widget(&big_container, big_screen[0]);
            tdrawer::display_dungeon_context(frame, &big_container ,&big_screen[0]);

        } else {
            let big_container = Block::default().borders(Borders::ALL).title("Menu").title_position(ratatui::widgets::block::Position::Top);
            frame.render_widget(big_container, big_screen[0])
        };

    }

    pub fn display_dungeon_context(frame: &mut Frame,container: &Block, area: &Rect) {

        let command= Self::render_queue().lock().unwrap();
        if command.eq("map".into()) {

            Self::draw_map(frame, container, area);
        } else if command.eq("combat".into()){
            Self::draw_combat(frame, container,area);

        }else if command.eq("inventory".into()){
            Self::draw_inventory(frame, container, area);
        }else if command.eq("look".into()){
            Self::draw_room(frame, container, area);
        }else if command.eq("help".into()){
            Self::draw_help(frame, container, area);
        }else if(command.eq("info".into())){
            Self::draw_character_sheet(frame, container, area)
        }
    }
    pub fn draw_help(frame: &mut Frame, container: &Block, area: &Rect) {


        frame.render_widget(Paragraph::new(konst::HELP), container.inner(*area))

    }
    pub fn draw_room(frame: &mut Frame, container: &Block, area: &Rect){
        let mut dungeon = Dungeon::dungeon_ref().lock().unwrap();
        let current_room = dungeon.get_current_room();


        let fulllayout = Layout::default()
            .direction(Horizontal)
            .constraints([Constraint::Ratio(1,3),Constraint::Ratio(1,3),Constraint::Ratio(1,3)])
            .split(container.inner(*area));
        let roomlayout = Layout::default()
            .direction(Vertical)
            .constraints([Constraint::Ratio(1,3),Constraint::Ratio(1,3),Constraint::Ratio(1,3)])
            .split(fulllayout[1]);

        let paragraphlayout = Layout::default()
            .direction(Vertical)
            .constraints([Constraint::Percentage(30),Constraint::Percentage(70)])
            .split(roomlayout[0]);
        let notelayout = Layout::default()
            .direction(Vertical)
            .constraints([Constraint::Percentage(30),Constraint::Percentage(70)])
            .split(roomlayout[2]);


        let test_room = Block::default()
            .borders(Borders::ALL)
            .title(current_room.get_room_title());

        let des = Paragraph::new(current_room.get_des())
            .wrap(Wrap {trim : true})
            .alignment(Alignment::Center);

        let note = Paragraph::new(format!("Here are notes:\n{}", current_room.get_note()))
            .wrap(Wrap {trim : true})
            .alignment(Alignment::Center);;

        frame.render_widget(test_room, roomlayout[1]);
        frame.render_widget(des, paragraphlayout[1]);
        frame.render_widget(note, notelayout[1]);

    }

    pub fn draw_map(frame: &mut Frame, container: &Block, area: &Rect){
        let dungeon = Dungeon::dungeon_ref().lock().unwrap();
        let dungeonrooms = dungeon.get_all_rooms();
        let pp = dungeon.get_player_position();

        let mapLayout = Layout::default()
            .direction(Direction::Vertical)
            .constraints((&dungeonrooms).iter().map(|_| { Constraint::Ratio(1, dungeonrooms.len() as u32)}))
            .split(container.inner(*area));

        let rows = dungeonrooms.len();

        for i in 0..rows {
            let dungeonroomrow = &dungeonrooms[i];
            let row_size = &dungeonrooms[i].len();
            let row_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(dungeonrooms[i].iter().map(|_|{Constraint::Ratio(1, dungeonroomrow.len() as u32)}))
                .split(mapLayout[i]);


            for j in 0..*row_size {
                let mut roomtitle = dungeonroomrow[j].get_room_title();
                if(i == pp[0] as usize && j == pp[1] as usize){
                    frame.render_widget(Block::default()
                                            .title(String::from(format!("{}: {}", "\\@/", roomtitle)))
                                            .borders(Borders::ALL)
                                            .red(),
                                        row_layout[j])
                }
                else if(!dungeonroomrow[j].get_Type().eq("None")){

                   if(dungeonroomrow[j].get_Type().eq("Goal")){
                       frame.render_widget(Block::default()
                                               .title(roomtitle)
                                               .borders(Borders::ALL)
                                               .red(), row_layout[j])
                   } else {

                       frame.render_widget(Block::default()
                                               .title(roomtitle)
                                               .borders(Borders::ALL), row_layout[j])
                   }
                }

            }

        }
    }

    pub fn draw_character_sheet (frame: &mut Frame, container: &Block, area: &Rect) {
        let player = Player::player_ref().lock().unwrap();


        let (name, health, max_health,inv_size, armor, level, skills ) = player.get_stats();
        frame.render_widget(Paragraph::new(konst::PLAYERINFO(name, level, health, max_health, armor, inv_size)), container.inner(*area))

    }

    pub fn draw_combat(frame: &mut Frame, container: &Block, area: &Rect){

        let mut dungeon = Dungeon::dungeon_ref().lock().unwrap();

        let monster = dungeon.get_current_room().get_Monster().unwrap();

        let player =  player::Player::player_ref().lock().unwrap();

        let mapLayout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(25),Constraint::Percentage(50),Constraint::Percentage(25),])
            .split(container.inner(*area));
        let helper_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
            .split(mapLayout[1]);

        let playercard = generate_Card(String::from(&player.name), *player.get_hp() as i8, *player.get_max_hp());
        let monstercard = generate_Card(String::from(&monster.name), *monster.get_hp() as i8, *monster.get_max_hp());
        let help = Paragraph::new(konst::COMBATHELPERMENU).block(Block::new().borders(Borders::ALL).title("Combat Basic Commands"));
        frame.render_widget(playercard, mapLayout[0]);
        frame.render_widget(help, helper_layout[1]);
        frame.render_widget(monstercard, mapLayout[2]);
    }
    fn draw_inventory(frame: &mut Frame, container: &Block, area: &Rect) {
        let player = Player::player_ref().lock().unwrap();

        let inventory = player.get_inventory();

        let inventory_layout = Layout::default()
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
            .direction(Horizontal)
            .split(container.inner(*area));

        let display_layout = Layout::default()
            .direction(Vertical)
            .constraints([Constraint::Percentage(70),Constraint::Percentage(30)])
            .split(inventory_layout[0]);





        let items_layout = Layout::default()
            .constraints((0..(inventory.len() + 1)).map(|_| {
                Constraint::Percentage((100 /(inventory.len() + 1)) as u16)
            }).collect::<Vec<Constraint>>())
            .direction(Vertical)
            .split(display_layout[0]);

        for i in 0..(inventory.len() + 1) {

            let row_layout = Layout::default().direction(Horizontal).constraints([Constraint::Ratio(1,4); 4]).split(items_layout[i]);

            if( i == 0) {

                frame.render_widget(Paragraph::new("Index"), row_layout[0]);
                frame.render_widget(Paragraph::new("Name"), row_layout[1]);

                frame.render_widget(Paragraph::new("Rarity"), row_layout[2]);
                frame.render_widget(Paragraph::new("Value"), row_layout[3]);

            }else  {

                frame.render_widget(Paragraph::new((i - 1).to_string()), row_layout[0]);
                frame.render_widget(Paragraph::new(inventory.get(i - 1).unwrap().get_name()), row_layout[1]);
                frame.render_widget(Paragraph::new(inventory.get(i - 1).unwrap().get_rarity().to_string()), row_layout[2]);
                frame.render_widget(Paragraph::new(inventory.get(i - 1).unwrap().get_value().to_string()), row_layout[3]);
            }

        }

        let help = Paragraph::new(
            Text::from(konst::INVENTORYHELP.split("\n").map(|txt|{
            Line::from(Span::from(txt))
        }).collect::<Vec<Line>>())).block(Block::new().title("Inventory Help").borders(Borders::ALL));

        let equipment_display = Block::new().borders(Borders::ALL);

        let mut equipment_layout = Layout::default()
            .constraints([Constraint::Percentage(25),Constraint::Percentage(25),Constraint::Percentage(25),Constraint::Percentage(25)])
            .direction(Vertical)
            .split(equipment_display.inner(inventory_layout[1]));

        if(player.get_inspect().0) {

            let itemdes = inventory.get(player.get_inspect().1 as usize).unwrap().get_des().split("\\").map(|line| {
               Line::from(Span::from(line))
            }).collect::<Vec<Line>>();



            frame.render_widget(equipment_display.title(format!("Item: {}",inventory.get(player.get_inspect().1 as usize).unwrap().get_name())), inventory_layout[1]);
            frame.render_widget(Paragraph::new(Text::from(itemdes)).wrap(Wrap {trim: true}),equipment_layout[1])


        }else {

            let equipment_list = List::new(vec![

                ListItem::from(Line::from(Span::from(format!("Head: {}", player.get_equipment_from_slot("head".into()).get_name())))),
                ListItem::from(Line::from(Span::from(format!("Torso: {}", player.get_equipment_from_slot("Torso".into()).get_name())))),
                ListItem::from(Line::from(Span::from(format!("Hands: {}", player.get_equipment_from_slot("Hands".into()).get_name())))),
                ListItem::from(Line::from(Span::from(format!("Pants: {}", player.get_equipment_from_slot("Pants".into()).get_name())))),
                ListItem::from(Line::from(Span::from(format!("Shoes: {}", player.get_equipment_from_slot("Shoes".into()).get_name())))),


            ]);

            frame.render_widget(equipment_list, equipment_display.inner(inventory_layout[1]));

            frame.render_widget( equipment_display.title("Equipment").title_position(Position::Top), inventory_layout[1]);
        }





        frame.render_widget(help, display_layout[1]);

    }

    pub fn get_log() -> List<'static>{


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
        QUEUE.get_or_init(||{
            let queue = Mutex::new(String::new());
            queue
        })
    }
    pub fn set_render_queue(name: String){
        if let Ok(mut queue) = Self::render_queue().lock(){
            *queue = name
        }else {
            eprintln!("Failed to lock render");
        }
    }

    pub fn get_specific_log(start: i8, end: i8) -> List<'static>{

        let logs = read_log();

        let mut messages: Vec<ListItem> = Vec::new();

        if (logs.len() as i8 >= end ){
            for i in start..end  {

                let log = logs.get(i as usize).unwrap();
                messages.push(ListItem::new(Line::from(Span::raw(format!("{log}")))));
            }
            let log_block = List::new(messages).block(Block::bordered().title("Log"));
            log_block
        } else {
            Self::get_log()
        }
    }



    pub fn tdrawer_ref() -> &'static Mutex<tdrawer>{
        static TDRAWER: OnceLock<Mutex<tdrawer>> = OnceLock::new();

        TDRAWER.get_or_init(||{

            let tdrawer = Mutex::new(tdrawer::new());
            tdrawer
        })
    }


}

pub  fn generate_Card(name: String, hp: i8, max_hp: i8) -> Paragraph<'static>{
    let card = Block::default()
        .title(name)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let info = Paragraph::new(format!("HP: {hp}/{max_hp}"))
        .block(card);
    info
}





