use std::io;

use actix_cors::Cors;
use actix_files as afs;
use actix_multipart::form::tempfile::TempFileConfig;
use actix_web::{App, HttpServer};

use crate::{
    http_apis::{
        auth::{
            login::{test_fn, token_test},
            scope::auth_scope,
        },
        // booking::scope::booking_scope,
        // driver_drive::scope::driver_drive_scope,
        // fetch_with_id::scope::fetch_with_id_scope,
        // forms::scope::forms_scope,
        // posts::scope::post_scope,
    },
    middlewares::{jwt_middleware::JwtMiddleware, multipart_middleware::MultipartMiddleware},
    structures::static_vars::{HOST_ADDR, POST_PICS_PATH, TEMP_FILE_PATH, UDS_PATH},
};

#[allow(clippy::future_not_send)]
pub async fn actix_services() -> io::Result<()> {
    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .wrap(MultipartMiddleware)
            .wrap(JwtMiddleware)
            .app_data(TempFileConfig::default().directory(&*TEMP_FILE_PATH))
            .service(afs::Files::new("/pics", &*POST_PICS_PATH))
            .service(auth_scope())
            // .service(forms_scope())
            // .service(post_scope())
            // .service(driver_drive_scope())
            // .service(fetch_with_id_scope())
            // .service(booking_scope())
            .service(test_fn)
            .service(token_test)
        // .service(swagger_file_serve)
    })
    .bind_auto_h2c(&*HOST_ADDR)?
    .bind_uds(&*UDS_PATH)?
    .run()
    .await
}

// #[get("/api-docs/openapi.yml")]
// pub async fn swagger_file_serve() -> impl Responder {
//     include_str!("/home/walker/rust/project/kargate_backend/openapi-v2.yml")
// }
