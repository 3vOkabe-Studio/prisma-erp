use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardStats {
    pub total_sales: f64,
    pub today_sales: f64,
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

    // Today Sales
    let today_res = sqlx::query_as::<_, TotalRow>("SELECT SUM(total) as total FROM invoices WHERE status = 'PAID' AND date(created_at) = date('now')")
        .fetch_one(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    let today_sales = today_res.total.unwrap_or(0.0);

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
        today_sales,
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

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct RecentInvoice {
    pub id: String,
    pub customer: String,
    pub amount: String,
    pub status: String,
    pub time: String,
}

#[tauri::command]
pub async fn get_recent_invoices(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<RecentInvoice>, String> {
    // We use a simple query returning basic strings to match the dashboard's needs
    // time can be calculated roughly or just returning the date. We will return the created_at directly.
    let records = sqlx::query_as::<_, (i64, Option<String>, f64, Option<String>, Option<String>)>(
        r#"
        SELECT i.id, c.name, i.total, i.status, datetime(i.created_at, 'localtime')
        FROM invoices i
        LEFT JOIN customers c ON i.customer_id = c.id
        ORDER BY i.created_at DESC
        LIMIT 5
        "#
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for (id, cust, total, status, time) in records {
        result.push(RecentInvoice {
            id: format!("FAC-{:03}", id),
            customer: cust.unwrap_or_else(|| "Mostrador".to_string()),
            amount: format!("${:.2}", total),
            status: match status.as_deref() {
                Some("PAID") => "Pagada".to_string(),
                Some("PENDING") => "Pendiente".to_string(),
                Some("QUOTE") => "Cotización".to_string(),
                _ => "Desconocido".to_string(),
            },
            time: time.unwrap_or_default(),
        });
    }

    Ok(result)
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct StockAlert {
    pub name: String,
    pub current_stock: f64,
    pub min_stock: f64,
}

#[tauri::command]
pub async fn get_stock_alerts(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<StockAlert>, String> {
    let alerts = sqlx::query_as::<_, StockAlert>(
        "SELECT name, current_stock, min_stock FROM products WHERE current_stock <= min_stock LIMIT 5"
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(alerts)
}
