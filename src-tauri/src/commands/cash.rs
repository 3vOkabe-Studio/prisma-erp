use sqlx::SqlitePool;
use tauri::State;
use crate::models::cash::{CashMovement, CreateCashMovement};

#[tauri::command]
pub async fn get_cash_movements(pool: State<'_, SqlitePool>) -> Result<Vec<CashMovement>, String> {
    sqlx::query_as::<_, CashMovement>("SELECT * FROM cash_register ORDER BY id DESC")
        .fetch_all(&*pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_cash_movement(
    movement: CreateCashMovement,
    pool: State<'_, SqlitePool>,
) -> Result<CashMovement, String> {
    let id = sqlx::query(
        r#"
        INSERT INTO cash_register (transaction_type, amount, notes)
        VALUES (?, ?, ?)
        "#
    )
    .bind(&movement.transaction_type)
    .bind(movement.amount)
    .bind(&movement.notes)
    .execute(&*pool)
    .await
    .map_err(|e| e.to_string())?
    .last_insert_rowid();

    sqlx::query_as::<_, CashMovement>("SELECT * FROM cash_register WHERE id = ?")
        .bind(id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| e.to_string())
}
