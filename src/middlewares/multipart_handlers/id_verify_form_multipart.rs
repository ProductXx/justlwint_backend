use actix_multipart::form::MultipartForm;
use actix_web::{HttpMessage, Result, dev::ServiceRequest};

use crate::structures::{
    auth_structures::login_structures::Claims,
    forms_structures::{
        FormStatus,
        id_verify_structures::{IdVerifyMultipartForm, IdVerifyRegistrationForm},
    },
    general::AccountInfo,
    static_vars::{ADMIN_DOMAIN, CITIZEN_ID_PATH, DB, USER_PHOTOS_PATH},
};

use super::utils::save_temps_files::save_temps_files;

#[allow(clippy::await_holding_refcell_ref)]
#[allow(clippy::future_not_send)]
pub async fn id_verify_form_multipath(req: &mut ServiceRequest) -> Result<()> {
    if let Ok(id_verify_form) = req.extract::<MultipartForm<IdVerifyMultipartForm>>().await {
        let mut extensions = req.extensions_mut();
        let Some(userinfo) = extensions.get::<Claims>() else {
            return Err(actix_web::error::ErrorUnauthorized("No JWT authed"));
        };

        // if userinfo.id_approved {
        //     return Err(actix_web::error::ErrorConflict("User is already verified!"));
        // }

        let userselect = DB
            .select::<Option<AccountInfo>>(&userinfo.id)
            .await
            .unwrap();

        if userselect.is_none() {
            return Err(actix_web::error::ErrorUnauthorized("User Not found"));
        }

        let id_photos = save_temps_files(
            &id_verify_form.id_photos,
            &userinfo.username,
            &CITIZEN_ID_PATH,
            &format!("{}/citizen_ids", &*ADMIN_DOMAIN),
            2,
            2,
        )
        .await?;

        let user_photos = save_temps_files(
            &id_verify_form.user_photos,
            &userinfo.username,
            &USER_PHOTOS_PATH,
            &format!("{}/user_photos", &*ADMIN_DOMAIN),
            2,
            5,
        )
        .await?;

        let id_verify_form_struct = IdVerifyRegistrationForm {
            user_id: userinfo.id.clone(),
            fullname: id_verify_form.fullname.to_string(),
            phone_number: userinfo.phone_number.to_string(),
            id_photos,
            user_photos,
            id_number: id_verify_form.id_number.to_string(),
            // birthdate: id_verify_form.birthdate.to_string(),
            address: id_verify_form.address.to_string(),
            status_type: FormStatus::Pending,
            message: None,
        };

        extensions.insert(id_verify_form_struct);
    }
    Ok(())
}
