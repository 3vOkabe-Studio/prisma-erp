# Changelog

Todos los cambios notables en Prisma ERP serán documentados en este archivo.

El formato está basado en [Keep a Changelog](https://keepachangelog.com/es-ES/1.0.0/),
y este proyecto se adhiere al [Versionado Semántico](https://semver.org/lang/es/).

## [0.1.0] - 2026-05-28

### Añadido
- **Módulo de Punto de Venta (POS):** Integrado con buscador global, atajos de teclado y cálculos automáticos de subtotal, impuestos y descuentos.
- **Inventario Offline-First:** Control de stock y catálogos usando SQLite WAL para rendimiento sin latencia.
- **Historial de Movimientos:** Trazabilidad absoluta de entradas y salidas de inventario, incluyendo ajustes manuales y salidas por ventas.
- **Buscador Global Inteligente (Command Palette):** Búsqueda en tiempo real desde el encabezado principal que vincula productos, SKUs y clientes.
- **Dashboard de Reportes:** Gráficos nativos semanales para visualizar ingresos, total de productos, total de clientes y estadísticas de cotizaciones pendientes.
- **Libro Mayor de Caja:** Unificación contable de ingresos por ventas y egresos misceláneos automáticos.
- **Sistema de Notificaciones:** Alertas dinámicas en el Header sobre productos que cruzaron el umbral de stock mínimo.
- **Estética Premium:** Tema oscuro/claro nativo, micro-animaciones en botones e interfaz pulida con Tailwind CSS y Lucide Icons.

### Seguridad
- La aplicación se ejecuta de forma completamente aislada (Offline-First), garantizando la privacidad absoluta de los datos transaccionales del cliente sin dependencias de nubes de terceros.
