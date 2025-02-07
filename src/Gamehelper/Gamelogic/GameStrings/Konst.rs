//In this file there will be story elements and string relations for the game

use crossterm::style::Stylize;
use terminal_link::Link;

pub const TOMANYARGUMENTS: &str =
    "You have supplyed to many arguments.\nPls use [filename] -- help to get help";

pub const ARGUMENTHELP: &str = r#"

Usage: [filename][EXE] -- <arg>

Options:
  --help           Print help
  --setup          Starts the db setup for the game
  --start          Starts the game, if there is a db connection
"#;

pub const UNKONWCMD: &str = "Unkown argument. Pls use -- help to see help";

pub const MAINMENU: &str = "
    Welcome to the shizo decend. This game was forged by me Lemmings.\n
    I may not be perfect, but it is...\n
    You can add your own weapons, treasures, consumables etc. to the db.\n
    If you'r are intressted in that pls read the section Adding to DB in my Github read me.\n
    If you'r here to play the game just type [start].\n
    If you want to now what you'r getting into [spoiler], to get a short explanation on the gameplay.\n
    \n
    And above else have fun :).

";

pub const SPOILER: &str = "\
*** SPOILER **

This is a simple dungeon Roguelike\n
You can type [help] if your are lost or don't remember all th commands.\n\
The enemy type is a basic Goblin todo :(\n
Your goal in the dungeon is to reach the final boss and defeat it. Be carefully the Boss ain't that weak.\n
You can find different item in the dungeon, some are usefull others aren't.\n
The rest should be self explaining, but if you are lost or encounter a bug you can create a new ticket on Github.\n
Type [menu] to see the menu text again.\n
";

pub const DEATHMESSAGE: &str = "\n
    YOU DIED\n
To be honest, I didn't expect anything else.\n
Type [exit] to return to the home screen.
";

pub const VICOTRYMESSAGE: &str = "\n
    Nice One.\n
    You really slayed the final boss.\n
    Good Job.\n
    Feel free to write a rating about the game or dont.\n
    Type [exit] to return to the home screen.\n

";

pub const PLAYERINFO: fn(
    name: &str,
    level: i8,
    health: u8,
    max_health: u8,
    armor: i8,
    inventory_size: i8,
) -> String = |name, level, health, max_health, armor, inventory_size| {
    let INFO = format!(
        "\n
        Stats:
        -----
        Name: {name}
        -----
        Level: {level}
        -----
        Health:{health}/{max_health}
        -----
        Armor: {armor} AD
        -----
        Inventory size: {inventory_size}
        -----

    "
    );
    INFO
};

pub const HELP: &str = "\
All commands avalibale / lower- or uppercase isn't important: \n
~ [up, down, left, right] | you can move always but for a better experience open the map \n
~ [map]: displays the dungeonmap\n
~ [la | Look around]: displays extra information for the current room\n
~ [info]: displays the character stats \n
~ [loot]: loots a corpse todo \n
~ [clear]: to clear the log
------------------------
Inventory commands:\n
~ [inventory | i] : opens the inventory\n
~ [drop [index]]: drops the item / carefull you cant pick it up again\n
~ [equip [index] [slot]]: equips an item to th given slot\n
~ [unequip [slot]]: unequipts an item \n
~ [inspect [index]]: inspect a give item\n
------------------------
Combat commands: \n
attack: to attack\n
defend: to defend\n
";

pub const LOGBUFFER: usize = 5;

pub const TEST_FIlE_PATH: &str = "./testingdata.json";
pub const FIlE_PATH: &str = "./data.json";

pub const GITHUBLINK: &str = "https://github.com/LILKEK361/Rust_Crawler";

pub const GAMENAME: &str = "RUST_CRAWLER";
pub const JSONFILEERROR: fn(link: Link) -> String = |link|  { format ! ("
    Look like you dont have the config file for the game :(.\n
    You may want to go to my Github page to download the file : \n
    {}\n
    There you also can read about creating your own config for the game\n
    or expand the one I provid.
", link.url)};

pub const JSONFILEPATHFOUND: fn(path : &str) -> String = |path|{
    format!("JSONFILE found at: \n {path}")
};