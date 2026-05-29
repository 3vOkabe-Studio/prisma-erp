use sqlx::SqlitePool;
use tauri::State;
use crate::models::search::SearchResult;

#[derive(sqlx::FromRow)]
struct SearchProduct {
    id: i64,
    name: String,
    sku: Option<String>,
}

#[derive(sqlx::FromRow)]
struct SearchCustomer {
    id: i64,
    name: String,
    email: Option<String>,
}

#[tauri::command]
pub async fn global_search(
    query: String,
    pool: State<'_, SqlitePool>,
) -> Result<Vec<SearchResult>, String> {
    let mut results = Vec::new();
    let q = format!("%{}%", query);

    // Search Products
    let products = sqlx::query_as::<_, SearchProduct>(
        "SELECT id, name, sku FROM products WHERE name LIKE ? OR sku LIKE ? LIMIT 5"
    )
    .bind(&q)
    .bind(&q)
    .fetch_all(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    for p in products {
        results.push(SearchResult {
            id: p.id,
            title: p.name,
            subtitle: format!("SKU: {}", p.sku.unwrap_or_default()),
            result_type: "PRODUCT".to_string(),
            url: "/inventory".to_string(),
        });
    }

    // Search Customers
    let customers = sqlx::query_as::<_, SearchCustomer>(
        "SELECT id, name, email FROM customers WHERE name LIKE ? OR email LIKE ? LIMIT 5"
    )
    .bind(&q)
    .bind(&q)
    .fetch_all(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    for c in customers {
        results.push(SearchResult {
            id: c.id,
            title: c.name,
            subtitle: c.email.unwrap_or_default(),
            result_type: "CUSTOMER".to_string(),
            url: "/customers".to_string(),
        });
    }

    Ok(results)
}
