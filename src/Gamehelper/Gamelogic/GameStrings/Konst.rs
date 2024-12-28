//In this file there will be story elements and string relations for the game


pub const MAINMENU: &str = "Welcome to the game! \n
                        1. Start Game \n
                        2. Exit Game \n
                        ";

pub const COMBATHELPERMENU: &str = "attack: to attack\ndefend: to defend\npassiv:todo\n";

pub const PLAYERINFO: fn(name: &str, level: i8, health: u8, max_health: i8, armor: i8, inventory_size: i8) -> String = |name, level, health,max_health, armor, inventory_size | {
    let INFO = format!("\n
        Stats:
        -----
        Name: {name}
        -----
        Level: {level}
        -----
        Health:{health}/{max_health}
        -----
        Armor: {armor}
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






