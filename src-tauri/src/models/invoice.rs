use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Invoice {
    pub id: i64,
    pub customer_id: Option<i64>,
    pub subtotal: f64,
    pub tax: f64,
    pub discount: f64,
    pub total: f64,
    pub status: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct CreateInvoiceItem {
    pub product_id: i64,
    pub quantity: f64,
    pub price: f64,
    pub subtotal: f64,
}

#[derive(Debug, Deserialize)]
pub struct CreateInvoice {
    pub customer_id: Option<i64>,
    pub subtotal: f64,
    pub tax: f64,
    pub discount: f64,
    pub total: f64,
    pub status: Option<String>,
    pub items: Vec<CreateInvoiceItem>,
}
