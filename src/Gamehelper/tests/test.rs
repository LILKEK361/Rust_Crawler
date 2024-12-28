#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::gamelogic::postgreshandler;
    use crate::dbpaths;

    #[test]
    pub fn write_to_db(){

      write_treasure_table();
        write_weapon_table();
    }


    pub fn write_treasure_table() {
        let mut handler = postgreshandler::PgHandler::new();

        match handler.execute(dbpaths::TREASURETABLECREATIONQUERY.parse().unwrap()){
            Ok(..) => {
                handler.execute(dbpaths::CREATEBASCITREASURE.parse().unwrap()).unwrap()
            }
            Err(_) => {eprintln!("Coundt create Table")}
        }

    }

    pub fn write_weapon_table() {
        let mut handler = postgreshandler::PgHandler::new();

        match handler.execute(dbpaths::WEAPONDBTABLECREATIONQUERY.parse().unwrap()){
            Ok(..) => {
                handler.execute(dbpaths::CREATEBASICWEAPONSINTABLE.parse().unwrap()).unwrap()
            }
            Err(_) => {eprintln!("Coundt create Table")}
        }
    }


}

