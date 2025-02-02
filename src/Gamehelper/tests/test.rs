#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::dbpaths;
    use crate::gamelogic::postgreshandler;
    use crate::gamelogic::postgreshandler::PgHandler;

    #[test]
    pub fn write_to_db() {
        println!("Weclome to the db setup:");
        println!("First the treasure table:");
        let handler = &mut PgHandler::new();
        write_treasure_table(handler);
        write_weapon_table(handler);
        write_equip_table(handler);
        write_consumable_table(handler);
        write_room_table(handler);
    }

    fn write_consumable_table(handler: &mut PgHandler) {
        match handler.execute(dbpaths::CONSUMABLESTABLECREATIONQUERY.parse().unwrap()) {
            Ok(..) => {
                println!("Consumable Table created successfully");
                match handler.execute(dbpaths::CREATEBASICCONSUMABLES.parse().unwrap()) {
                    Ok(..) => {
                        println!("Values were successfully inserted");
                    }
                    Err(..) => {
                        eprintln!("Error on value insertion")
                    }
                }
            }
            Err(..) => {
                eprintln!("Cound't create Consumable Table")
            }
        }
    }

    pub fn write_treasure_table(handler: &mut PgHandler) {
        match handler.execute(dbpaths::TREASURETABLECREATIONQUERY.parse().unwrap()) {
            Ok(..) => {
                println!("Treasure Table created successfully");
                match handler.execute(dbpaths::CREATEBASCITREASURE.parse().unwrap()) {
                    Ok(..) => {
                        println!("Values were successfully inserted")
                    }
                    Err(..) => {
                        eprintln!("Error on value insertion");
                    }
                }
            }
            Err(_) => {
                eprintln!("Coundt create Treasure Table")
            }
        }
    }

    pub fn write_weapon_table(handler: &mut PgHandler) {
        match handler.execute(dbpaths::WEAPONDBTABLECREATIONQUERY.parse().unwrap()) {
            Ok(..) => {
                println!("Weapon Table created successfully");

                match handler.execute(dbpaths::CREATEBASICWEAPONSINTABLE.parse().unwrap()) {
                    Ok(..) => {
                        println!("Values were successfully inserted")
                    }
                    Err(..) => {
                        eprintln!("Error on value insertion");
                    }
                }
            }
            Err(_) => {
                eprintln!("Coundt create weapon Table")
            }
        }
    }

    pub fn write_equip_table(handler: &mut PgHandler) {
        match handler.execute(dbpaths::EQUIPMENTTABLECREATIONQUERY.parse().unwrap()) {
            Ok(..) => {
                println!("Equip Table created successfully");

                match handler.execute(dbpaths::CREATEBASICEQUIPTABLE.parse().unwrap()) {
                    Ok(..) => {
                        println!("Values were successfully inserted")
                    }
                    Err(..) => {
                        eprintln!("Error on value insertion");
                    }
                }
            }
            Err(_) => {
                eprintln!("Coundt create Equip Table")
            }
        }
    }

    pub fn write_room_table(handler: &mut PgHandler) {
        match handler.execute(dbpaths::ROOMTABLECREATINQUERY.parse().unwrap()) {
            Ok(..) => {
                println!("Room Table created successfully");

                match handler.execute(dbpaths::CREATEBASICROOMS.parse().unwrap()) {
                    Ok(..) => {
                        println!("Values were successfully inserted")
                    }
                    Err(..) => {
                        eprintln!("Error on value insertion");
                    }
                }
            }
            Err(_) => {
                eprintln!("Coundt create room Table")
            }
        }
    }
}
