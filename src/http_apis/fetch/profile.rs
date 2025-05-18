use actix_web::{HttpResponse, get, web::Path};
use serde::{Deserialize, Serialize};

use crate::structures::{
    auth_structures::sign_up_structures::AccountData, post_structures::NewFeedStruct,
    static_vars::DB,
};

#[derive(Serialize, Deserialize)]
pub struct Profile {
    pub username: String,
    pub email_address: String,
    pub purchases: Vec<NewFeedStruct>,
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            username: "Thant Zin Gay".to_string(),
            email_address: "vixxgrego@kargate.site".to_string(),
            purchases: vec![
                NewFeedStruct::default(),
                NewFeedStruct::default(),
                NewFeedStruct::default(),
            ],
        }
    }
}

#[get("/user/{uid}")]
pub async fn get_user_profile(uid: Path<String>) -> HttpResponse {
    // let surql = "SELECT * FROM type::thing($uid) LIMIT 50;";
    // let userinfo = r#"SELECT * FROM type::thing("tb_users", $uid);"#;
    //
    // let mut resul = DB
    //     .query(surql)
    //     .query(userinfo)
    //     .bind(("uid", uid.into_inner()))
    //     .await
    //     .unwrap();

    // let feed_posts = resul.take::<Vec<NewFeedStruct>>(0).unwrap();
    // let userinfos = resul.take::<Option<AccountData>>(1).unwrap().unwrap();

    let profile = Profile::default();

    HttpResponse::Ok().json(profile)
}
