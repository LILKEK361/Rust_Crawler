use std::io::Stdout;

use crossterm::{self, event};
use ratatui::{prelude::CrosstermBackend, text::Text, widgets::Paragraph, Terminal};

pub struct TerminalHandler{
   pub terminal: Terminal<CrosstermBackend<Stdout>>
}

impl TerminalHandler {
    pub fn new() -> Self {
        let terminal = ratatui::init();
        Self { terminal }
    }

    pub fn draw(&mut self) {
        self.terminal.draw(|frame| {
            
            frame.render_widget(&Paragraph::new("Hello, world!"), frame.area());
        }).unwrap();
    }

    pub fn run(&mut self) {
        loop{
            self.terminal.draw(self.draw());
        }
    }
}