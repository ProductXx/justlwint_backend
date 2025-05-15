use actix_web::{HttpResponse, get, web::Path};

use crate::structures::{post_structures::NewFeedStruct, static_vars::DB};

#[get("/user/{uid}")]
pub async fn get_user_profile(uid: Path<String>) -> HttpResponse {
    let surql = "SELECT * FROM type::thing($uid) LIMIT 50;";

    let mut resul = DB
        .query(surql)
        .bind(("uid", uid.into_inner()))
        .await
        .unwrap();

    let feed_posts = resul.take::<Vec<NewFeedStruct>>(0).unwrap();

    HttpResponse::Ok().json(feed_posts)
}
