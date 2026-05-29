<div align="center">
  <h1>💎 Prisma ERP</h1>
  <p><strong>El Sistema de Planificación de Recursos Empresariales Offline-First del Futuro</strong></p>

  <p>
    <a href="https://github.com/tauri-apps/tauri"><img src="https://img.shields.io/badge/Tauri-2.0-FFC131.svg?logo=tauri&logoColor=white" alt="Tauri" /></a>
    <a href="https://svelte.dev/"><img src="https://img.shields.io/badge/Svelte-5.0-FF3E00.svg?logo=svelte&logoColor=white" alt="Svelte" /></a>
    <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/Rust-1.77+-000000.svg?logo=rust&logoColor=white" alt="Rust" /></a>
    <a href="https://www.sqlite.org/index.html"><img src="https://img.shields.io/badge/SQLite-WAL-003B57.svg?logo=sqlite&logoColor=white" alt="SQLite" /></a>
    <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-green.svg" alt="License" /></a>
  </p>
</div>

---

## 📖 Descripción del Proyecto

**Prisma ERP** es una plataforma de gestión empresarial integral de grado comercial, diseñada bajo la arquitectura **Offline-First**. Construida utilizando las tecnologías más modernas y performantes del mercado, garantiza una experiencia de usuario fluida, diseño premium y cero latencia de red.

Este sistema permite gestionar ventas (Punto de Venta), inventario, clientes, y contabilidad desde una interfaz hermosa y moderna, ideal para pequeños y medianos negocios que requieren robustez sin depender de una conexión a internet activa.

## ✨ Características Principales

- ⚡ **Offline-First Absoluto**: Base de datos SQLite integrada. Nada se envía a servidores externos. Cero latencia.
- 🎨 **Diseño Premium**: Interfaz de usuario construida con Tailwind CSS, temática dinámica (Claro/Oscuro) y micro-animaciones fluidas.
- 📦 **Gestión de Inventario**: Control de stock en tiempo real, alertas de inventario bajo, y registro histórico de movimientos.
- 🛒 **Punto de Venta (POS) Inteligente**: Búsqueda global, autocompletado en milisegundos y cobro rápido.
- 📊 **Dashboard y Reportes**: Gráficos nativos para analizar las ventas, movimientos y crecimiento del negocio.
- 💼 **Gestión de Egresos y Caja**: Libro mayor de caja unificado y automático.

## 🛠️ Stack Tecnológico

Prisma ERP aprovecha lo mejor de dos mundos: el ecosistema nativo ultrarrápido y el desarrollo web moderno.

* **Frontend:** [Svelte 5](https://svelte.dev/) (Runes), TypeScript, Tailwind CSS, Lucide Icons.
* **Backend:** [Rust](https://www.rust-lang.org/) (Rendimiento crudo y seguridad de memoria).
* **Base de Datos:** [SQLite](https://www.sqlite.org/) con modo WAL (Write-Ahead Logging) usando `sqlx` para accesos concurrentes de alta velocidad.
* **Framework Nativo:** [Tauri v2](https://v2.tauri.app/) (Binarios livianos, menor consumo de RAM que Electron).

## 🚀 Instalación y Desarrollo

### Requisitos Previos

Asegúrate de tener instalados los siguientes componentes:
- [Node.js](https://nodejs.org/) (v18 o superior)
- [Rust](https://www.rust-lang.org/tools/install) (cargo, rustc)
- Dependencias de sistema operativo para Tauri (ej. `libwebkit2gtk-4.1-dev` en Linux).

### Configuración del Entorno

1. **Clonar el repositorio:**
```bash
git clone https://github.com/tu-usuario/prisma-erp.git
cd prisma-erp
```

2. **Instalar dependencias del frontend:**
```bash
npm install
```

3. **Iniciar el servidor de desarrollo:**
```bash
npm run dev
```
Esto abrirá la aplicación en modo desarrollo, con recarga en caliente (HMR) tanto para Svelte como para Rust.

## 📦 Compilación (Producción)

Para generar el ejecutable nativo para tu sistema operativo actual:

```bash
npm run tauri:build
```
El binario final (app, exe o deb) se ubicará en `src-tauri/target/release/bundle/`.

## 📂 Estructura del Proyecto

El repositorio sigue una arquitectura estricta de separación de responsabilidades:

```
prisma-erp/
├── docs/                   # Documentación técnica detallada
├── src/                    # Frontend (SvelteKit + Tailwind)
│   ├── components/         # Componentes reutilizables de UI
│   ├── routes/             # Páginas y módulos del ERP
│   └── lib/                # Utilidades JS/TS
├── src-tauri/              # Backend (Rust + Tauri)
│   ├── src/
│   │   ├── commands/       # Lógica de negocio (Controladores)
│   │   ├── models/         # Estructuras de datos (Structs)
│   │   └── database.rs     # Configuración de SQLite/WAL
│   ├── Cargo.toml          # Dependencias de Rust
│   └── tauri.conf.json     # Configuración de Tauri
└── package.json            # Scripts y dependencias NPM
```

## 📚 Documentación Adicional

Para entender a fondo la ingeniería detrás de Prisma ERP, por favor consulta nuestra carpeta `/docs/`:

* [Arquitectura General y Flujo de Datos](docs/arquitectura/GENERAL.md)
* [Esquema de Base de Datos y SQLite WAL](docs/base_de_datos/ESQUEMA.md)
* [Uso de Svelte 5 (Runes) en el Frontend](docs/frontend/SVELTE5.md)
* [API de Comandos de Tauri (Backend)](docs/backend/TAURI_COMMANDS.md)
* [Guía de Compilación y Despliegue](docs/deployment/BUILD.md)

## 🤝 Contribuir

¡Aceptamos contribuciones! Prisma ERP es construido por y para la comunidad.
Por favor, lee nuestro [CONTRIBUTING.md](CONTRIBUTING.md) para detalles sobre nuestro código de conducta, estándares de código y el proceso para enviarnos *Pull Requests*.

## 📜 Filosofía del Software

Prisma ERP nace de la creencia de que el software de gestión empresarial para PYMES no tiene por qué ser feo, lento o estar atado a suscripciones en la nube obligatorias. Creemos en la propiedad de los datos, la velocidad instantánea y el diseño excepcional.

## 📄 Licencia

Este proyecto está bajo la Licencia MIT - mira el archivo [LICENSE](LICENSE) para más detalles.
