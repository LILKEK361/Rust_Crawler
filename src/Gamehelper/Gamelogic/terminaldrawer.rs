use ratatui::layout::{Direction, Rect};
use ratatui::{crossterm::event::{self, KeyCode, KeyEventKind}, layout, layout::{Constraint, Layout}, text::{Line, Span, Text}, widgets::{Block, List, ListItem, Paragraph}, DefaultTerminal, Frame};
use std::cmp::PartialEq;
use std::io::{self};
use std::sync::{Mutex, OnceLock};

use ratatui::widgets::Borders;

use crate::gameobjects::dungeon::{Dungeon, DungeonHandler};
use crate::{add_log, gamestate_ref, read_log, Gamestate};
/*
    This file will handle the ui drawing for the game

*/
pub struct tdrawer {


    input_string: String,
    character_index:usize,
    input_mode: InputMode,
    dislay: Block<'static>,




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
        add_log(&self.input_string);

        if(self.input_string.clone() == "start" && *gamestate_ref().lock().unwrap() == Gamestate::home){
            *gamestate_ref().lock().unwrap() = Gamestate::run;
            DungeonHandler::dungeon_handler_ref().lock().unwrap().send_action("start".into());
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

                self.screen(frame);

            })?;
            
            if let event::Event::Key(key) = event::read()? {
               if (key.kind == KeyEventKind::Press) {
                    match key.code {
                        KeyCode::Esc => return Ok("exit"),
                        KeyCode::Enter => {
                            self.submit_message();

                        },
                        KeyCode::Char(to_insert) => self.enter_char(to_insert),
                        KeyCode::Backspace => self.delete_char(),
                        KeyCode::Left => self.move_cursor_left(),
                        KeyCode::Right => self.move_cursor_right(),

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
                    Constraint::Percentage(80),
                    Constraint::Percentage(20),
                ]
                    .as_ref(),
            )
            .split(main_layout[0]);





        let t = Text::raw(crate::story::MAINMENU);
        let input = Paragraph::new(self.input_string.as_str());


        let input_block = Block::default().borders(Borders::ALL).title("Input").title_position(ratatui::widgets::block::Position::Top);


        frame.render_widget(&input_block, main_layout[1]);
        frame.render_widget(input, input_block.inner(main_layout[1]));
        frame.render_widget(Self::get_log(), big_screen[1]);

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


        let command = Self::render_queue().lock().unwrap();
        if command.eq(&String::from("map")) {
            Self::draw_dungeon(frame,container,area);
        }





    }

    pub fn draw_dungeon(frame: &mut Frame,container: &Block, area: &Rect){
        let dungeon = Dungeon::dungeon_ref().lock().unwrap();
        let dungeonrooms = dungeon.get_all_rooms();

        let mapLayout = Layout::default()
            .direction(Direction::Vertical)
            .constraints((&dungeonrooms).iter().map(|_| { Constraint::Percentage(10) }))
            .split(container.inner(*area));

        let rows = dungeonrooms.len();

        for i in 0..rows {
            let dungeonroomrow = &dungeonrooms[i];
            let row_size = &dungeonrooms[i].len();
            let row_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(dungeonrooms[i].iter().map(|_|{Constraint::Percentage(10)}))
                .split(mapLayout[i]);


            for j in 0..*row_size {
                frame.render_widget(Block::default().title(dungeonroomrow[j].display_room()[0]).borders(Borders::ALL),row_layout[j])
            }

        }
    }

    pub fn get_log() ->  List<'static>{
        let messages: Vec<ListItem> = read_log()
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let content = Line::from(Span::raw(format!("{i}: {m}")));
                ListItem::new(content)
            })
            .collect();


        let log_block = List::new(messages).block(Block::bordered().title("Command Log"));
        log_block
    }

    pub fn render_queue() -> &'static Mutex<String> {
        static QUEUE: OnceLock<Mutex<String>> = OnceLock::new();
        QUEUE.get_or_init(||{
            let queue = Mutex::new(String::new());
            queue
        })
    }

    pub fn set_render_queue(to_display: String){
        if let Ok(mut queue) = Self::render_queue().lock(){
            *queue = to_display
        }else {
            eprintln!("Failed to lock render");
        }
    }








}




