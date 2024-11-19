use ratatui::Terminal;

#[path ="./Gamehelper/player.rs"]
mod player;
#[path ="./Gamehelper/Gamelogic/Terminalhandler.rs"]
mod terminalhandler;

fn main() {
    terminalhandler::TerminalHandler::new().draw();
}
