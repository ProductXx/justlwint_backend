use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Serialize, Deserialize)]
pub struct LoginInfo {
    pub email_address: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
    pub exp: usize,
    pub id: RecordId,
    pub email_address: String,
    pub username: String,
    // pub address: Option<String>,
}
