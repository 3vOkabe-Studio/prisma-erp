# Guía para Contribuir a Prisma ERP

¡Gracias por tu interés en contribuir a Prisma ERP! Somos un proyecto abierto a la comunidad y valoramos tu ayuda para hacer que esta herramienta crezca y sea mejor para todos.

A continuación, encontrarás pautas para contribuir al proyecto de forma fluida.

## 1. Flujo de Trabajo (Workflow)

El modelo que utilizamos es el de **Fork & Pull Request**:
1. Haz un **Fork** de este repositorio.
2. Clona el fork en tu entorno local.
3. Crea una rama (`branch`) descriptiva para tu función o corrección:
   `git checkout -b feature/nueva-funcion` o `git checkout -b fix/error-inventario`
4. Realiza tus cambios. ¡Asegúrate de probarlos!
5. Haz *commit* de tus cambios usando mensajes descriptivos convencionales.
6. Haz *push* a tu repositorio (fork).
7. Abre un **Pull Request (PR)** hacia la rama `main` del repositorio original.

## 2. Estándares de Código

Para mantener la base de código limpia y unificada, te pedimos que sigas estas reglas:
- **Idioma del Código:** Las variables, funciones, nombres de archivos y estructura de base de datos **DEBEN estar en Inglés Técnico** (`get_products`, `ShoppingCart`, etc.).
- **Comentarios:** Los comentarios que expliquen la lógica de negocio deben estar **en Español** para facilitar la lectura a desarrolladores hispanohablantes.
- **Formateo:**
  - En Svelte/JS: Corre `npm run format` antes de enviar tu PR.
  - En Rust: Corre `cargo fmt` en el directorio `src-tauri` antes de enviar tu PR.

## 3. Arquitectura y Tecnologías

Por favor, asegúrate de leer la [Documentación de Arquitectura](docs/arquitectura/GENERAL.md) antes de proponer cambios estructurales grandes. 
El proyecto usa:
- **Tauri** + **Rust** para el backend (control de la base de datos).
- **SQLite (WAL)** para persistencia de datos (Offline-First).
- **Svelte 5 (Runes)** para reactividad en el frontend.
- **Tailwind CSS** para estilado (evita añadir CSS tradicional a menos que sea estrictamente necesario).

## 4. Reporte de Errores (Bugs)

Si encuentras un error, por favor abre un *Issue* en GitHub usando la plantilla proporcionada. Asegúrate de incluir:
- Tu sistema operativo y versión.
- Versión de Prisma ERP.
- Pasos detallados para reproducir el problema.
- Capturas de pantalla si es aplicable.

¡Gracias por construir el futuro del software Offline-First con nosotros!
