use std::clone::Clone;
use std::collections::HashMap;
use ratatui::{layout, Frame};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::layout::Alignment::Center;
use ratatui::layout::Direction::Horizontal;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use crate::gamelogic::game_screens::WindowContents::INVENTORY;
use crate::gamelogic::{draw_functions, gamehelperfunctions, konst};

pub enum WindowContents {
    INVENTORY(GameScreen),
    COMBAT(GameScreen),
    MAP(GameScreen),
    ROOM(GameScreen),
    VIC(GameScreen),
    DEATH(GameScreen),
}

impl WindowContents {
    pub fn new_inventory_screen() -> Self{
        INVENTORY( GameScreen{
            layout: Layout::new(Direction::Vertical,[Constraint::Percentage(90), Constraint::Percentage(10)]),
            content_layout: Layout::default()
                                .direction(Direction::Horizontal)
                                .margin(1)
                                .constraints([Constraint::Percentage(75), Constraint::Percentage(25)]),
            content: Option::from(Layout::default()
                                      .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
                                      .direction(Horizontal)),
        })
    }
}

impl Drawable for WindowContents{
     fn draw(&self, mut frame:  &mut Frame, input_string: &str, log: Vec<String>) {
        match &self {
            INVENTORY(screen) =>{

                //draw_functions::draw_inventory(frame, screen)

            }
            _ => {}
        }
    }
}




pub struct MainScreen{
    layout: Layout,
    content_layout: Layout,
    show: bool,
}

impl MainScreen {
    pub fn new() -> Self{
        Self{
            layout: Layout::new(Direction::Vertical,[Constraint::Percentage(90), Constraint::Percentage(10)]),
            content_layout: Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Percentage(75), Constraint::Percentage(25)]),
            show: false
        }
    }
}

impl Drawable for MainScreen {
    fn draw(&self, mut frame: &mut Frame, input_string: &str, log: Vec<String>) {
        let frame_layout = &self.layout.split(frame.area());
        let content_window = &self.content_layout.split( *&self.layout.split(frame.area())[0]);

        if *&self.show {
            frame.render_widget(Paragraph::new(konst::SPOILER).block(Block::default().borders(Borders::ALL))
                                    .alignment(Center)
                                    .wrap(Wrap { trim: true }),  *&self.content_layout.split( *&self.layout.split(frame.area())[0])[0])
        } else {
            frame.render_widget( Paragraph::new(konst::MAINMENU).block(Block::default().borders(Borders::ALL))
                                     .alignment(Center)
                                     .wrap(Wrap { trim: true }),  *&self.content_layout.split( *&self.layout.split(frame.area())[0])[0])
        }


        //TODO Make Log block
        frame.render_widget(draw_functions::create_log(log), *&self.content_layout.split( *&self.layout.split(frame.area())[0])[1]);


        frame.render_widget(Paragraph::new(input_string).block(Block::default()
            .borders(Borders::ALL)
            .title("Input")
            .title_position(ratatui::widgets::block::Position::Top)), *&self.layout.split(frame.area())[1]);

    }
}

pub struct GameScreen {
    layout: Layout,
    content_layout: Layout,
    pub(crate) content: Option<Layout>,


}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame, input_string: &str, log: Vec<String>);
}

//pub const  MAIN_LAYOUT: Layout = Layout::new(Direction::Vertical,[Constraint::Percentage(90), Constraint::Percentage(10)]);



//pub const CONTENT_WINDOW: Layout = Layout::default() .direction(Direction::Horizontal) .margin(1) .constraints([Constraint::Percentage(75), Constraint::Percentage(25)]);
/*
pub const INPUTBLOG: Block = Block::default()
    .borders(Borders::ALL)
    .title("Input")
    .title_position(ratatui::widgets::block::Position::Top);

pub const SPOILER_PARAGRAPH: Paragraph =  Paragraph::new(konst::SPOILER)
    .alignment(Center)
    .wrap(Wrap { trim: true });

pub const GREET_PARAGRAPH: Paragraph = Paragraph::new(konst::MAINMENU)
    .alignment(Center)
    .wrap(Wrap { trim: true });

pub const INVENTORY_LAYOUT: Layout = Layout::default()
    .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
    .direction(Horizontal);

*/


