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
pub(crate) const WEAPONDBTABLECREATIONQUERY: &str = "Create table if not exists weapons (id SERIAL PRIMARY KEY, name TEXT NOT NULL, des TEXT NOT NULL, rarity TEXT NOT NULL, dmg_bonus INT NOT NULL);";

pub const DELETEWEPONTABLE: &str = "DROP TABLE weapons";

pub const CREATEBASICWEAPONSINTABLE: &str = r#"INSERT into weapons (name, des, rarity, dmg_bonus) VALUES
('sword','a sword', 'common',2),
('axe', 'a axe', 'common', 4),
('dagger', 'a dagger','common', 3),
('spoon', 'a spoon', 'trash', 2);"#;
