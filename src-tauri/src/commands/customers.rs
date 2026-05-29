use sqlx::SqlitePool;
use tauri::State;
use crate::models::customer::{Customer, CreateCustomer};

#[tauri::command]
pub async fn get_customers(pool: State<'_, SqlitePool>) -> Result<Vec<Customer>, String> {
    sqlx::query_as::<_, Customer>("SELECT * FROM customers ORDER BY name ASC")
        .fetch_all(&*pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_customer(
    new_customer: CreateCustomer,
    pool: State<'_, SqlitePool>,
) -> Result<Customer, String> {
    let id = sqlx::query(
        r#"
        INSERT INTO customers (name, phone, email, address, credit_limit, balance)
        VALUES (?, ?, ?, ?, ?, 0)
        "#
    )
    .bind(&new_customer.name)
    .bind(&new_customer.phone)
    .bind(&new_customer.email)
    .bind(&new_customer.address)
    .bind(new_customer.credit_limit)
    .execute(&*pool)
    .await
    .map_err(|e| e.to_string())?
    .last_insert_rowid();

    sqlx::query_as::<_, Customer>("SELECT * FROM customers WHERE id = ?")
        .bind(id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_customer(
    customer: crate::models::customer::UpdateCustomer,
    pool: State<'_, SqlitePool>,
) -> Result<(), String> {
    sqlx::query(
        r#"
        UPDATE customers 
        SET name = ?, phone = ?, email = ?, address = ?, credit_limit = ?
        WHERE id = ?
        "#
    )
    .bind(&customer.name)
    .bind(&customer.phone)
    .bind(&customer.email)
    .bind(&customer.address)
    .bind(customer.credit_limit)
    .bind(customer.id)
    .execute(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}
