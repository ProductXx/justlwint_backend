use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewFeedStruct {
    pub id: RecordId,
    pub images: Vec<String>,
    pub sizes: Vec<String>,
    pub rating: f32,
    pub discount: u32,
    pub price: u32,
    pub description: String,
    pub spices: Vec<Spices>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Spices {
    pub images: Vec<String>,
    pub texts: Vec<SpicesDetails>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpicesDetails {
    pub title: String,
    pub details: String,
}
