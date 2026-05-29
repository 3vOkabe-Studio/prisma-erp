use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CashMovement {
    pub id: i64,
    pub transaction_type: String, // 'OPEN', 'CLOSE', 'INCOME', 'EXPENSE'
    pub amount: f64,
    pub notes: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCashMovement {
    pub transaction_type: String,
    pub amount: f64,
    pub notes: Option<String>,
}
