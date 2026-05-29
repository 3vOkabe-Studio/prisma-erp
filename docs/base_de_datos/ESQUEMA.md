# Esquema de Base de Datos y Configuración

Prisma ERP utiliza **SQLite** incrustado debido a su naturaleza de un solo archivo y despliegue simple.

## SQLite WAL (Write-Ahead Logging)

Una decisión arquitectónica clave fue habilitar el **PRAGMA journal_mode=WAL**.
En modo WAL, las lecturas y escrituras pueden ocurrir concurrentemente. Esto es crítico porque el sistema está construido sobre llamadas asíncronas de Tauri (`async/await` en Rust con `sqlx`). Si no usáramos WAL, el sistema podría bloquearse y lanzar errores "Database is locked" si un reporte se generara al mismo tiempo que se cobra una factura.

## Tablas Principales

- **`products`**: Almacena el catálogo de inventario (nombre, sku, stock actual, stock mínimo, precio, costo).
- **`inventory_movements`**: Tabla de trazabilidad inmutable. Cada vez que el stock de `products` cambia, se registra aquí (IN, OUT, ADJUSTMENT).
- **`customers`**: Datos de clientes (nombre, email, teléfono, balance deudor).
- **`invoices` & `invoice_items`**: Facturas emitidas o cotizaciones (status: PAID, PENDING, QUOTE). Cada venta genera un `invoice` y múltiples `invoice_items` atados por llave foránea.
- **`expenses`**: Egresos operativos.
- **`cash_register`**: Libro mayor. Registra `INCOME` (proveniente de `invoices` en estado PAID) y `EXPENSE` (provenientes de `expenses`).

## Migraciones

El esquema se inicializa automáticamente cuando la aplicación arranca por primera vez, leyendo el archivo de base de datos desde `app_data_dir()` proporcionado por Tauri para evitar sobreescribir la base de datos local en las actualizaciones de versión.
