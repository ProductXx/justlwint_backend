use actix_web::{
    HttpResponse, get, post,
    web::{self, Json},
};
use chrono::{TimeDelta, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde_json::json;
use surrealdb::{Error, error::Api};
use tracing::error;

use crate::structures::{
    auth_structures::login_structures::{Claims, LoginInfo},
    general::AccountInfo,
    static_vars::{DB, JWT_SECRET},
};

#[post("/login")]
pub async fn login_fn(login_info: Json<LoginInfo>) -> HttpResponse {
    let query = "SELECT * FROM tb_users WHERE email_address = $email_address AND crypto::argon2::compare(password, $password)";
    let Ok(select) = DB.query(query).bind(login_info.into_inner()).await else {
        return HttpResponse::InternalServerError().finish();
    };

    let mut select = match select.check() {
        Ok(response) => response,
        Err(Error::Api(Api::Query(shits))) => {
            return HttpResponse::BadRequest().json(shits);
        }
        Err(shits) => {
            error!("{shits:?}");
            return HttpResponse::InternalServerError().finish();
        }
    };

    let Some(user) = select.take::<Option<AccountInfo>>(0).unwrap() else {
        return HttpResponse::NotFound()
            .json("Sorry user with that email address or password not exists!");
    };

    let exp = usize::try_from((Utc::now() + TimeDelta::try_days(9_999_999).unwrap()).timestamp())
        .unwrap();

    let user2 = user.clone();

    let claims = Claims {
        exp,
        id: user.id,
        email_address: user.email_address,
        username: user.username,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )
    .unwrap();

    let token = json! ({
        "token": token,
        "user_info": user2
    });

    HttpResponse::Ok().json(token)
}

#[get("/token")]
pub async fn token_test(claims: web::ReqData<Claims>) -> HttpResponse {
    println!("Hello, {}", claims.username);
    HttpResponse::Ok().json(format!("Hello, {}!", claims.username))
}

// #[get("test")]
// pub async fn test_fn() -> HttpResponse {
//     #[derive(Serialize, Deserialize)]
//     struct Test {
//         username: String,
//     }
//     let sql = r"
//         BEGIN TRANSACTION;
//
//         CREATE user:walker SET username = 'walker';
//         CREATE post:idk SET title = 'wtf';
//
//         RELATE user:walker -> upload -> post:idk;
//
//         SELECT *, -> upload -> post FROM user;
//
//         COMMIT TRANSACTION;
//     ";
//     let mut query = DB.query(sql).await.unwrap();
//
//     let idk = query.take::<Vec<Test>>(3).unwrap();
//     HttpResponse::Ok().json(idk)
// }
