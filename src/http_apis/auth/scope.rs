use actix_web::{Scope, web};

use super::{login::login_fn, sign_up::create_acc, verify_otp::verify_otp};

pub fn auth_scope() -> Scope {
    web::scope("/auth")
        .service(login_fn)
        .service(create_acc)
        // .service(token_test)
        .service(verify_otp)
}
