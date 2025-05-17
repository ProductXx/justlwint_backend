use std::sync::LazyLock;

use surrealdb::{Surreal, engine::remote::ws::Client};

// pub static ADMIN_DOMAIN: LazyLock<String> = LazyLock::new(|| dotenvy::var("ADMIN_DOMAIN").unwrap());
#[allow(dead_code)]
pub static CLIENT_DOMAIN: LazyLock<String> =
    LazyLock::new(|| dotenvy::var("CLIENT_DOMAIN").unwrap());
pub static DATA_PATH: LazyLock<String> = LazyLock::new(|| dotenvy::var("DATA_DIR").unwrap());
pub static TEMP_FILE_PATH: LazyLock<String> = LazyLock::new(|| format!("{}/tmp", &*DATA_PATH));
pub static UDS_PATH: LazyLock<String> = LazyLock::new(|| dotenvy::var("UDS_PATH").unwrap());
pub static HOST_ADDR: LazyLock<String> = LazyLock::new(|| dotenvy::var("HOST_ADDR").unwrap());
pub static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);
pub static JWT_SECRET: LazyLock<String> = LazyLock::new(|| dotenvy::var("JWT_SECRET").unwrap());
pub static DRIVER_LICENSE_PATH: LazyLock<String> =
    LazyLock::new(|| format!("{}/driver_licenses", &*DATA_PATH));
pub static USER_PHOTOS_PATH: LazyLock<String> =
    LazyLock::new(|| format!("{}/user_photos", &*DATA_PATH));
pub static CITIZEN_ID_PATH: LazyLock<String> =
    LazyLock::new(|| format!("{}/citizen_ids", &*DATA_PATH));
pub static CAR_PROOFS_PATH: LazyLock<String> =
    LazyLock::new(|| format!("{}/car_photos", &*DATA_PATH));
pub static CAR_PHOTOS_PATH: LazyLock<String> =
    LazyLock::new(|| format!("{}/car_proofs", &*DATA_PATH));
pub static POST_PICS_PATH: LazyLock<String> =
    LazyLock::new(|| format!("{}/post_pics", &*DATA_PATH));
pub static DB_BACKUP_PATH: LazyLock<String> =
    LazyLock::new(|| format!("{}/database_backup", &*DATA_PATH));

pub static SMTP_USERNAME: LazyLock<String> =
    LazyLock::new(|| dotenvy::var("SMTP_USERNAME").unwrap());
pub static SMTP_PASSWD: LazyLock<String> = LazyLock::new(|| dotenvy::var("SMTP_PASSWD").unwrap());
pub static SMTP_RELAY_ADDR: LazyLock<String> =
    LazyLock::new(|| dotenvy::var("SMTP_RELAY_ADDR").unwrap());
