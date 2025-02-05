use ratatui::layout::Alignment::Center;
use ratatui::layout::Direction::Horizontal;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::{layout, Frame};
use std::clone::Clone;
use std::collections::HashMap;

use crate::gamelogic::game_screens::WindowContents::{COMBAT, DEATH, HELP, INFO, INVENTORY, MAPSCREEN, ROOM, VIC};
use crate::gamelogic::{draw_functions, gamehelperfunctions, konst};
use crate::gameobjects::player::Player;

pub enum WindowContents {
    INVENTORY(GameScreen),
    COMBAT(GameScreen), //TODO
    MAPSCREEN(GameScreen),
    ROOM(GameScreen),  //TODO
    VIC(GameScreen),   //TODO
    DEATH(GameScreen), //TODO
    HELP(GameScreen),
    INFO(GameScreen),
}

impl WindowContents {
    pub fn new_inventory_screen() -> Self {
        INVENTORY(GameScreen::from(
            Layout::default()
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
                .direction(Horizontal),
        ))
    }

    pub fn new_map_screen() -> Self {
        MAPSCREEN(GameScreen::new())
    }

    pub fn new_vic_screen() -> Self {
        VIC(GameScreen::new())
    }
    pub fn new_death_screen() -> Self {
        DEATH(GameScreen::new())
    }

    pub fn new_room_screen() -> Self {
        ROOM(GameScreen::from(
            Layout::default().direction(Horizontal).constraints([
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ]),
        ))
    }
    pub fn new_info_screen() -> Self {
        INFO(GameScreen::new())
    }

    pub fn new_help_screen() -> Self {
        HELP(GameScreen::new())
    }

    pub fn new_combat_screen() -> Self {
        COMBAT(GameScreen::from(Layout::default().direction(Horizontal).constraints([Constraint::Ratio(1,3),Constraint::Ratio(1,3),Constraint::Ratio(1,3)])))
    }
}

impl Drawable for WindowContents {
    fn draw(&self, mut frame: &mut Frame, input_string: &str, log: Vec<String>) {
        match &self {
            INVENTORY(screen) => {

                draw_functions::draw_inventory(frame, screen);
                draw_functions::draw_log_and_input(frame,  log,input_string, screen);
            }
            MAPSCREEN(screen) => {
                draw_functions::draw_map(
                    frame,
                    screen
                        .content_layout
                        .split(screen.layout.split(frame.area())[0])[0],
                );

                draw_functions::draw_log_and_input(frame, log, input_string, screen);
            }
            DEATH(screen) => {
                frame.render_widget(
                    Paragraph::new(konst::DEATHMESSAGE)
                        .block(Block::default().borders(Borders::ALL).title("Death"))
                        .alignment(Alignment::Center)
                        .wrap(Wrap { trim: true }),
                    screen
                        .content_layout
                        .split(screen.layout.split(frame.area())[0])[0],
                );

                draw_functions::draw_log_and_input(frame, log, input_string, screen);
            }
            VIC(screen) => {
                frame.render_widget(
                    Paragraph::new(konst::VICOTRYMESSAGE)
                        .block(Block::default().borders(Borders::ALL).title("Victory"))
                        .alignment(Alignment::Center)
                        .wrap(Wrap { trim: true }),
                    screen
                        .content_layout
                        .split(screen.layout.split(frame.area())[0])[0],
                );

                draw_functions::draw_log_and_input(frame, log, input_string, screen);
            }
            ROOM(screen) => {
                if let Some(content) = &screen.content {
                    draw_functions::draw_room(
                        frame,
                        content.split(
                            screen
                                .content_layout
                                .split(screen.layout.split(frame.area())[0])[0],
                        )[1],
                    );
                }
                draw_functions::draw_log_and_input(frame, log, input_string, screen);
            }
            HELP(screen) => {
                frame.render_widget(
                    Paragraph::new(konst::HELP)
                        .block(Block::new().borders(Borders::ALL).title("Dungeon")),
                    screen
                        .content_layout
                        .split(screen.layout.split(frame.area())[0])[0],
                );

                draw_functions::draw_log_and_input(frame, log, input_string, screen);
            }
            INFO(screen) => {
                draw_functions::draw_player_info(frame, screen);
                draw_functions::draw_log_and_input(frame, log, input_string, screen);
            }
            COMBAT(screen) => {
                draw_functions::draw_combat(frame, screen);
                draw_functions::draw_log_and_input(frame, log, input_string, screen);
            }

            _ => {}
        }
    }
}

pub struct MainScreen {
    layout: Layout,
    content_layout: Layout,
    show: bool,
}

impl MainScreen {
    pub fn new() -> Self {
        Self {
            layout: Layout::new(
                Direction::Vertical,
                [Constraint::Percentage(90), Constraint::Percentage(10)],
            ),
            content_layout: Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Percentage(75), Constraint::Percentage(25)]),
            show: false,
        }
    }
}

impl Drawable for MainScreen {
    fn draw(&self, mut frame: &mut Frame, input_string: &str, log: Vec<String>) {
        if *&self.show {
            frame.render_widget(
                Paragraph::new(konst::SPOILER)
                    .block(Block::default().borders(Borders::ALL))
                    .alignment(Center)
                    .wrap(Wrap { trim: true }),
                *&self
                    .content_layout
                    .split(*&self.layout.split(frame.area())[0])[0],
            )
        } else {
            frame.render_widget(
                Paragraph::new(konst::MAINMENU)
                    .block(Block::default().borders(Borders::ALL))
                    .alignment(Center)
                    .wrap(Wrap { trim: true }),
                *&self
                    .content_layout
                    .split(*&self.layout.split(frame.area())[0])[0],
            )
        }

        //TODO Make Log block
        frame.render_widget(
            draw_functions::create_log(
                log,
                *&self
                    .content_layout
                    .split(*&self.layout.split(frame.area())[0])[1]
                    .height as usize,
            ),
            *&self
                .content_layout
                .split(*&self.layout.split(frame.area())[0])[1],
        );

        frame.render_widget(
            Paragraph::new(input_string).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Input")
                    .title_position(ratatui::widgets::block::Position::Top),
            ),
            *&self.layout.split(frame.area())[1],
        );
    }
}

pub struct GameScreen {
    pub(crate) layout: Layout,
    pub(crate) content_layout: Layout,
    pub(crate) content: Option<Layout>,
}

impl GameScreen {
    pub fn new() -> Self {
        GameScreen {
            layout: Layout::new(
                Direction::Vertical,
                [Constraint::Percentage(90), Constraint::Percentage(10)],
            ),
            content_layout: Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Percentage(75), Constraint::Percentage(25)]),
            content: None,
        }
    }

    pub fn from(content: Layout) -> Self {
        GameScreen {
            layout: Layout::new(
                Direction::Vertical,
                [Constraint::Percentage(90), Constraint::Percentage(10)],
            ),
            content_layout: Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Percentage(75), Constraint::Percentage(25)]),
            content: Option::from(content),
        }
    }
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
