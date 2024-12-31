# RUST CRAWLER

Hello and Welcome.
This Project of mine, is a simple Dungeon Crawler writen in Rust. It uses Postgres for the Database Backend and Ratatui for the Frontend in the terminal.

## Table of Contents: 
1. [Setup](###Setup)
2. [Starting](###Starting)

### Setup
1. Make sure you have postgres installed.
2. Run the following commands in order to create the games Database:

- `Create table if not exists weapons (id SERIAL PRIMARY KEY, name TEXT NOT NULL, des TEXT NOT NULL, rarity TEXT NOT NULL, dmg_bonus INT NOT NULL, VALUE INT NOT NULL);`

- `INSERT into weapons (name, des, rarity, dmg_bonus, value) VALUES('sword','a sword', 'common',2,5),('axe', 'a axe', 'common', 4,5),('dagger', 'a dagger','common', 3,6),('spoon', 'a spoon', 'trash', 2,1);`

- `Create table if not exists treasures (id SERIAL PRIMARY KEY, name TEXT NOT NULL, des TEXT NOT NULL, rarity TEXT NOT NULL, passiv TEXT NOT NULL, VALUE INT NOT NULL);`

- `INSERT into treasures (name, des, rarity, passiv, value) VALUES ('Red Orb','A reb marble.\Some stranges thoughts pop up in the back of you head', 'DEMONIC', 'The voices have gone strangly quiet.', 666), ('Blue Orb', 'A blue marble.\You feel the void closing,\but you cant quiet tell why' ,'SHIZO', '+100:HP/+5:AD',911 ), ('Strange Rock', 'A stange looking Rock.\Blue like the ocean, but it seems so flammable.\Maybe if you could lay it into something\and head it then...', 'TRASH', 'NONE',0 );`

- `Create table if not exists equip (id SERIAL PRIMARY KEY, name TEXT NOT NULL, des TEXT NOT NULL, slot TEXT NOT NULL, rarity TEXT NOT NULL, adbonus INT NOT NULL, VALUE INT NOT NULL);`

- `INSERT into equip (name, des,slot, rarity, adbonus, value) VALUES ('Iron Chestplate','a broken Iron Chestplate.\Kinda crusty on the edges','torso', 'common',2,5),('Iron Leggins','a broken Iron Leggins.\Kinda crusty on the edges','pants', 'common',2,5),('Iron Shoes','a broken Iron Shoes.\Kinda crusty on the edges','shoes', 'common',2,5),('Iron Helm','a broken Iron Helm.\Kinda crusty on the edges','head', 'common',2,5);`

- `Create Table if not exists consumables (id SERIAL PRIMARY KEY, name TEXT NOT NULL, des TEXT NOT NULL,rarity TEXT NOT NULL, uses INT NOT NULL, value INT NOT NULL);`

- `INSERT into consumables (name, des, rarity,uses, value) VALUES ('Healing Potion', 'A strange looking liquid.\The smell remindes you of\something. Where have you smelled it before?','common', 3, 0);`

3. Make sure you have the Rust Toolchain installed

### Starting
- To start the progamm, open the Directorie of the .exe in an Editor of your choise. And start the .exe.
