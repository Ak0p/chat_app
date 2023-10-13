// import rocket
use argon2::{
    password_hash::{PasswordHash,
    rand_core::OsRng,
    PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
// use rocket::serde::{self, Deserialize, Serialize};
use rocket::serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(crate = "rocket::serde")]
pub struct Message {
    // #[field(validate = len(..30))]
    // pub room: String,
    pub sender_id: uuid::Uuid,
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




