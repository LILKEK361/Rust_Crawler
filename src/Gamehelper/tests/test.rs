#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::gamelogic::postgreshandler;
    use crate::dbpaths;

    #[test]
    pub fn write_to_db(){

        write_treasure_table();
        write_weapon_table();
        write_equip_table();
        write_consumable_table();
    }

    fn write_consumable_table() {
        let mut handler = postgreshandler::PgHandler::new();

        match handler.execute(dbpaths::CONSUMABLESTABLECREATIONQUERY.parse().unwrap()){
            Ok(..) => {
                handler.execute(dbpaths::CREATEBASICCONSUMABLES.parse().unwrap()).unwrap()
            }
            Err(..) => {eprintln!("Cound't create Consumable Table")}
        }
    }

    pub fn write_treasure_table() {
        let mut handler = postgreshandler::PgHandler::new();

        match handler.execute(dbpaths::TREASURETABLECREATIONQUERY.parse().unwrap()){
            Ok(..) => {
                handler.execute(dbpaths::CREATEBASCITREASURE.parse().unwrap()).unwrap()
            }
            Err(_) => {eprintln!("Coundt create Treasure Table")}
        }

    }

    pub fn write_weapon_table() {
        let mut handler = postgreshandler::PgHandler::new();

        match handler.execute(dbpaths::WEAPONDBTABLECREATIONQUERY.parse().unwrap()){
            Ok(..) => {
                handler.execute(dbpaths::CREATEBASICWEAPONSINTABLE.parse().unwrap()).unwrap()
            }
            Err(_) => {eprintln!("Coundt create weapon Table")}
        }
    }

    pub fn write_equip_table() {
        let mut handler = postgreshandler::PgHandler::new();

        match handler.execute(dbpaths::EQUIPMENTTABLECREATIONQUERY.parse().unwrap()){
            Ok(..) => {
                handler.execute(dbpaths::CREATEBASICEQUIPTABLE.parse().unwrap()).unwrap()
            }
            Err(_) => {eprintln!("Coundt create equip Table")}
        }
    }


}

