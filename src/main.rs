use std::{io, path::Path};

use actix_web::rt;
use servers::{actix_server::actix_services, axum_server::axum_services};
use structures::static_vars::{
    CAR_PHOTOS_PATH, CAR_PROOFS_PATH, CITIZEN_ID_PATH, DB, DB_BACKUP_PATH, DRIVER_LICENSE_PATH,
    POST_PICS_PATH, TEMP_FILE_PATH, USER_PHOTOS_PATH,
};
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root};
use tokio::fs;
use tracing::Level;

mod http_apis;
mod middlewares;
mod servers;
mod structures;
mod websocket_apis;

fn surql_queries() -> String {
    include_str!("surrealql/indexes.surql").to_owned()

    // format!(
    //     "{} \n {} \n {} \n {} \n {} \n {} \n {} \n {} \n {} \n {} \n {} \n",
    //     include_str!("surrealql/indexes.surql"),
    //     include_str!("surrealql/user_events/forms/car_form/car_form_status_event.surql"),
    //     include_str!("surrealql/user_events/forms/car_form/relate_car_form_event.surql"),
    //     include_str!("surrealql/user_events/forms/driver_form/driver_form_status_event.surql"),
    //     include_str!("surrealql/user_events/forms/driver_form/relate_driver_form_event.surql"),
    //     include_str!(
    //         "surrealql/user_events/forms/id_verify_form/id_verify_form_status_event.surql"
    //     ),
    //     include_str!(
    //         "surrealql/user_events/forms/id_verify_form/relate_id_verify_form_event.surql"
    //     ),
    //     include_str!("surrealql/user_events/posts/car_post/car_post_status.surql"),
    //     include_str!("surrealql/user_events/posts/car_post/relate_car_post_event.surql"),
    //     include_str!("surrealql/user_events/posts/general.surql"),
    //     include_str!("surrealql/user_events/booking/listen_create_event.surql")
    // )
}

async fn init_important_dirs() -> io::Result<()> {
    let dirs = [
        &*DRIVER_LICENSE_PATH,
        &*CAR_PROOFS_PATH,
        &*DB_BACKUP_PATH,
        &*POST_PICS_PATH,
        &*TEMP_FILE_PATH,
        &*USER_PHOTOS_PATH,
        &*CITIZEN_ID_PATH,
        &*CAR_PHOTOS_PATH,
    ];

    for dir_name in dirs {
        if !Path::new(dir_name).exists() {
            fs::create_dir_all(dir_name).await?;
        }
    }

    Ok(())
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    color_eyre::install().unwrap();
    tracing_subscriber::fmt()
        .with_max_level(Level::ERROR)
        .pretty()
        .with_ansi(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    DB.connect::<Ws>(dotenvy::var("SURREALDB_ADDR").unwrap())
        .await
        .unwrap();

    // DB.connect::<Mem>(()).await.unwrap();

    DB.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .unwrap();

    DB.use_ns("fuckkk").use_db("justlwint").await.unwrap();

    DB.query(surql_queries()).await.unwrap();

    init_important_dirs().await?;

    rt::spawn(actix_services());

    tokio::spawn(axum_services());

    // tokio::spawn(swagger_services());

    tokio::signal::ctrl_c().await.unwrap();

    Ok(())
}
