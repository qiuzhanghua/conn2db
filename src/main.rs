#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    if db_url.starts_with("postgres") {
        if let Ok(conn) = PgConnection::establish(&db_url) {
            use diesel::expression::sql_literal::sql;
            let version = sql("select version() v")
                .get_results::<String>(&conn)
                .unwrap();
            println!("Get connection to {:?}. 👍", version[0])
        } else {
            panic!("Connection to PostgreSQL Error!")
        }
    } else if let Ok(conn) = MysqlConnection::establish(&db_url) {
        use diesel::expression::sql_literal::sql;
        let version = sql("select version() v")
            .get_results::<String>(&conn)
            .unwrap();
        println!("Get connection to MySQL {:?}.👍", version[0])
    } else {
        panic!("DATABASE_URL Error?")
    }
}
