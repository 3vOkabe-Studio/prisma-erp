mod database;
mod models;
mod commands;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::block_on(async move {
                let pool = database::establish_connection(&handle).await.expect("Failed to initialize database");
                handle.manage(pool);
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::products::get_products,
            commands::products::create_product,
            commands::products::delete_product,
            commands::products::update_product,
            commands::products::get_inventory_movements,
            commands::products::get_low_stock_products,
            commands::invoices::get_invoices,
            commands::invoices::get_invoice_items,
            commands::invoices::create_invoice,
            commands::customers::get_customers,
            commands::customers::create_customer,
            commands::customers::update_customer,
            commands::cash::get_cash_movements,
            commands::cash::create_cash_movement,
            commands::suppliers::get_suppliers,
            commands::suppliers::create_supplier,
            commands::expenses::get_expenses,
            commands::expenses::create_expense,
            commands::search::global_search,
            commands::reports::get_dashboard_stats,
            commands::reports::get_weekly_sales,
            commands::reports::get_recent_invoices,
            commands::reports::get_stock_alerts,
            commands::settings::get_settings,
            commands::settings::update_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
