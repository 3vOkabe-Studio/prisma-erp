use sqlx::SqlitePool;
use tauri::State;
use crate::models::product::{Product, CreateProduct};

#[tauri::command]
pub async fn get_products(pool: State<'_, SqlitePool>) -> Result<Vec<Product>, String> {
    sqlx::query_as::<_, Product>("SELECT * FROM products ORDER BY id DESC")
        .fetch_all(&*pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_product(
    new_product: CreateProduct,
    pool: State<'_, SqlitePool>,
) -> Result<Product, String> {
    let mut final_barcode = new_product.barcode.clone();
    let mut final_qr = new_product.qr_code.clone();

    if final_barcode.is_none() || final_barcode.as_ref().unwrap().is_empty() {
        let ts = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
        final_barcode = Some(format!("{}", ts));
    }
    if final_qr.is_none() || final_qr.as_ref().unwrap().is_empty() {
        let ts = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros();
        final_qr = Some(format!("QR-{}", ts));
    }

    let result = sqlx::query(
        r#"
        INSERT INTO products (name, sku, barcode, qr_code, category, current_stock, min_stock, cost, price)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#
    )
    .bind(&new_product.name)
    .bind(&new_product.sku)
    .bind(&final_barcode)
    .bind(&final_qr)
    .bind(&new_product.category)
    .bind(new_product.current_stock)
    .bind(new_product.min_stock)
    .bind(new_product.cost)
    .bind(new_product.price)
    .execute(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    sqlx::query_as::<_, Product>("SELECT * FROM products WHERE id = ?")
        .bind(result.last_insert_rowid())
        .fetch_one(&*pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_product(id: i64, pool: State<'_, SqlitePool>) -> Result<(), String> {
    sqlx::query("DELETE FROM products WHERE id = ?")
        .bind(id)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn update_product(
    product: crate::models::product::UpdateProduct,
    pool: tauri::State<'_, sqlx::SqlitePool>,
) -> Result<(), String> {
    sqlx::query(
        r#"
        UPDATE products 
        SET name = ?, sku = ?, barcode = ?, qr_code = ?, category = ?, current_stock = ?, min_stock = ?, cost = ?, price = ?
        WHERE id = ?
        "#
    )
    .bind(&product.name)
    .bind(&product.sku)
    .bind(&product.barcode)
    .bind(&product.qr_code)
    .bind(&product.category)
    .bind(product.current_stock)
    .bind(product.min_stock)
    .bind(product.cost)
    .bind(product.price)
    .bind(product.id)
    .execute(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn get_inventory_movements(
    pool: tauri::State<'_, sqlx::SqlitePool>,
) -> Result<Vec<crate::models::movement::InventoryMovement>, String> {
    sqlx::query_as::<_, crate::models::movement::InventoryMovement>(
        r#"
        SELECT m.id, m.product_id, p.name as product_name, m.movement_type, m.quantity, m.notes, m.created_at
        FROM inventory_movements m
        JOIN products p ON m.product_id = p.id
        ORDER BY m.id DESC
        LIMIT 100
        "#
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_low_stock_products(pool: State<'_, SqlitePool>) -> Result<Vec<Product>, String> {
    sqlx::query_as::<_, Product>("SELECT * FROM products WHERE current_stock <= min_stock ORDER BY current_stock ASC")
        .fetch_all(&*pool)
        .await
        .map_err(|e| e.to_string())
}
