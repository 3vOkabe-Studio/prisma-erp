# Guía de Compilación y Despliegue

Prisma ERP, al utilizar Tauri, empaqueta su código web optimizado junto con un binario de Rust hiperrápido.

## 1. Requisitos para Compilar (Linux)
Si estás compilando desde o para Linux, asegúrate de tener las dependencias nativas instaladas (por ejemplo en Ubuntu):
```bash
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

## 2. Preparar el Frontend
Asegúrate de que Vite pueda empaquetar tus assets Svelte:
```bash
npm run build
```

## 3. Compilación de Producción (Release)
Para generar el ejecutable que entregarás al cliente final, usa:
```bash
npm run tauri:build
```
Este proceso puede tardar unos minutos ya que Cargo descargará, optimizará y compilará la base de datos SQLite embebida y los componentes GTK/Webkit.

## 4. Localización del Ejecutable
Una vez finalizado, encontrarás el ejecutable nativo en:
- `src-tauri/target/release/bundle/deb/` (Instaladores Debian/Ubuntu)
- `src-tauri/target/release/bundle/appimage/` (Ejecutable portable Linux)
- `src-tauri/target/release/bundle/msi/` (Instalador Windows)

## Actualizaciones Futuras
Prisma ERP almacena su base de datos localmente usando el directorio estándar de aplicación del Sistema Operativo (e.g. `~/.local/share/prisma-erp` en Linux o `%APPDATA%` en Windows). Gracias a esto, actualizar el archivo `.exe` o `.deb` no borrará los datos del usuario.
