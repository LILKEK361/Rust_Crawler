use std::any::Any;
use std::string::ToString;

//Url for the postgres db
pub(crate) const POSTGRESSURL: fn(user: &str, pass: &str, address: &str, name: &str) -> String =
    |user, pass, address, name| format!("postgres://{}:{}@{}/{}", user, pass, address, name);

pub(crate) const POSTGRESSEARCHQUERY: fn(table: &str, column: &str, filter: &str) -> String =
    |t, c, f| format!("Select * from {} WHERE {} = '{}';", t, c, f);
pub(crate) const CREATEGAMEDB: &str = "CREATE DATABASE gamedb";

//Alles zun den Waffen im Game
pub(crate) const WEAPONDBTABLECREATIONQUERY: &str = "Create table if not exists weapons (id SERIAL PRIMARY KEY, name TEXT NOT NULL, des TEXT NOT NULL, rarity TEXT NOT NULL, dmg_bonus INT NOT NULL, VALUE INT NOT NULL);";

pub const DELETEWEPONTABLE: &str = "DROP TABLE weapons";

pub const CREATEBASICWEAPONSINTABLE: &str = r#"INSERT into weapons (name, des, rarity, dmg_bonus, value) VALUES
('sword','a sword', 'common',2,5),
('axe', 'a axe', 'common', 4,5),
('dagger', 'a dagger','common', 3,6),
('spoon', 'a spoon', 'trash', 2,1);"#;

//Alles zu Treasuren
pub(crate) const TREASURETABLECREATIONQUERY: &str  = "Create table if not exists treasures (id SERIAL PRIMARY KEY, name TEXT NOT NULL, des TEXT NOT NULL, rarity TEXT NOT NULL, passiv TEXT NOT NULL, VALUE INT NOT NULL);";
pub const CREATEBASCITREASURE: &str = r#"INSERT into treasures (name, des, rarity, passiv, value) VALUES
('Red Orb','A reb marble.\Some stranges thoughts pop up in the back of you head', 'DEMONIC', 'The voices have gone strangly quiet.', 666),
('Blue Orb', 'A blue marble.\You feel the void closing,\but you cant quiet tell why' ,'SHIZO', '+100:HP/+5:AD',911 ),
('Strange Rock', 'A stange looking Rock.\Blue like the ocean, but it seems so flammable.\Maybe if you could lay it into something\and head it then...', 'TRASH', 'NONE',0 );
"#;

//Alles zu Equipmentitems
pub(crate) const EQUIPMENTTABLECREATIONQUERY: &str  = "Create table if not exists equip (id SERIAL PRIMARY KEY, name TEXT NOT NULL, des TEXT NOT NULL, slot TEXT NOT NULL, rarity TEXT NOT NULL, adbonus INT NOT NULL, VALUE INT NOT NULL);";

pub const CREATEBASICEQUIPTABLE: &str = r#"INSERT into equip (name, des,slot, rarity, adbonus, value) VALUES
('Iron Chestplate','a broken Iron Chestplate.\Kinda crusty on the edges','torso', 'common',2,5),
('Iron Leggins','a broken Iron Leggins.\Kinda crusty on the edges','pants', 'common',2,5),
('Iron Shoes','a broken Iron Shoes.\Kinda crusty on the edges','shoes', 'common',2,5),
('Iron Helm','a broken Iron Helm.\Kinda crusty on the edges','head', 'common',2,5)
;"#;

pub(crate) const CONSUMABLESTABLECREATIONQUERY: &str ="Create Table if not exists consumables (id SERIAL PRIMARY KEY, name TEXT NOT NULL, des TEXT NOT NULL,rarity TEXT NOT NULL, uses INT NOT NULL, buf INT NOT NULL, value INT NOT NULL);";

pub(crate) const CREATEBASICCONSUMABLES: &str = r#" INSERT into consumables (name, des, rarity,uses, buf, value) VALUES
('Healing Potion', 'A strange looking liquid.\The smell remindes you of\something. Where have you smelled it before?','common', 3, 15 ,0);
"#;

pub const ROOMTABLECREATINQUERY: &str = r#"CREATE TABLE if not exists rooms (id SERIAL PRIMARY KEY, roomname TEXT NOT NULL, roomdes TEXT NOT NULL);"#;

pub(crate) const CREATEBASICROOMS: &str = r#"INSERT into rooms (roomname, roomdes) VALUES
('Empty', 'This room looks empty. Just like my mind'),
('Laboretory', 'A big desk at the right side of the room. Some flask lie around on and under the table. Strange... '),
('Dirty Room', 'A dirty room. You never saw so much rubbish before.');"#;
