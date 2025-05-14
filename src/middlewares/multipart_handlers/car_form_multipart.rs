use actix_multipart::form::MultipartForm;
use actix_web::{HttpMessage, Result, dev::ServiceRequest};

use crate::structures::{
    auth_structures::login_structures::Claims,
    forms_structures::{
        FormStatus,
        car_form_structures::{CarMultipartForm, CarRegistrationForm},
    },
    general::AccountInfo,
    static_vars::{ADMIN_DOMAIN, CAR_PHOTOS_PATH, CAR_PROOFS_PATH, DB},
};

use super::utils::save_temps_files::save_temps_files;

#[allow(clippy::await_holding_refcell_ref)]
#[allow(clippy::future_not_send)]
pub async fn car_form_multipath(req: &mut ServiceRequest) -> Result<()> {
    if let Ok(car_form) = req.extract::<MultipartForm<CarMultipartForm>>().await {
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

        let car_photos = save_temps_files(
            &car_form.car_photos,
            &userinfo.username,
            &CAR_PHOTOS_PATH,
            &format!("{}/car_photos", &*ADMIN_DOMAIN),
            3,
            5,
        )
        .await?;

        let owner_proofs = save_temps_files(
            &car_form.owner_proofs,
            &userinfo.username,
            &CAR_PROOFS_PATH,
            &format!("{}/car_proofs", &*ADMIN_DOMAIN),
            2,
            5,
        )
        .await?;

        let car_form_struct = CarRegistrationForm {
            user_id: userinfo.id.clone(),
            model: car_form.model.to_string(),
            details: car_form.details.to_string(),
            license_number: car_form.license_number.to_string(),
            car_photos,
            owner_proofs,
            status_type: FormStatus::Pending,
            message: None,
        };

        extensions.insert(car_form_struct);
    }
    Ok(())
}
