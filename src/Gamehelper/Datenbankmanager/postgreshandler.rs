use crate::gamehelper::dbpaths;
use postgres::{Client, NoTls, Row};
use std::io::Error;

pub(crate) struct PgHandler {
    url: String,
    client: Client,
}

impl PgHandler {
    pub fn new() -> Self {
        Self {
            url: dbpaths::POSTGRESSURL("postgres", "123", "localhost", "postgres"),
            client: Client::connect(
                &*dbpaths::POSTGRESSURL("postgres", "123", "localhost", "postgres"),
                NoTls,
            )
            .unwrap(),
        }
    }

    pub fn execute(&mut self, query: String) -> Result<(), String> {
        match &self.client.execute(&*query, &[]) {
            Ok(..) => Ok(()),
            Err(error) => Err(error.to_string()),
        }
    }

    pub fn search(&mut self, search: &str) -> Vec<Row> {
        let mut rows: Vec<Row> = self.client.query(search, &[]).unwrap();
        rows
    }
}
