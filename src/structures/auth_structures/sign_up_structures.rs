use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

// use crate::structures::{/*forms_structures::FormStatus,*/ general::AccountInfo};
use crate::structures::general::AccountInfo;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateAccInfo {
    pub email_address: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AccountData {
    pub id: String,
    pub email_address: String,
    pub username: String,
    pub password: String,
    // pub address: Option<String>,
    // pub is_driver: Option<FormStatus>,
    // pub is_owner: Option<FormStatus>,
    // pub id_approved: Option<FormStatus>,
}

impl From<AccountData> for AccountInfo {
    fn from(val: AccountData) -> Self {
        Self {
            id: RecordId::from_table_key("tb_users", val.id),
            email_address: val.email_address,
            username: val.username,
            password: val.password,
            // address: val.address,
            // is_driver: val.is_driver,
            // is_owner: val.is_owner,
            // id_approved: val.id_approved,
        }
    }
}
