use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Serialize, Deserialize, Clone)]
pub struct AccountInfo {
    pub id: RecordId,
    pub email_address: String,
    pub username: String,
    pub password: String,
    // pub address: Option<String>,
    // pub is_driver: Option<FormStatus>,
    // pub is_owner: Option<FormStatus>,
    // pub id_approved: Option<FormStatus>,
}
