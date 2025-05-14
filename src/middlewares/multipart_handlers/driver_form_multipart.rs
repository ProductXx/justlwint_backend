use actix_multipart::form::MultipartForm;
use actix_web::{HttpMessage, Result, dev::ServiceRequest};

use crate::structures::{
    auth_structures::login_structures::Claims,
    forms_structures::{
        FormStatus,
        driver_form_structures::{DriverMultipartForm, DriverRegistrationForm},
    },
    general::AccountInfo,
    static_vars::{ADMIN_DOMAIN, DB, DRIVER_LICENSE_PATH},
};

use super::utils::save_temps_files::save_temps_files;

#[allow(clippy::await_holding_refcell_ref)]
#[allow(clippy::future_not_send)]
pub async fn driver_form_multipath(req: &mut ServiceRequest) -> Result<()> {
    if let Ok(driver_form) = req.extract::<MultipartForm<DriverMultipartForm>>().await {
        let mut extensions = req.extensions_mut();
        let Some(userinfo) = extensions.get::<Claims>() else {
            return Err(actix_web::error::ErrorUnauthorized("No JWT authed"));
        };

        // if !userinfo.id_approved {
        //     return Err(actix_web::error::ErrorConflict("User is not verified!"));
        // }

        if let Some(is_driver) = &userinfo.is_driver {
            if is_driver != &FormStatus::Rejected {
                return Err(actix_web::error::ErrorConflict("User is already a driver"));
            }
        }

        let userselect = DB
            .select::<Option<AccountInfo>>(&userinfo.id)
            .await
            .unwrap();

        if userselect.is_none() {
            return Err(actix_web::error::ErrorUnauthorized("User Not found"));
        }

        let license_photos = save_temps_files(
            &driver_form.license_photos,
            &userinfo.username,
            &DRIVER_LICENSE_PATH,
            &format!("{}/license_photos", &*ADMIN_DOMAIN),
            2,
            4,
        )
        .await?;

        let driver_form_struct = DriverRegistrationForm {
            user_id: userinfo.id.clone(),
            fullname: driver_form.fullname.to_string(),
            phone_number: driver_form.phone_number.to_string(),
            license_photos,
            license_number: driver_form.license_number.clone(),
            experience_details: driver_form.experience_details.clone(),
            status_type: FormStatus::Pending,
            message: None,
        };

        extensions.insert(driver_form_struct);
    }
    Ok(())
}
