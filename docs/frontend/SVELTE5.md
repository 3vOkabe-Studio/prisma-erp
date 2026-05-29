# Uso de Svelte 5 (Runes) en Prisma ERP

El frontend está escrito puramente en **Svelte 5**, aprovechando su nuevo motor de reactividad basado en Runes. Esto permite un código mucho más limpio y performante.

## Runes Principales Utilizados

### `$state`
Se utiliza para declarar variables reactivas. Altera el DOM instantáneamente cuando su valor cambia.
```svelte
let cart = $state([]);
let barcodeInput = $state('');
```

### `$derived`
Reemplaza las antiguas declaraciones `$:`. Se utiliza para valores computados que dependen del `$state`.
```svelte
// Calcula el subtotal en tiempo real basándose en el contenido de 'cart'
let subtotal = $derived(cart.reduce((acc, item) => acc + (item.price * item.qty), 0));
```

### `$effect`
Reemplaza a `onMount` en muchos casos y ejecuta efectos secundarios cuando sus dependencias cambian. Se utiliza ampliamente para hacer *fetching* (llamadas a Tauri).
```svelte
$effect(() => {
    async function loadProducts() {
        products = await invoke('get_products');
    }
    loadProducts();
});
```

## Estructura de Rutas y Navegación
Usamos el enrutador de SvelteKit. Cada módulo principal (Ventas, Inventario, Clientes, Reportes) tiene su propia carpeta en `src/routes/` y un archivo `+page.svelte` que sirve como punto de entrada de la vista.
