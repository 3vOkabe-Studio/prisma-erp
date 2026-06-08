use sqlx::SqlitePool;
use tauri::State;
use crate::models::invoice::{Invoice, CreateInvoice};

#[tauri::command]
pub async fn create_invoice(
    new_invoice: CreateInvoice,
    pool: State<'_, SqlitePool>,
) -> Result<Invoice, String> {
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    let status = new_invoice.status.unwrap_or_else(|| "PAID".to_string());

    // 1. Insert Invoice
    let invoice_id = sqlx::query(
        r#"
        INSERT INTO invoices (customer_id, subtotal, tax, discount, total, status)
        VALUES (?, ?, ?, ?, ?, ?)
        "#
    )
    .bind(new_invoice.customer_id)
    .bind(new_invoice.subtotal)
    .bind(new_invoice.tax)
    .bind(new_invoice.discount)
    .bind(new_invoice.total)
    .bind(&status)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?
    .last_insert_rowid();

    // 2. Insert Items and Deduct Stock
    for item in new_invoice.items {
        sqlx::query(
            r#"
            INSERT INTO invoice_items (invoice_id, product_id, quantity, price, subtotal)
            VALUES (?, ?, ?, ?, ?)
            "#
        )
        .bind(invoice_id)
        .bind(item.product_id)
        .bind(item.quantity)
        .bind(item.price)
        .bind(item.subtotal)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        if status == "PAID" {
            // Deduct inventory
            sqlx::query(
                "UPDATE products SET current_stock = current_stock - ? WHERE id = ?"
            )
            .bind(item.quantity)
            .bind(item.product_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

            // Log movement
            sqlx::query(
                r#"
                INSERT INTO inventory_movements (product_id, movement_type, quantity, notes)
                VALUES (?, 'OUT', ?, ?)
                "#
            )
            .bind(item.product_id)
            .bind(item.quantity)
            .bind(format!("Sale - Invoice #{}", invoice_id))
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
        }
    }

    if status == "PAID" {
        // 3. Register cash movement
        sqlx::query(
            r#"
            INSERT INTO cash_register (transaction_type, amount, notes)
            VALUES ('INCOME', ?, ?)
            "#
        )
        .bind(new_invoice.total)
        .bind(format!("Payment for Invoice #{}", invoice_id))
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;
    }

    tx.commit().await.map_err(|e| e.to_string())?;

    sqlx::query_as::<_, Invoice>("SELECT * FROM invoices WHERE id = ?")
        .bind(invoice_id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_invoices(pool: State<'_, SqlitePool>) -> Result<Vec<Invoice>, String> {
    sqlx::query_as::<_, Invoice>("SELECT * FROM invoices ORDER BY id DESC")
        .fetch_all(&*pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_invoice_items(invoice_id: i64, pool: State<'_, SqlitePool>) -> Result<Vec<crate::models::invoice::InvoiceItemView>, String> {
    sqlx::query_as::<_, crate::models::invoice::InvoiceItemView>(
        r#"
        SELECT i.id, i.invoice_id, i.product_id, p.name as product_name, i.quantity, i.price, i.subtotal
        FROM invoice_items i
        LEFT JOIN products p ON i.product_id = p.id
        WHERE i.invoice_id = ?
        "#
    )
    .bind(invoice_id)
    .fetch_all(&*pool)
    .await
    .map_err(|e| e.to_string())
}
