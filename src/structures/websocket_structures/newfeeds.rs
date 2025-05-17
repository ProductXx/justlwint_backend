use std::collections::HashMap;

use surrealdb::{Datetime, RecordId};

use crate::http_apis::posts::PostType;

pub enum Post {
    Car(CarPost),
    Package(PackagePost),
}

#[allow(dead_code)]
pub struct CarPost {
    pub date: Datetime,
    pub author: String,
    pub author_id: RecordId,
    pub post_type: PostType,
    pub title: String,
    pub owner_id: RecordId,
    pub driver_id: RecordId,
    pub car_id: RecordId,
    pub description: String,
    pub start_location_detail: String,
    pub expected_date_to_start: Datetime,
    pub start_location: (f64, f64),
    pub end_location_detail: String,
    pub end_location: (f64, f64),
    pub post_photos: Vec<String>,
    pub main_package_types: Vec<String>,
    pub cost_per_type: HashMap<String, u64>,
}

#[allow(dead_code)]
pub struct PackagePost {
    pub date: Datetime,
    pub title: String,
    pub description: String,
    pub author: String,
    pub author_id: RecordId,
    pub post_type: PostType,
    pub start_location_detail: String,
    pub start_location: (f64, f64),
    pub end_location_detail: String,
    pub end_location: (f64, f64),
    pub expected_date_to_start: Datetime,
    pub post_photos: Vec<String>,
    pub cost_per_type: (String, u64),
    pub main_package_types: Vec<String>,
}
