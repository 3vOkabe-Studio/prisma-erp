# Arquitectura General de Prisma ERP

## Filosofía: Offline-First Absoluto

Prisma ERP está diseñado con un objetivo claro: **cero dependencia de la nube**. Las PYMES a menudo sufren caídas de internet que paralizan su facturación. Prisma soluciona esto ejecutándose nativamente en la máquina del usuario.

## Componentes del Sistema

1. **Frontend (Capa de Presentación):**
   - **Tecnología:** Svelte 5 con SvelteKit (SSG/SPA).
   - **Responsabilidad:** Renderizar la interfaz de usuario, manejar el estado reactivo local (Runes) y escuchar los eventos del usuario.
   - **Estilos:** Tailwind CSS proporciona utilidad y rapidez sin necesidad de archivos `.css` masivos.

2. **Backend (Capa de Negocio y Persistencia):**
   - **Tecnología:** Rust (dentro de Tauri).
   - **Responsabilidad:** Conexión directa con el sistema de archivos del SO, manejo seguro de la base de datos SQLite y ejecución de lógicas pesadas (búsqueda global rápida).

3. **IPC (Inter-Process Communication):**
   - Tauri actúa como el puente entre el Frontend en JavaScript/TypeScript y el Backend en Rust.
   - Usamos `@tauri-apps/api/core` para invocar comandos de Rust (ej. `invoke('get_products')`).

## Flujo de Datos Típico (Venta en POS)

1. **Usuario** escanea un código de barras en el frontend.
2. **Frontend** evalúa el código (memoria local o llama a `invoke('global_search')`).
3. **Backend (Rust)** ejecuta la query optimizada en SQLite y devuelve la estructura de datos segura en JSON.
4. **Frontend** actualiza el carrito (`$state` reactivo).
5. **Usuario** presiona "Cobrar".
6. **Frontend** empaqueta los datos de la venta y llama a `invoke('create_invoice')`.
7. **Backend** abre una transacción SQL, inserta la factura, deduce el stock, registra la entrada en `cash_register` y hace commit de la transacción (Mode WAL).

Esta separación estricta garantiza que el frontend sea extremadamente rápido y que la manipulación de datos sea 100% segura y concurrente.
