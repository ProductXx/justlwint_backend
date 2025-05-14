use actix_multipart::form::MultipartForm;
use actix_web::{HttpMessage, Result, dev::ServiceRequest};
use chrono::{NaiveDate, Utc};

use crate::{
    http_apis::posts::PostType,
    structures::{
        auth_structures::login_structures::Claims,
        general::AccountInfo,
        posts_structures::post_request_structures::package_post_request_structures::{
            PackagePostForm, PackageUploadPost,
        },
        static_vars::{CLIENT_DOMAIN, DB, POST_PICS_PATH},
    },
};

use super::utils::{
    fields_parsers::{text_to_string_f64, text_to_two_vec_f64, text_to_vec_string},
    save_temps_files::save_temps_files,
};

#[allow(clippy::await_holding_refcell_ref)]
#[allow(clippy::future_not_send)]
pub async fn package_post_multipath(req: &mut ServiceRequest) -> Result<()> {
    if let Ok(package_post) = req.extract::<MultipartForm<PackagePostForm>>().await {
        let mut extensions = req.extensions_mut();
        let Some(userinfo) = extensions.get::<Claims>() else {
            return Err(actix_web::error::ErrorUnauthorized("No JWT authed"));
        };

        // if !userinfo.id_approved {
        //     return Err(actix_web::error::ErrorConflict("User is not verified!"));
        // }

        let userselect = DB
            .select::<Option<AccountInfo>>(&userinfo.id)
            .await
            .unwrap();

        if userselect.is_none() {
            return Err(actix_web::error::ErrorUnauthorized("User Not found"));
        }

        let post_photos = save_temps_files(
            &package_post.post_photos,
            &userinfo.username,
            &POST_PICS_PATH,
            &format!("{}/post_pics", &*CLIENT_DOMAIN),
            3,
            5,
        )
        .await?;

        let naive_date =
            NaiveDate::parse_from_str(package_post.expected_date_to_start.as_str(), "%d/%m/%Y")
                .unwrap();

        let datetime = naive_date.and_hms_opt(0, 0, 0).unwrap();
        let end_location = text_to_two_vec_f64(&package_post.end_location)?;
        let start_location = text_to_two_vec_f64(&package_post.start_location)?;
        let cost_per_type = text_to_string_f64(&package_post.cost_per_type)?;
        let main_package_types = text_to_vec_string(&package_post.main_package_types)?;

        let package_post_struct = PackageUploadPost {
            date: Utc::now().into(),
            post_type: PostType::Package,
            author: userinfo.username.clone(),
            author_id: userinfo.id.clone(),
            title: package_post.title.clone(),
            description: package_post.description.clone(),
            end_location_detail: package_post.end_location_detail.clone(),
            end_location,
            start_location_detail: package_post.start_location_detail.clone(),
            start_location,
            expected_date_to_start: datetime.and_utc().into(),
            post_photos,
            cost_per_type,
            main_package_types,
            available: true,
        };

        extensions.insert(package_post_struct);
    }
    Ok(())
}
