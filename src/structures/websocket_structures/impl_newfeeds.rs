use serde::de::{self, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::fmt;
use surrealdb::{Datetime, RecordId};

use crate::http_apis::posts::PostType;
use crate::structures::websocket_structures::newfeeds::{CarPost, PackagePost};

use super::newfeeds::Post;

// Implement Serialize for Post
impl Serialize for Post {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_map(None)?;
        match self {
            Self::Car(car) => {
                state.serialize_entry("post_type", &PostType::Car)?;
                state.serialize_entry("date", &car.date)?;
                state.serialize_entry("author", &car.author)?;
                state.serialize_entry("author_id", &car.author_id)?;
                state.serialize_entry("title", &car.title)?;
                state.serialize_entry("owner_id", &car.owner_id)?;
                state.serialize_entry("driver_id", &car.driver_id)?;
                state.serialize_entry("car_id", &car.car_id)?;
                state.serialize_entry("description", &car.description)?;
                state.serialize_entry("start_location_detail", &car.start_location_detail)?;
                state.serialize_entry("expected_date_to_start", &car.expected_date_to_start)?;
                state.serialize_entry("start_location", &car.start_location)?;
                state.serialize_entry("end_location_detail", &car.end_location_detail)?;
                state.serialize_entry("end_location", &car.end_location)?;
                state.serialize_entry("post_photos", &car.post_photos)?;
                state.serialize_entry("main_package_types", &car.main_package_types)?;
                state.serialize_entry("cost_per_type", &car.cost_per_type)?;
            }
            Self::Package(package) => {
                state.serialize_entry("post_type", &PostType::Package)?;
                state.serialize_entry("date", &package.date)?;
                state.serialize_entry("title", &package.title)?;
                state.serialize_entry("description", &package.description)?;
                state.serialize_entry("author", &package.author)?;
                state.serialize_entry("author_id", &package.author_id)?;
                state.serialize_entry("start_location_detail", &package.start_location_detail)?;
                state.serialize_entry("start_location", &package.start_location)?;
                state.serialize_entry("end_location_detail", &package.end_location_detail)?;
                state.serialize_entry("end_location", &package.end_location)?;
                state.serialize_entry("expected_date_to_start", &package.expected_date_to_start)?;
                state.serialize_entry("post_photos", &package.post_photos)?;
                state.serialize_entry("cost_per_type", &package.cost_per_type)?;
                state.serialize_entry("main_package_types", &package.main_package_types)?;
            }
        }
        state.end()
    }
}

// Implement Deserialize for Post
#[allow(clippy::too_many_lines)]
impl<'de> Deserialize<'de> for Post {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PostVisitor;

        impl<'de> Visitor<'de> for PostVisitor {
            type Value = Post;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a map representing a CarPost or PackagePost")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Post, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut post_type: Option<PostType> = None;
                let mut date: Option<Datetime> = None;
                let mut title: Option<String> = None;
                let mut description: Option<String> = None;
                let mut author: Option<String> = None;
                let mut author_id: Option<RecordId> = None;
                let mut owner_id: Option<RecordId> = None;
                let mut driver_id: Option<RecordId> = None;
                let mut car_id: Option<RecordId> = None;
                let mut start_location_detail: Option<String> = None;
                let mut start_location: Option<(f64, f64)> = None;
                let mut end_location_detail: Option<String> = None;
                let mut end_location: Option<(f64, f64)> = None;
                let mut expected_date_to_start: Option<Datetime> = None;
                let mut post_photos: Option<Vec<String>> = None;
                let mut main_package_types: Option<Vec<String>> = None;
                let mut cost_per_type: Option<HashMap<String, u64>> = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "post_type" => post_type = Some(map.next_value()?),
                        "date" => date = Some(map.next_value()?),
                        "title" => title = Some(map.next_value()?),
                        "description" => description = Some(map.next_value()?),
                        "author" => author = Some(map.next_value()?),
                        "author_id" => author_id = Some(map.next_value()?),
                        "owner_id" => owner_id = Some(map.next_value()?),
                        "driver_id" => driver_id = Some(map.next_value()?),
                        "car_id" => car_id = Some(map.next_value()?),
                        "start_location_detail" => start_location_detail = Some(map.next_value()?),
                        "start_location" => start_location = Some(map.next_value()?),
                        "end_location_detail" => end_location_detail = Some(map.next_value()?),
                        "end_location" => end_location = Some(map.next_value()?),
                        "expected_date_to_start" => {
                            expected_date_to_start = Some(map.next_value()?);
                        }
                        "post_photos" => post_photos = Some(map.next_value()?),
                        "main_package_types" => main_package_types = Some(map.next_value()?),
                        "cost_per_type" => cost_per_type = Some(map.next_value()?),
                        _ => {
                            return Err(de::Error::unknown_field(
                                &key,
                                &[
                                    "post_type",
                                    "date",
                                    "title",
                                    "description",
                                    "author",
                                    "author_id",
                                    "owner_id",
                                    "driver_id",
                                    "car_id",
                                ],
                            ));
                        }
                    }
                }

                let post_type = post_type.ok_or_else(|| de::Error::missing_field("post_type"))?;

                match post_type {
                    PostType::Car => Ok(Post::Car(CarPost {
                        date: date.ok_or_else(|| de::Error::missing_field("date"))?,
                        author: author.ok_or_else(|| de::Error::missing_field("author"))?,
                        author_id: author_id
                            .ok_or_else(|| de::Error::missing_field("author_id"))?,
                        post_type,
                        title: title.ok_or_else(|| de::Error::missing_field("title"))?,
                        owner_id: owner_id.ok_or_else(|| de::Error::missing_field("owner_id"))?,
                        driver_id: driver_id
                            .ok_or_else(|| de::Error::missing_field("driver_id"))?,
                        car_id: car_id.ok_or_else(|| de::Error::missing_field("car_id"))?,
                        description: description
                            .ok_or_else(|| de::Error::missing_field("description"))?,
                        start_location_detail: start_location_detail
                            .ok_or_else(|| de::Error::missing_field("start_location_detail"))?,
                        expected_date_to_start: expected_date_to_start
                            .ok_or_else(|| de::Error::missing_field("expected_date_to_start"))?,
                        start_location: start_location
                            .ok_or_else(|| de::Error::missing_field("start_location"))?,
                        end_location_detail: end_location_detail
                            .ok_or_else(|| de::Error::missing_field("end_location_detail"))?,
                        end_location: end_location
                            .ok_or_else(|| de::Error::missing_field("end_location"))?,
                        post_photos: post_photos
                            .ok_or_else(|| de::Error::missing_field("post_photos"))?,
                        main_package_types: main_package_types
                            .ok_or_else(|| de::Error::missing_field("main_package_types"))?,
                        cost_per_type: cost_per_type
                            .ok_or_else(|| de::Error::missing_field("cost_per_type"))?,
                    })),
                    PostType::Package => Ok(Post::Package(PackagePost {
                        date: date.ok_or_else(|| de::Error::missing_field("date"))?,
                        title: title.ok_or_else(|| de::Error::missing_field("title"))?,
                        description: description
                            .ok_or_else(|| de::Error::missing_field("description"))?,
                        author: author.ok_or_else(|| de::Error::missing_field("author"))?,
                        author_id: author_id
                            .ok_or_else(|| de::Error::missing_field("author_id"))?,
                        post_type,
                        start_location_detail: start_location_detail
                            .ok_or_else(|| de::Error::missing_field("start_location_detail"))?,
                        start_location: start_location
                            .ok_or_else(|| de::Error::missing_field("start_location"))?,
                        end_location_detail: end_location_detail
                            .ok_or_else(|| de::Error::missing_field("end_location_detail"))?,
                        end_location: end_location
                            .ok_or_else(|| de::Error::missing_field("end_location"))?,
                        expected_date_to_start: expected_date_to_start
                            .ok_or_else(|| de::Error::missing_field("expected_date_to_start"))?,
                        post_photos: post_photos
                            .ok_or_else(|| de::Error::missing_field("post_photos"))?,
                        cost_per_type: ("default".to_string(), 0), // Handle tuple (String, u64)
                        main_package_types: main_package_types
                            .ok_or_else(|| de::Error::missing_field("main_package_types"))?,
                    })),
                }
            }
        }

        deserializer.deserialize_map(PostVisitor)
    }
}
