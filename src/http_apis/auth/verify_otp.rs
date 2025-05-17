use actix_web::{
    HttpResponse, put,
    web::{Json, Path},
};
use serde::{Deserialize, Serialize};

use crate::structures::static_vars::DB;

#[derive(Serialize, Deserialize)]
struct OtpReq {
    pub otp_code: String,
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

            RETURN user_acc_content;
        }

        COMMIT;
    "#;

    let resul = DB
        .query(surql)
        .bind(("uid", uid.into_inner()))
        .bind(("otp_code", otp_code.otp_code.clone()))
        .await
        .unwrap();
    todo!()
}
