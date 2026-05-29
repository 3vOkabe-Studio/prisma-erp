use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardStats {
    pub total_sales: f64,
    pub total_products: i64,
    pub total_customers: i64,
    pub pending_invoices: i64,
}

#[derive(sqlx::FromRow)]
struct TotalRow { total: Option<f64> }

#[derive(sqlx::FromRow)]
struct CountRow { count: i32 }

#[tauri::command]
pub async fn get_dashboard_stats(
    pool: State<'_, SqlitePool>,
) -> Result<DashboardStats, String> {
    // Total Sales (only PAID)
    let sales_res = sqlx::query_as::<_, TotalRow>("SELECT SUM(total) as total FROM invoices WHERE status = 'PAID'")
        .fetch_one(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    
    let total_sales = sales_res.total.unwrap_or(0.0);

    // Total Products
    let prod_res = sqlx::query_as::<_, CountRow>("SELECT COUNT(*) as count FROM products")
        .fetch_one(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    
    let total_products = prod_res.count as i64;

    // Total Customers
    let cust_res = sqlx::query_as::<_, CountRow>("SELECT COUNT(*) as count FROM customers")
        .fetch_one(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    let total_customers = cust_res.count as i64;

    // Pending Invoices (Quotes)
    let pending_res = sqlx::query_as::<_, CountRow>("SELECT COUNT(*) as count FROM invoices WHERE status = 'QUOTE'")
        .fetch_one(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    let pending_invoices = pending_res.count as i64;

    Ok(DashboardStats {
        total_sales,
        total_products,
        total_customers,
        pending_invoices,
    })
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct DailySales {
    pub date: String,
    pub total: f64,
}

#[tauri::command]
pub async fn get_weekly_sales(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<DailySales>, String> {
    // In SQLite, we can group by date(created_at)
    let sales = sqlx::query_as::<_, DailySales>(
        r#"
        SELECT date(created_at) as date, SUM(total) as total
        FROM invoices
        WHERE status = 'PAID' AND created_at >= date('now', '-7 days')
        GROUP BY date(created_at)
        ORDER BY date(created_at) ASC
        "#
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(sales)
}
