use crate::auth::{create_jwt, NetworkResponse, JWT};
use crate::database::{Credentials, Message, User};
use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use cookie::time::OffsetDateTime;
use rocket::http::{Cookie, CookieJar};
use rocket::response::stream::{Event, EventStream};
use rocket::serde::json::Json;
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::{channel, error::RecvError, Receiver, Sender};
use rocket::Shutdown;
use rocket::State;
use sqlx::Row;
use sqlx::{Pool, Postgres};
use rocket::serde::{Serialize, Deserialize};
use rocket::response::status::NoContent;


#[post("/login", format = "application/json", data = "<info>")]
pub async fn login(
    db: &State<Pool<Postgres>>,
    cookies: &CookieJar<'_>,
    info: Json<Credentials>,
) -> Result<String, NetworkResponse> {
    let creds = info.into_inner();
    let Ok(res) = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(&creds.username)
        .fetch_optional(&**db)
        .await
    else {
        return Err(NetworkResponse::BadRequest("Database error".to_string()));
    };
    let entry = match res {
        Some(entry) => entry,
        None => return Err(NetworkResponse::BadRequest("User not found".to_string())),
    };

    let parsed_hash = PasswordHash::new(&entry.hash).unwrap();

    if let Err(_) = Argon2::default().verify_password(creds.password.as_bytes(), &parsed_hash) {
        return Err(NetworkResponse::Unauthorized(
            "Invalid password".to_string(),
        ));
    };

    let cookie = match login_user(creds) {
        Ok(cookie) => cookie,
        Err(err) => return Err(err),
    };
    // let response = Response::build().header(cookie).finalize();

    cookies.add(cookie);
    Ok(entry.id.to_string())
}



#[post("/register", format = "application/json", data = "<info>")]
pub async fn register(
    db: &State<Pool<Postgres>>,
    info: Json<Credentials>,
) -> Result<(), NetworkResponse> {
    let user = User::new(info.into_inner());
    // check if the user with the same username already exists
    let Ok(None) = sqlx::query("SELECT * FROM users WHERE username = $1")
        .bind(&user.username)
        .fetch_optional(&**db)
        .await
    else {
        return Err(NetworkResponse::BadRequest(
            "User already exists".to_string(),
        ));
    };
    let query =
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1").bind(&user.username);
    let mut res = query.fetch_optional(&**db).await;
    match res {
        Ok(_) => match res.unwrap() {
            Some(_) => {
                return Err(NetworkResponse::BadRequest(
                    "User already exists".to_string(),
                ))
            }
            None => (),
        },
        Err(err) => println!("Error: {:?}", err),
    }
    println!("aha");
    match sqlx::query("INSERT INTO users (id, username, hash, salt) VALUES ($1, $2, $3, $4)")
        .bind(&user.id)
        .bind(&user.username)
        .bind(&user.hash)
        .bind(&user.salt)
        .execute(&**db)
        .await
    {
        Ok(_) => (),
        Err(err) => return Err(NetworkResponse::InternalServerError(err.to_string())),
    };
    Ok(())
}

#[post("/message", format = "application/json", data = "<message>")]
pub async fn message(
    db: &State<Pool<Postgres>>,
    _key: JWT,
    message: Json<Message>,
    queue: &State<Sender<Message>>,
) -> Result<(), NetworkResponse> {
    // A send 'fails' if there are no active subscribers. That's okay.:w
    let msg = message.into_inner();
    let date = chrono::Utc::now();
    println!("JWT {:?}", _key);
    // let id = match crate::database::get_uuid(&**db, msg.username.clone()).await {
    //     Ok(id) => id,
    //     Err(err) => return Err(NetworkResponse::InternalServerError(err.to_string())),
    // };

    let _res = queue.send(msg.clone());
    let res = match sqlx::query("INSERT INTO messages (username, sender_id, message, sent_at) VALUES ($1, $2, $3, $4)")
        .bind(&msg.username)
        .bind(&msg.sender_id)
        .bind(&msg.message)
        .bind(&date)
        .execute(&**db)
        .await
    {
        Ok(_) => (),
        Err(err) => return Err(NetworkResponse::InternalServerError(err.to_string())),
    };
    Ok(())
}

#[get("/events")]
pub async fn events(queue: &State<Sender<Message>>, mut end: Shutdown) -> EventStream![] {
    let mut rx = queue.subscribe();
    EventStream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };

            yield Event::json(&msg);
        }
    }
}

fn login_user<'c>(creds: Credentials) -> Result<Cookie<'c>, NetworkResponse> {
    let mut expiration_date: usize = 0;
    let user = User::new(creds);
    let token = match create_jwt(user.id.as_u128(), &mut expiration_date) {
        Ok(token) => token,
        Err(err) => return Err(NetworkResponse::Unauthorized(err.to_string())),
    };

    let date = match OffsetDateTime::from_unix_timestamp(expiration_date as i64) {
        Ok(date) => date,
        Err(err) => return Err(NetworkResponse::InternalServerError(err.to_string())),
    };
    let cookie = Cookie::build("api_token", token)
        .expires(date)
        .path("/")
        .http_only(true)
        .finish();

    Ok(cookie)
}

#[get("/health")]
pub fn health() -> rocket::http::Status {
    rocket::http::Status::Ok
}

// #[options("/login")]
// pub fn login_preflight() -> NoContent {
//     NoContent
// }
//
// #[options("/register")]
// pub fn register_preflight() -> NoContent {
//     NoContent
// }
//
// #[options("/message")]
// pub fn message_preflight() -> NoContent {
//     NoContent
// }
//
// #[options("/events")]
// pub fn events_preflight() -> NoContent {
//     NoContent
// }
