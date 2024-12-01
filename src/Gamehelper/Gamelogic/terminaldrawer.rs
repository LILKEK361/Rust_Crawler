use std::io::{self, stderr, stdout, Stdout};


use ratatui::{crossterm::event::{self, Event, KeyCode, KeyEventKind}, layout, layout::{Constraint, Layout, Position}, style::{Color, Modifier, Style, Stylize}, text::{Line, Span, Text}, widgets::{Block, List, ListItem, Paragraph}, DefaultTerminal, Frame, Terminal};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::Direction;
use ratatui::widgets::Borders;
use log::log;
/*
    This file will handle the ui drawing for the game

*/
pub struct tdrawer {


    input_string: String,
    character_index:usize,
    input_mode: InputMode,
    commands: Vec<String>,
}



enum InputMode {
    Normal,
    Editing,
}

struct home_screen {}

impl tdrawer {


    pub fn new() -> tdrawer {

        tdrawer {

            input_string: String::new(),
            input_mode: InputMode::Editing,
            commands: Vec::new(),
            character_index: 0,
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

    fn submit_message(&mut self) {
        self.commands.push(self.input_string.clone());
        self.input_string.clear();
        self.reset_cursor();
    }

    pub fn draw_ui(mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
        loop{
            
            
            terminal.draw( |frame: &mut Frame| {
                self.draw_home(frame);
                ()
            })?;
            
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    return Ok(());
                } else if (key.kind == KeyEventKind::Press) {
                    match key.code {
                        KeyCode::Enter => self.submit_message(),
                        KeyCode::Char(to_insert) => self.enter_char(to_insert),
                        KeyCode::Backspace => self.delete_char(),
                        KeyCode::Left => self.move_cursor_left(),
                        KeyCode::Right => self.move_cursor_right(),
                        KeyCode::Esc => self.input_mode = InputMode::Normal,
                        _ => {}
                    }
                }
            
            }
        }
    }
    
    pub fn draw_home(&self,frame: &mut Frame) {
        let  main_layout = Layout::default()
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
        
        
        let menu_block = Block::default().borders(Borders::ALL).title("Main Menu").title_position(ratatui::widgets::block::Position::Top);
        let input_block = Block::default().borders(Borders::ALL).title("Input").title_position(ratatui::widgets::block::Position::Top);
        
        let t = Text::raw(crate::story::MAINMENU);
        let input = Paragraph::new(self.input_string.as_str());

        let messages: Vec<ListItem> = self
            .commands
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let content = Line::from(Span::raw(format!("{i}: {m}")));
                ListItem::new(content)
            })
            .collect();


        let log_block = List::new(messages).block(Block::bordered().title("Command Log"));


        frame.render_widget(&menu_block, big_screen[0]);

        frame.render_widget(&input_block, main_layout[1]);
        frame.render_widget(t, menu_block.inner(big_screen[0]));
        frame.render_widget(input, input_block.inner(main_layout[1]));
        frame.render_widget(log_block, big_screen[1]);

    }
    
    pub fn draw(frame: &mut Frame) {
        let text = Text::raw("Hello World!");
        frame.render_widget(text, frame.area());
    }

}




