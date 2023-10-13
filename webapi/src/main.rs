#[macro_use]
extern crate rocket;

pub mod auth;
pub mod database;
pub mod handlers;

use dotenvy::dotenv;

use crate::handlers::{events, login, post, register};
use crate::database::Message;
use sqlx::Row;


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();


    let conn_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match sqlx::PgPool::connect(&conn_url).await {
        Ok(pool) => pool,
        Err(_) => {
            panic!("Error connecting to database");
        }
    };




    let (tx, mut rx) = rocket::tokio::sync::broadcast::channel::<Message>(1024);
    rocket::build()
        .manage(tx)
        .manage(pool)
        .mount("/", routes![post, events, login, register])
        .launch()
        .await?;

    Ok(())
}
