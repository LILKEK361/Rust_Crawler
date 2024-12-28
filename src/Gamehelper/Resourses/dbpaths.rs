use std::any::Any;
use std::string::ToString;


//Url for the postgres db
pub(crate) const POSTGRESSURL: fn(user: &str,pass: &str,address: &str,name: &str) -> String = |user,pass,address,name| {
    format!("postgres://{}:{}@{}/{}", user, pass, address, name)
};

pub(crate) const POSTGRESSEARCHQUERY: fn(table: &str,column: &str, filter: &str) -> String = |t,c, f| {
    format!("Select * from {} WHERE {} = '{}';",t,c,f)
};
pub(crate) const CREATEGAMEDB: &str ="CREATE DATABASE gamedb";

//Alles zun den Waffen im Game
pub(crate) const WEAPONDBTABLECREATIONQUERY: &str = "Create table if not exists weapons (id SERIAL PRIMARY KEY, name TEXT NOT NULL, des TEXT NOT NULL, rarity TEXT NOT NULL, dmg_bonus INT NOT NULL, VALUE INT NOT NULL);";

pub const DELETEWEPONTABLE: &str = "DROP TABLE weapons";

pub const CREATEBASICWEAPONSINTABLE: &str = r#"INSERT into weapons (name, des, rarity, dmg_bonus, value) VALUES
('sword','a sword', 'common',2,5),
('axe', 'a axe', 'common', 4,5),
('dagger', 'a dagger','common', 3,6),
('spoon', 'a spoon', 'trash', 2,1);"#;

//Alles zu Treasuren
pub(crate) const TREASURETABLECREATIONQUERY: &str  = "Create table if not exists treasures (id SERIAL PRIMARY KEY, name TEXT NOT NULL, des TEXT NOT NULL, rarity TEXT NOT NULL, passiv TEXT NOT NULL, VALUE INT NOT NULL)";
pub const CREATEBASCITREASURE: &str = r#"INSERT into treasures (name, des, rarity, passiv, value) VALUES
('Red Orb','A reb marble.\Some stranges thoughts pop up in the back of you head', 'DEMONIC', 'The voices have gone strangly quiet.', 666),
('Blue Orb', 'A blue marble.\You fill the void closing,\but you cant quiet tell why' ,'SHIZO', '+100:HP/+5:AD',911 ),
('Strange Rock', 'A stange looking Rock.\Blue like the ocean, but it seems so flammable.\Maybe if you could lay it into something\and head it then...', 'TRASH', 'NONE',0 );
"#;