use actix_web::{Scope, web};

use super::{
    login::{login_fn, token_test},
    sign_up::create_acc,
};

pub fn auth_scope() -> Scope {
    web::scope("/auth")
        .service(login_fn)
        .service(create_acc)
        .service(token_test)
}
