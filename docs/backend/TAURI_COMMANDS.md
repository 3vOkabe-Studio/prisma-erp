# Comandos de Tauri (Backend Rust)

El backend expone comandos a los que se puede llamar desde JavaScript/Svelte.

## Lista de Comandos Implementados

### Búsqueda Global
- `global_search(query: String)`: Busca a lo largo de las tablas de productos y clientes y unifica los resultados para el autocompletado en el encabezado principal.

### Inventario (Products)
- `get_products()`: Retorna todo el catálogo.
- `create_product(newProduct: CreateProduct)`
- `update_product(product: Product)`
- `delete_product(id: i32)`
- `get_low_stock_products()`: Retorna productos donde `current_stock <= min_stock`.
- `get_inventory_movements()`: Trae la bitácora de movimientos (IN, OUT, ADJUSTMENT).

### Ventas (Invoices)
- `get_invoices()`: Retorna las facturas procesadas y pendientes (cotizaciones).
- `create_invoice(newInvoice: CreateInvoice)`: Lógica transaccional compleja. Inserta la factura, los items asociados, disminuye el inventario y envía una señal de ingreso (`INCOME`) a la caja registradora.

### Clientes (Customers)
- `get_customers()`
- `create_customer(newCustomer: CreateCustomer)`
- `update_customer(customer: Customer)`
- `delete_customer(id: i32)`

### Egresos (Expenses)
- `get_expenses()`
- `create_expense(newExpense: CreateExpense)`: Registra el gasto e inyecta un egreso (`EXPENSE`) en la caja registradora.

### Caja Registradora (Cash Register)
- `get_cash_movements()`: Historial del libro mayor unificado.
- `create_cash_movement(...)`

### Reportes (Dashboard)
- `get_dashboard_stats()`: Retorna un objeto estructurado con ventas totales, productos, clientes y cotizaciones pendientes.
- `get_weekly_sales()`: Extrae las ventas (`PAID`) de los últimos 7 días agrupadas por día para la renderización de gráficos.
