use actix_web::{Scope, web};

use super::{get_nf_posts::get_newfeed_posts, profile::get_user_profile};

pub fn fetch_scope() -> Scope {
    web::scope("/fetch")
        .service(get_newfeed_posts)
        .service(get_user_profile)
}
