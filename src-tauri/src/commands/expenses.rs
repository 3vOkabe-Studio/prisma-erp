use sqlx::SqlitePool;
use tauri::State;
use crate::models::expense::{Expense, CreateExpense};

#[tauri::command]
pub async fn get_expenses(pool: State<'_, SqlitePool>) -> Result<Vec<Expense>, String> {
    sqlx::query_as::<_, Expense>("SELECT * FROM expenses ORDER BY date DESC")
        .fetch_all(&*pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_expense(
    new_expense: CreateExpense,
    pool: State<'_, SqlitePool>,
) -> Result<Expense, String> {
    let id = sqlx::query(
        r#"
        INSERT INTO expenses (category, amount, notes)
        VALUES (?, ?, ?)
        "#
    )
    .bind(&new_expense.category)
    .bind(new_expense.amount)
    .bind(&new_expense.notes)
    .execute(&*pool)
    .await
    .map_err(|e| e.to_string())?
    .last_insert_rowid();

    // Automatically register as EXPENSE in cash_register
    let cash_notes = format!("Gasto Automático: {} - {}", new_expense.category, new_expense.notes.unwrap_or_default());
    sqlx::query(
        r#"
        INSERT INTO cash_register (transaction_type, amount, notes)
        VALUES ('EXPENSE', ?, ?)
        "#
    )
    .bind(new_expense.amount)
    .bind(cash_notes)
    .execute(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    sqlx::query_as::<_, Expense>("SELECT * FROM expenses WHERE id = ?")
        .bind(id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| e.to_string())
}
