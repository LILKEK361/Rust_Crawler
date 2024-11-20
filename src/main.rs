use ratatui::Terminal;

#[path ="./Gamehelper/player.rs"]
mod player;
#[path = "Gamehelper/Gamelogic/terminalhandler.rs"]
mod terminalhandler;
#[path= "Gamehelper/GameObjects/item_handler.rs"]
mod item_handler;
#[path= "Gamehelper/GameObjects/Items/weaponitem.rs"]
mod weaponitem;


fn main() {
    terminalhandler::TerminalHandler::new().draw();
}
