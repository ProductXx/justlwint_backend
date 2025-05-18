use actix_web::{HttpResponse, get};

use crate::structures::{post_structures::NewFeedStruct, static_vars::DB};

#[get("/newfeed")]
pub async fn get_newfeed_posts() -> HttpResponse {
    // let surql = "SELECT * FROM tb_posts ORDER BY RAND() LIMIT 50;";
    //
    // let mut resul = DB.query(surql).await.unwrap();
    //
    // let feed_posts = resul.take::<Vec<NewFeedStruct>>(0).unwrap();

    HttpResponse::Ok().json(vec![
        NewFeedStruct::default(),
        NewFeedStruct::default(),
        NewFeedStruct::default(),
    ])
}
