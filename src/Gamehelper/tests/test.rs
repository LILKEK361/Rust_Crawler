#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::gamelogic::postgreshandler;
    use crate::dbpaths;

    #[test]
    pub fn write_to_db(){

        let mut handler = postgreshandler::PgHandler::new();

        handler.execute(dbpaths::CREATEBASCITREASURE.parse().unwrap()).unwrap()

    }
}
