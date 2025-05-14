use actix_multipart::form::tempfile::TempFile;
use tokio::fs;
use uuid::Uuid;

pub async fn save_temps_files(
    files: &Vec<TempFile>,
    username: &str,
    root_path: &str,
    base_url_with_route_path: &str,
    min_pics: usize,
    max_pics: usize,
) -> actix_web::Result<Vec<String>> {
    if files.len() < min_pics {
        return Err(actix_web::error::ErrorBadRequest(format!(
            "Photos should be more than {min_pics}"
        )));
    }
    if files.len() > max_pics {
        return Err(actix_web::error::ErrorPayloadTooLarge(format!(
            "Photos are more than {max_pics}"
        )));
    }

    let mut pic_url_links = vec![];

    for photo in files {
        let Some(ref content_type) = photo.content_type else {
            return Err(actix_web::error::ErrorBadRequest("Content Type Not Found"));
        };

        if content_type.type_() != mime::IMAGE {
            return Err(actix_web::error::ErrorUnsupportedMediaType(
                "Unsupported file type",
            ));
        }

        let Some(ref photo_name) = photo.file_name else {
            return Err(actix_web::error::ErrorBadRequest("File Name Not Found"));
        };

        let file_name = format!(
            "{}-{}-{}-{}",
            root_path,
            username,
            Uuid::new_v4(),
            photo_name
        );

        fs::rename(photo.file.path(), &file_name).await.unwrap();

        pic_url_links.push(format!("{base_url_with_route_path}/{file_name}"));
    }
    Ok(pic_url_links)
}
