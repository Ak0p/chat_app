// import rocket
use argon2::{
    password_hash::{PasswordHash,
    rand_core::OsRng,
    PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
// use rocket::serde::{self, Deserialize, Serialize};
use rocket::serde::{Deserialize, Serialize};
use sqlx::{FromRow, Postgres, Pool};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(crate = "rocket::serde")]
pub struct MessageData {
    // #[field(validate = len(..30))]
    // pub room: String,
    pub sender_id: uuid::Uuid,
    pub message: String,
    pub sent_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Message {
    // #[field(validate = len(..30))]
    // pub room: String,
    pub username: String,
    pub message: String,
    pub sent_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Credentials {
   pub username: String,
   pub password: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub hash: String,
    pub salt: String, 
}

impl User {
    pub fn new(creds: Credentials) -> Self {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default(); 
        let hash = argon2.hash_password(creds.password.as_bytes(), &salt).unwrap().to_string();
         User {
            id: uuid::Uuid::new_v4(),
            username: creds.username,
            hash,
            salt: salt.to_string(),
        }
    }
}

pub async fn get_uuid(db: &Pool<Postgres>, username: String ) -> Result<uuid::Uuid, sqlx::Error> {
    let res = match sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(username)
        .fetch_optional(db)
        .await {
            Ok(res) => res,
            Err(err) => return Err(err),
        };
    Ok(res.unwrap().id)
}




