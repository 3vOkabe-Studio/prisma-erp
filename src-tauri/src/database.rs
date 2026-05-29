use sqlx::{sqlite::{SqliteConnectOptions, SqlitePoolOptions, SqliteJournalMode}, SqlitePool};
use std::str::FromStr;
use std::fs;
use anyhow::Result;
use tauri::{AppHandle, Manager};

pub async fn establish_connection(app_handle: &AppHandle) -> Result<SqlitePool> {
    // Determine the database path. In production, this should be in the AppData dir.
    // For development, we'll place it in the current directory or local app data.
    let app_dir = app_handle.path().app_data_dir().map_err(|e| anyhow::anyhow!("Failed to get app data dir: {}", e))?;
    if !app_dir.exists() {
        fs::create_dir_all(&app_dir)?;
    }
    
    let db_path = app_dir.join("prisma.db");
    let database_url = format!("sqlite://{}", db_path.to_string_lossy());

    if !db_path.exists() {
        // Create empty file if it doesn't exist
        fs::File::create(&db_path)?;
    }

    // Configure connection options with WAL mode and appropriate timeouts to handle concurrent access safely
    let connection_options = SqliteConnectOptions::from_str(&database_url)?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal) // WAL mode for better concurrency
        .pragma("synchronous", "NORMAL") // Optimized for WAL
        .busy_timeout(std::time::Duration::from_millis(5000)) // Avoid locks
        .foreign_keys(true); // Enforce foreign keys

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connection_options)
        .await?;

    // Optimización de la base de datos (VACUUM y PRAGMA optimize)
    // Esto garantiza que el archivo de base de datos se desfragmente y los índices se optimicen para producción.
    sqlx::query("PRAGMA optimize;").execute(&pool).await?;
    sqlx::query("VACUUM;").execute(&pool).await?;

    run_migrations(&pool).await?;

    Ok(pool)
}

async fn run_migrations(pool: &SqlitePool) -> Result<()> {
    // Initial schema migration
    let schema = r#"
        CREATE TABLE IF NOT EXISTS products (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            sku TEXT UNIQUE,
            barcode TEXT,
            category TEXT,
            current_stock REAL DEFAULT 0,
            min_stock REAL DEFAULT 0,
            cost REAL DEFAULT 0,
            price REAL DEFAULT 0,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS customers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            phone TEXT,
            email TEXT,
            address TEXT,
            credit_limit REAL DEFAULT 0,
            balance REAL DEFAULT 0,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS suppliers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            phone TEXT,
            email TEXT,
            balance REAL DEFAULT 0,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS inventory_movements (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            product_id INTEGER NOT NULL,
            movement_type TEXT NOT NULL, -- 'IN', 'OUT', 'ADJUST'
            quantity REAL NOT NULL,
            notes TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (product_id) REFERENCES products(id)
        );

        CREATE TABLE IF NOT EXISTS invoices (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            customer_id INTEGER,
            subtotal REAL DEFAULT 0,
            tax REAL DEFAULT 0,
            discount REAL DEFAULT 0,
            total REAL DEFAULT 0,
            status TEXT DEFAULT 'PENDING',
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (customer_id) REFERENCES customers(id)
        );

        CREATE TABLE IF NOT EXISTS invoice_items (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            invoice_id INTEGER NOT NULL,
            product_id INTEGER NOT NULL,
            quantity REAL NOT NULL,
            price REAL NOT NULL,
            subtotal REAL NOT NULL,
            FOREIGN KEY (invoice_id) REFERENCES invoices(id) ON DELETE CASCADE,
            FOREIGN KEY (product_id) REFERENCES products(id)
        );
        
        CREATE TABLE IF NOT EXISTS cash_register (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            transaction_type TEXT NOT NULL, -- 'OPEN', 'CLOSE', 'INCOME', 'EXPENSE'
            amount REAL NOT NULL,
            notes TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS expenses (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            category TEXT NOT NULL,
            amount REAL NOT NULL,
            notes TEXT,
            date DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        -- Índices de Rendimiento y Búsqueda
        CREATE INDEX IF NOT EXISTS idx_products_sku ON products(sku);
        CREATE INDEX IF NOT EXISTS idx_products_name ON products(name);
        CREATE INDEX IF NOT EXISTS idx_inventory_product_id ON inventory_movements(product_id);
        CREATE INDEX IF NOT EXISTS idx_invoices_customer_id ON invoices(customer_id);
        CREATE INDEX IF NOT EXISTS idx_invoice_items_invoice_id ON invoice_items(invoice_id);
        CREATE INDEX IF NOT EXISTS idx_invoice_items_product_id ON invoice_items(product_id);
    "#;

    sqlx::query(schema).execute(pool).await?;

    Ok(())
}
