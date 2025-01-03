//In this file there will be story elements and string relations for the game


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

pub const COMBATHELPERMENU: &str = "attack: to attack\ndefend: to defend\npassiv:todo\n";

pub const PLAYERINFO: fn(name: &str, level: i8, health: u8, max_health: u8, armor: i8, inventory_size: i8) -> String = |name, level, health,max_health, armor, inventory_size | {
    let INFO = format!("\n
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

    ");
    INFO


};

pub const HELP: &str = "\
All commands avalibale / lower- or uppercase isn't important: \n
~Movement: [up, down, left, right] | you can move always but for a better experience open the map \n
~Map: displays the dungeonmap\n
~La | Look around: displays extra information for the current room\n
~inventory: opens the inventory\n
~info: displays the character stats \n
~loot: loots a corpse todo
";

pub const INVENTORYHELP: &str = "\n
~close: to close the inventoy
~drop [index]: drops the item / carefull you cant pick it up again
~equip [index] [slot]: equips an item to th given slot
~unequip [slot]: unequipts an item / carefull it is delete after
~inspect [slot]: inspect a give item

";






