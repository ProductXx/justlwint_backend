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
        email_address: account_info.0.email_address,
        username: account_info.0.username,
        password: account_info.0.password,
    };

    let accinfo2 = accinfo.clone();

    let token = tokio::task::spawn_blocking(move || generate_token(accinfo2.into()));

    let sql = r"
        CREATE tb_users SET email_address = $accinfo.email_address, username = $accinfo.username, password = crypto::argon2::generate($accinfo.password);
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
