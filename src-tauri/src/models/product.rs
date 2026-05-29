use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub sku: Option<String>,
    pub barcode: Option<String>,
    pub category: Option<String>,
    pub current_stock: f64,
    pub min_stock: f64,
    pub cost: f64,
    pub price: f64,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct CreateProduct {
    pub name: String,
    pub sku: Option<String>,
    pub barcode: Option<String>,
    pub category: Option<String>,
    pub current_stock: f64,
    pub min_stock: f64,
    pub cost: f64,
    pub price: f64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProduct {
    pub id: i64,
    pub name: String,
    pub sku: Option<String>,
    pub barcode: Option<String>,
    pub category: Option<String>,
    pub current_stock: f64,
    pub min_stock: f64,
    pub cost: f64,
    pub price: f64,
}
