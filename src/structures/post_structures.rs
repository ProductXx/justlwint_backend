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
    pub spices: Vec<Spices>, // represents materials/details
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Spices {
    pub images: Vec<String>,       // e.g., fabric samples, tags
    pub texts: Vec<SpicesDetails>, // e.g., material, washing instructions
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpicesDetails {
    pub title: String,   // e.g., "Material"
    pub details: String, // e.g., "100% Cotton"
}

impl Default for SpicesDetails {
    fn default() -> Self {
        Self {
            title: "Material".to_string(),
            details: "100% Cotton".to_string(),
        }
    }
}

impl Default for Spices {
    fn default() -> Self {
        Self {
            images: vec!["fabric_sample.jpg".to_string()],
            texts: vec![
                SpicesDetails {
                    title: "Material".to_string(),
                    details: "100% Cotton".to_string(),
                },
                SpicesDetails {
                    title: "Care".to_string(),
                    details: "Machine wash cold, tumble dry low.".to_string(),
                },
            ],
        }
    }
}

impl Default for NewFeedStruct {
    fn default() -> Self {
        Self {
            id: RecordId::from(("tb_feeds", "default_cloth_post")),
            images: vec!["shirt_front.jpg".to_string(), "shirt_back.jpg".to_string()],
            sizes: vec![
                "S".to_string(),
                "M".to_string(),
                "L".to_string(),
                "XL".to_string(),
            ],
            rating: 4.2,
            discount: 15,
            price: 3499, // e.g., $34.99 if using cents
            description: "Comfortable and stylish everyday wear shirt.".to_string(),
            spices: vec![Spices::default()],
        }
    }
}
