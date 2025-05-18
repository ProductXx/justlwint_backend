use actix_web::{HttpResponse, post, rt, web::Json};
use lettre::{
    Message, SmtpTransport, Transport, message::header::ContentType,
    transport::smtp::authentication::Credentials,
};
// use argon2::{Config, hash_encoded};
use rand::random_range;
// use serde_json::json;
use surrealdb::{Error, RecordId, error::Api};
use tracing::error;
// use uuid::Uuid;

use crate::{
    // http_apis::utils::token::generate_token,
    http_apis::utils::token::generate_token,
    structures::{
        auth_structures::sign_up_structures::{AccountData, CreateAccInfo},
        static_vars::{DB, SMTP_PASSWD, SMTP_RELAY_ADDR, SMTP_USERNAME},
    },
};

// pub fn generate_random_salt(min_len: usize, max_len: usize) -> Vec<u8> {
//     let mut rng = rng();
//     let len = rng.random_range(min_len..=max_len);
//     let mut salt = vec![0u8; len];
//     rng.fill(&mut salt[..]);
//     salt
// }

#[post("/create_acc")]
pub async fn create_acc(account_info: Json<CreateAccInfo>) -> HttpResponse {
    // let uuid = Uuid::new_v4().simple().to_string();

    // let mv_passwd = account_info.password.clone();
    //
    // let hashed_passwd = tokio::task::spawn_blocking(move || {
    //     let salt = generate_random_salt(16, 32);
    //
    //     hash_encoded(mv_passwd.as_bytes(), &salt, &Config::default()).unwrap()
    // });

    let accinfo = AccountData {
        id: account_info.0.username.clone(),
        email_address: account_info.0.email_address,
        username: account_info.0.username,
        password: account_info.0.password,
    };

    let accinfo2 = accinfo.clone();

    let token_check1 = tokio::task::spawn_blocking(move || generate_token(accinfo2.into()));
    let otp = random_range(100_000..1_000_000);

    // let surql_cusr = "SELECT * FROM tb_users WHERE (id IS type::thing($uid) OR email_address IS type::string($email_address));";

    // let surql_cusr = r#"
    //     IF $uid.username {
    //        THROW "Username already exists";
    //     } ELSE {
    //
    //     }
    // "#;

    let vsel = DB
        .select::<Option<AccountData>>(RecordId::from_table_key("tb_users", &accinfo.username))
        .await
        .unwrap();

    if vsel.is_some() {
        return HttpResponse::NotAcceptable().json("Username already exists");
    }

    let token_check2 = match token_check1.await {
        Ok(token) => token,
        Err(shits) => {
            error!("{shits:?}");
            return HttpResponse::InternalServerError().finish();
        }
    };

    let token = match token_check2 {
        Ok(token) => token,
        Err(shits) => {
            error!("{shits:?}");
            return HttpResponse::InternalServerError().finish();
        }
    };

    let sql = r#"
        UPSERT type::thing("tb_users_verify", $accinfo.id) SET email_address = $accinfo.email_address, username = $accinfo.username, password = crypto::argon2::generate($accinfo.password), otp_code = $otp_code, exp = time::now() + 10m, jwt_token = $jwt_token;
    "#;

    let result = DB
        .query(sql)
        .bind(("accinfo", accinfo.clone()))
        .bind(("otp_code", otp))
        .bind(("jwt_token", token))
        .await
        .unwrap();

    let check_result = result.check();

    if let Err(Error::Api(Api::Query(shits))) = check_result {
        return HttpResponse::BadRequest().json(shits);
    }

    if let Err(shits) = check_result {
        error!("{shits:?}");
        return HttpResponse::InternalServerError().finish();
    }

    let accinfo2 = accinfo.clone();

    rt::task::spawn_blocking(move || send_email(&accinfo2, otp));

    HttpResponse::Ok().json(format!("/auth/verify/{}", accinfo.username))

    // let token_result = token.await.map_err(|shits| {
    //     error!("{shits:?}");
    // });
    //
    // let Ok(token) = token_result else {
    //     return HttpResponse::InternalServerError().finish();
    // };

    // match token {
    //     Ok(token) => {
    //         // let sign_up_body = json!({
    //         //     "token": token,
    //         //     "user_info": accinfo
    //         // });
    //         // HttpResponse::Ok().json(sign_up_body)
    //
    //     }
    //     Err(shits) => {
    //         error!("{shits:?}");
    //         HttpResponse::InternalServerError().finish()
    //     }
    // }
}

fn send_email(accinfo: &AccountData, otp: i32) {
    let email = Message::builder()
        .from("Verify <verify@justlwint.com>".parse().unwrap())
        .to(
            format!("{} <{}>", accinfo.username, accinfo.email_address.trim())
                .parse()
                .unwrap(),
        )
        .subject("Verify Code for Justlwint")
        .header(ContentType::TEXT_PLAIN)
        .body(format!(
            "Your otp code for Justlwint account verification is {otp}"
        ))
        .unwrap();

    let creds = Credentials::new(SMTP_USERNAME.to_owned(), SMTP_PASSWD.to_owned());

    let smtp_relay = SmtpTransport::relay(&SMTP_RELAY_ADDR)
        .unwrap()
        .credentials(creds)
        .build();

    match smtp_relay.send(&email) {
        Ok(_) => {}
        Err(shits) => {
            error!("{shits:?}");
        }
    }
}
