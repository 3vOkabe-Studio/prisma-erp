use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Expense {
    pub id: i64,
    pub category: String,
    pub amount: f64,
    pub notes: Option<String>,
    pub date: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct CreateExpense {
    pub category: String,
    pub amount: f64,
    pub notes: Option<String>,
}
