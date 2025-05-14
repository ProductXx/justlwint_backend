use actix_web::{HttpResponse, post, web::Json};
// use argon2::{Config, hash_encoded};
use rand::{Rng, rng};
use serde_json::json;
use surrealdb::{Error, error::Api};
use tracing::error;
use uuid::Uuid;

use crate::{
    http_apis::utils::token::generate_token,
    structures::{
        auth_structures::sign_up_structures::{AccountData, CreateAccInfo},
        static_vars::DB,
    },
};

pub fn generate_random_salt(min_len: usize, max_len: usize) -> Vec<u8> {
    let mut rng = rng();
    let len = rng.random_range(min_len..=max_len);
    let mut salt = vec![0u8; len];
    rng.fill(&mut salt[..]);
    salt
}

#[post("/create_acc")]
pub async fn create_acc(account_info: Json<CreateAccInfo>) -> HttpResponse {
    let uuid = Uuid::new_v4().simple().to_string();

    // let mv_passwd = account_info.password.clone();
    //
    // let hashed_passwd = tokio::task::spawn_blocking(move || {
    //     let salt = generate_random_salt(16, 32);
    //
    //     hash_encoded(mv_passwd.as_bytes(), &salt, &Config::default()).unwrap()
    // });

    let accinfo = AccountData {
        id: uuid,
        phone_number: account_info.0.phone_number,
        username: account_info.0.username,
        password: account_info.0.password,
        address: None,
        is_owner: None,
        is_driver: None,
        id_approved: None,
    };

    let accinfo2 = accinfo.clone();

    let token = tokio::task::spawn_blocking(move || generate_token(accinfo2.into()));

    // let sql = r#"
    //     LET $is_user_exits = SELECT VALUE phone_number FROM ONLY tb_users WITH INDEX user_phone_number_index WHERE (phone_number IS $accinfo.phone_number) LIMIT 1;
    //
    //     IF !$is_user_exits THEN {
    //         CREATE tb_users SET phone_number = $accinfo.phone_number, username = $accinfo.username, password = crypto::argon2::generate($accinfo.password), is_driver = false, is_owner = false, id_approved = false, address = $accinfo.address;
    //     } ELSE {
    //         THROW string::concat("User with phone numer ", $is_user_exits, " is already exists");
    //     } END;
    // "#;

    let sql = r"
        CREATE tb_users SET phone_number = $accinfo.phone_number, username = $accinfo.username, password = crypto::argon2::generate($accinfo.password), is_driver = $accinfo.is_driver, is_owner = $accinfo.is_owner, id_approved = $accinfo.id_approved, address = $accinfo.address;
    ";

    let Ok(result) = DB.query(sql).bind(("accinfo", accinfo.clone())).await else {
        return HttpResponse::InternalServerError().finish();
    };

    let check_result = result.check();

    if let Err(Error::Api(Api::Query(shits))) = check_result {
        return HttpResponse::BadRequest().json(shits);
    }

    if let Err(shits) = check_result {
        error!("{shits:?}");
        return HttpResponse::InternalServerError().finish();
    }

    let token_result = token.await.map_err(|shits| {
        error!("{shits:?}");
    });

    let Ok(token) = token_result else {
        return HttpResponse::InternalServerError().finish();
    };

    match token {
        Ok(token) => {
            let sign_up_body = json!({
                "token": token,
                "user_info": accinfo
            });
            HttpResponse::Ok().json(sign_up_body)
        }
        Err(shits) => {
            error!("{shits:?}");
            HttpResponse::InternalServerError().finish()
        }
    }
}
