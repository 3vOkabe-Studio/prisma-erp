use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct InventoryMovement {
    pub id: i64,
    pub product_id: i64,
    pub product_name: String,
    pub movement_type: String, // 'IN', 'OUT', 'ADJUST'
    pub quantity: f64,
    pub notes: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
}
