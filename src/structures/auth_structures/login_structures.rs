use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Serialize, Deserialize)]
pub struct LoginInfo {
    pub phone_number: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
    pub exp: usize,
    pub id: RecordId,
    pub phone_number: String,
    pub username: String,
    // pub address: Option<String>,
}
