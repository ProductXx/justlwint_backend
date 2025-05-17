use actix_web::{
    HttpResponse, put,
    web::{Json, Path},
};
use serde::{Deserialize, Serialize};
use surrealdb::{Error, error::Api};
use tracing::error;

use crate::structures::static_vars::DB;

#[derive(Serialize, Deserialize)]
struct OtpReq {
    pub otp_code: String,
}

#[derive(Serialize, Deserialize)]
struct VReturn {
    token: String,
}

#[put("/verify/{uid}")]
pub async fn verify_otp(uid: Path<String>, otp_code: Json<OtpReq>) -> HttpResponse {
    let surql = r#"
        BEGIN;

        LET $user = SELECT * FROM type::thing("tb_users_verify", $uid);

        IF !$user.username {
            THROW "Sorry there is no otp for requested username";
        } ELSE IF $user.exp < ( time::now() - 10m ) {
            THROW "Sorry the otp code is expired and will be remove";
        } ELSE IF type::string($user.otp) IS $otp_code {
            LET $user_acc_content = {
                email_address: $user.email_address,
                username: $user.username,
                password: $user.password,
            };

            CREATE type::thing("tb_users", $user.id.id) CONTENT user_acc_content;

            DELETE $user.id;

            LET $user_token_content = {
                token: $user.token
            };

            RETURN user_token_content;
        }

        COMMIT;
    "#;

    let Ok(resul) = DB
        .query(surql)
        .bind(("uid", uid.into_inner()))
        .bind(("otp_code", otp_code.otp_code.clone()))
        .await
    else {
        return HttpResponse::InternalServerError().finish();
    };

    match resul.check() {
        Ok(mut resp) => {
            let vtoken = resp.take::<Option<VReturn>>(0).unwrap().unwrap();
            HttpResponse::Ok().json(vtoken.token)
        }
        Err(Error::Api(Api::Query(shits))) => HttpResponse::BadRequest().json(shits),
        Err(shits) => {
            error!("{shits:?}");
            HttpResponse::InternalServerError().finish()
        }
    }

    // if let Err(Error::Api(Api::Query(shits))) = check_result {}
    //
    // if let Err(shits) = check_result {
    //     error!("{shits:?}");
    //     return HttpResponse::InternalServerError().finish();
    // }
    //
    // todo!()
}
