use std::io::{self, stderr, stdout, Stdout};

use crossterm::{event::{KeyCode, KeyEventKind}, terminal};
use ratatui::{backend, layout::{self, Constraint, Direction, Layout}, prelude::CrosstermBackend, widgets::{Block, Borders}, DefaultTerminal, Terminal};
use crossterm::event::{self, Event};
use ratatui::{text::Text, Frame};

pub struct tdrawer {

    terminal: Terminal<CrosstermBackend<Stdout>>,    

}

struct home_screen {}

impl tdrawer {


    pub fn new() -> tdrawer {
        let terminal = ratatui::init();
        tdrawer {
            terminal,
        }
    }

    pub fn draw_ui(&mut self) -> io::Result<()> {
        loop{
            
            
            let _ = &self.terminal.draw(Self::draw_home)?;
            
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    return Ok(());
                }
            }
            
        }
    }
    
    pub fn draw_home(frame: &mut Frame) {
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
        let log_block = Block::default().borders(Borders::ALL).title("Log").title_position(ratatui::widgets::block::Position::Top);
        let input_block = Block::default().borders(Borders::ALL).title("Input").title_position(ratatui::widgets::block::Position::Top);
        
        let t = Text::raw(crate::story::MAINMENU);
        


        frame.render_widget(&menu_block, big_screen[0]);
        frame.render_widget(log_block, big_screen[1]);
        frame.render_widget(input_block, main_layout[1]);
        frame.render_widget(t, menu_block.inner(big_screen[0]));

    }
    
    pub fn draw(frame: &mut Frame) {
        let text = Text::raw("Hello World!");
        frame.render_widget(text, frame.area());
    }

}




