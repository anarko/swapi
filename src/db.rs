extern crate diesel;

use diesel::prelude::*;
use diesel::RunQueryDsl;
use dotenv::dotenv;
use std::env;

use crate::schema::swaps::dsl::*;

use crate::models::{DbSwap,SwapConfig};

fn connect_db() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}",
                                   database_url))
}

pub fn get_swap_config(key:String) -> String {
    use crate::schema::swap_config::dsl::*;
    let connection = connect_db();
    let config = swap_config.filter(conf.eq(key)).first::<SwapConfig>(&connection);
    return config.unwrap_or(SwapConfig{conf:"".to_string(),value:"".to_string()}).value;
}

pub fn save_swap(insert_swap: DbSwap) {
    let connection = connect_db();

    diesel::insert_into(swaps)
        .values(&insert_swap)
        .execute(&connection)
        .expect("Error DB INSERT");
}
