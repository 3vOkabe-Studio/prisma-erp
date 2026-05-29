use sqlx::SqlitePool;
use tauri::State;
use crate::models::supplier::{Supplier, CreateSupplier};

#[tauri::command]
pub async fn get_suppliers(pool: State<'_, SqlitePool>) -> Result<Vec<Supplier>, String> {
    sqlx::query_as::<_, Supplier>("SELECT * FROM suppliers ORDER BY name ASC")
        .fetch_all(&*pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_supplier(
    new_supplier: CreateSupplier,
    pool: State<'_, SqlitePool>,
) -> Result<Supplier, String> {
    let id = sqlx::query(
        r#"
        INSERT INTO suppliers (name, phone, email, balance)
        VALUES (?, ?, ?, 0)
        "#
    )
    .bind(&new_supplier.name)
    .bind(&new_supplier.phone)
    .bind(&new_supplier.email)
    .execute(&*pool)
    .await
    .map_err(|e| e.to_string())?
    .last_insert_rowid();

    sqlx::query_as::<_, Supplier>("SELECT * FROM suppliers WHERE id = ?")
        .bind(id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| e.to_string())
}
