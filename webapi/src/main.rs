#[macro_use]
extern crate rocket;

pub mod auth;
pub mod database;
pub mod handlers;

use dotenvy::dotenv;

use crate::database::Message;
use crate::handlers::{
    events,
    health,
    login,
    register,
    message
};
use rocket::fairing::{Fairing, Info, Kind};
use sqlx::Row;
use rocket::http::{ContentType, Header, Method, Status};
use rocket::{Request, Response};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "CORS",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "http://localhost:3000"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, DELETE, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        response.set_header(Header::new("Access-Control-Max-Age", "86400"));
        // response.set_header(Header::new("Content-Type", "application/json"));

        // source: https://webprogramming.ninja/2022/08/25/handling-options-requests-in-rust-using-rocket-with-cors/
        if request.method() == Method::Options {
            let body = "";
            response.set_header(ContentType::Plain);
            response.set_sized_body(body.len(), std::io::Cursor::new(body));
            response.set_status(Status::Ok);
        }
    }

}

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
        .attach(CORS)
        .manage(tx)
        .manage(pool)
        .mount(
            "/",
            routes![
                message,
                events,
                login,
                register,
                health,
            ],
        )
        .launch()
        .await?;

    Ok(())
}
