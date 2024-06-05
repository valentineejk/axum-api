mod handler;
mod model;
mod route;
mod schema;

use dotenv::dotenv;
// use std::sync::Arc;

// use axum::http::{

// };

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
#[tokio::main]
async fn main() {
    println!("Hello, world!");
    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("NO DATABASE URL FOUND");
    print!("{}", db_url);
}
