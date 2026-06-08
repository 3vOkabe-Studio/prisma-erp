<script lang="ts">
  import { Plus, Search, Filter, MoreVertical, Edit, Trash2 } from '@lucide/svelte';
  import { invoke } from '@tauri-apps/api/core';
  import JsBarcode from 'jsbarcode';
  import QRCode from 'qrcode';

  // State using Runes
  // State using Runes
  let products = $state<any[]>([]);
  let movements = $state<any[]>([]);
  let isLoading = $state(true);
  let currentTab = $state('PRODUCTS'); // 'PRODUCTS' | 'MOVEMENTS'

  $effect(() => {
    async function loadProducts() {
      isLoading = true;
      try {
        products = await invoke('get_products');
        movements = await invoke('get_inventory_movements');
      } catch (e) {
        console.error("Error loading products:", e);
      } finally {
        isLoading = false;
      }
    }
    loadProducts();
  });

  // Create Product Modal State
  let showCreateModal = $state(false);
  let newProduct = $state({ name: '', sku: '', category: '', current_stock: 0, price: 0, min_stock: 0, cost: 0, barcode: '', qr_code: '' });

  async function handleCreateProduct() {
    try {
      if (editingProductId) {
        await invoke('update_product', { product: { ...newProduct, id: editingProductId } });
      } else {
        await invoke('create_product', { newProduct });
      }
      showCreateModal = false;
      editingProductId = null;
      newProduct = { name: '', sku: '', category: '', current_stock: 0, price: 0, min_stock: 0, cost: 0, barcode: '', qr_code: '' };
      // Reload products
      products = await invoke('get_products');
    } catch (e) {
      console.error("Error saving product:", e);
      alert("Error al guardar producto");
    }
  }

  let editingProductId = $state<number | null>(null);

  function openEditModal(product: any) {
    editingProductId = product.id;
    newProduct = { 
      name: product.name, 
      sku: product.sku || '', 
      category: product.category || '', 
      current_stock: product.current_stock || 0, 
      price: product.price || 0, 
      min_stock: product.min_stock || 0, 
      cost: product.cost || 0, 
      barcode: product.barcode || '',
      qr_code: product.qr_code || ''
    };
    showCreateModal = true;
  }

  async function handleDeleteProduct(id: number) {
    if (!confirm("¿Eliminar este producto?")) return;
    try {
      await invoke('delete_product', { id });
      products = await invoke('get_products');
    } catch (e) {
      console.error("Error deleting product:", e);
    }
  }

  let rawSearchQuery = $state('');
  let searchQuery = $state('');
  let searchTimeout: any;

  $effect(() => {
    // Debounce logic
    clearTimeout(searchTimeout);
    searchTimeout = setTimeout(() => {
      searchQuery = rawSearchQuery;
    }, 250);
  });

  $effect(() => {
    if (showCreateModal && editingProductId) {
      setTimeout(() => {
        try {
          if (newProduct.barcode) JsBarcode("#barcode-canvas", newProduct.barcode, { format: "CODE128", width: 1.5, height: 40, displayValue: true, background: "transparent", lineColor: "#000" });
          const qrCanvas = document.getElementById('qrcode-canvas');
          if (qrCanvas && newProduct.qr_code) QRCode.toCanvas(qrCanvas, newProduct.qr_code, { width: 80, margin: 1, color: { dark: "#000", light: "#0000" } });
        } catch(e) { console.log(e); }
      }, 100);
    }
  });

  let currentPage = $state(1);
  let itemsPerPage = 20;

  let filteredProducts = $derived(
    products.filter(p => 
      p.name.toLowerCase().includes(searchQuery.toLowerCase()) || 
      p.sku.toLowerCase().includes(searchQuery.toLowerCase())
    )
  );

  let paginatedProducts = $derived(filteredProducts.slice((currentPage - 1) * itemsPerPage, currentPage * itemsPerPage));
  let paginatedMovements = $derived(movements.slice((currentPage - 1) * itemsPerPage, currentPage * itemsPerPage));

  // Reset page when switching tabs or searching
  $effect(() => {
    let _ = currentTab;
    let __ = searchQuery;
    currentPage = 1;
  });
</script>

<div class="h-full flex flex-col space-y-6">
  <!-- Header & Actions -->
  <div class="flex items-center justify-between">
    <div>
      <h2 class="text-2xl font-bold tracking-tight">Inventario</h2>
      <p class="text-text-muted text-sm mt-1">Gestiona tus productos y existencias.</p>
    </div>
    <div class="flex items-center gap-3">
      {#if currentTab === 'PRODUCTS'}
        <button onclick={() => { editingProductId = null; newProduct = { name: '', sku: '', category: '', current_stock: 0, price: 0, min_stock: 0, cost: 0, barcode: '' }; showCreateModal = true; }} class="flex items-center gap-2 px-4 py-2 bg-accent text-primary-fg rounded-lg font-bold hover:bg-accent-hover transition-colors shadow-sm">
          <Plus size={18} /> Nuevo Producto
        </button>
      {/if}
    </div>
  </div>

  <div class="flex gap-4 border-b border-border">
    <button 
      onclick={() => currentTab = 'PRODUCTS'}
      class={`pb-2 font-medium ${currentTab === 'PRODUCTS' ? 'text-accent border-b-2 border-accent' : 'text-text-muted hover:text-text-main'}`}
    >
      Catálogo de Productos
    </button>
    <button 
      onclick={() => currentTab = 'MOVEMENTS'}
      class={`pb-2 font-medium ${currentTab === 'MOVEMENTS' ? 'text-accent border-b-2 border-accent' : 'text-text-muted hover:text-text-main'}`}
    >
      Historial de Movimientos
    </button>
  </div>

  <!-- Table Container -->
  <div class="flex-1 bg-surface border border-border rounded-xl shadow-sm flex flex-col overflow-hidden">
    <!-- Search Bar -->
    <div class="p-4 border-b border-border bg-base/50 flex gap-4">
      <div class="relative flex-1 max-w-md">
        <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
          <Search size={18} class="text-text-muted" />
        </div>
        <input
          type="text"
          bind:value={rawSearchQuery}
          placeholder="Buscar por nombre o SKU..."
          class="block w-full pl-10 pr-3 py-2.5 border border-border rounded-lg leading-5 bg-surface text-text-main placeholder-text-muted focus:outline-none focus:ring-1 focus:ring-accent focus:border-accent sm:text-sm transition-all"
        />
      </div>
    </div>

    {#if currentTab === 'PRODUCTS'}
      <!-- Data Table -->
      <div class="flex-1 overflow-auto">
        <table class="w-full text-sm text-left">
          <thead class="text-xs text-text-muted uppercase bg-base sticky top-0 z-10 shadow-sm">
            <tr>
              <th class="px-6 py-4 font-medium">SKU</th>
              <th class="px-6 py-4 font-medium">Producto</th>
              <th class="px-6 py-4 font-medium">Categoría</th>
              <th class="px-6 py-4 font-medium">Stock</th>
              <th class="px-6 py-4 font-medium">Costo</th>
              <th class="px-6 py-4 font-medium">Precio</th>
              <th class="px-6 py-4 font-medium text-right">Acciones</th>
            </tr>
          </thead>
          <tbody>
            {#if isLoading}
              <tr>
                <td colspan="6" class="px-6 py-12 text-center text-text-muted">Cargando inventario...</td>
              </tr>
            {:else if filteredProducts.length === 0}
              <tr>
                <td colspan="6" class="px-6 py-12 text-center text-text-muted">No se encontraron productos.</td>
              </tr>
            {:else}
              {#each paginatedProducts as product}
                <tr class="border-b border-border hover:bg-surface-hover/50 transition-colors">
                  <td class="px-6 py-4 font-mono text-xs text-text-muted">{product.sku}</td>
                  <td class="px-6 py-4 font-bold text-text-main">{product.name}</td>
                  <td class="px-6 py-4">
                    <span class="px-2.5 py-1 text-xs rounded-full font-medium bg-base border border-border text-text-muted">
                      {product.category}
                    </span>
                  </td>
                  <td class="px-6 py-4">
                    <div class="flex items-center gap-2">
                      {#if product.current_stock <= product.min_stock}
                        <span class="w-2 h-2 rounded-full bg-red-500"></span>
                      {:else}
                        <span class="w-2 h-2 rounded-full bg-green-500"></span>
                      {/if}
                      <span class={`font-bold ${product.current_stock <= product.min_stock ? 'text-red-500' : 'text-text-main'}`}>
                        {product.current_stock}
                      </span>
                    </div>
                  </td>
                  <td class="px-6 py-4 font-medium text-text-muted">${product.cost.toFixed(2)}</td>
                  <td class="px-6 py-4 font-medium">${product.price.toFixed(2)}</td>
                  <td class="px-6 py-4 text-right">
                    <div class="flex items-center justify-end gap-2">
                      <button onclick={() => openEditModal(product)} class="p-2 text-text-muted hover:text-accent rounded-lg transition-colors" title="Editar">
                        <Edit size={16} />
                      </button>
                      <button onclick={() => handleDeleteProduct(product.id)} class="p-2 text-text-muted hover:text-red-500 rounded-lg transition-colors" title="Eliminar">
                        <Trash2 size={16} />
                      </button>
                    </div>
                  </td>
                </tr>
              {/each}
            {/if}
          </tbody>
        </table>
      </div>
    {:else}
      <!-- Movements Table -->
      <div class="flex-1 overflow-auto">
        <table class="w-full text-sm text-left">
          <thead class="text-xs text-text-muted uppercase bg-base sticky top-0 z-10 shadow-sm">
            <tr>
              <th class="px-6 py-4 font-medium">Fecha</th>
              <th class="px-6 py-4 font-medium">Producto</th>
              <th class="px-6 py-4 font-medium">Tipo</th>
              <th class="px-6 py-4 font-medium">Cantidad</th>
              <th class="px-6 py-4 font-medium">Notas</th>
            </tr>
          </thead>
          <tbody>
            {#if isLoading}
              <tr>
                <td colspan="5" class="px-6 py-12 text-center text-text-muted">Cargando movimientos...</td>
              </tr>
            {:else if movements.length === 0}
              <tr>
                <td colspan="5" class="px-6 py-12 text-center text-text-muted">No hay movimientos registrados.</td>
              </tr>
            {:else}
              {#each paginatedMovements as movement}
                <tr class="border-b border-border hover:bg-surface-hover/50 transition-colors">
                  <td class="px-6 py-4 text-text-muted">{movement.created_at ? new Date(movement.created_at).toLocaleString() : 'N/A'}</td>
                  <td class="px-6 py-4 font-bold text-text-main">{movement.product_name}</td>
                  <td class="px-6 py-4">
                    <span class={`px-2.5 py-1 text-xs rounded-full font-medium border ${movement.movement_type === 'IN' ? 'bg-green-500/10 text-green-500 border-green-500/20' : movement.movement_type === 'OUT' ? 'bg-red-500/10 text-red-500 border-red-500/20' : 'bg-blue-500/10 text-blue-500 border-blue-500/20'}`}>
                      {movement.movement_type}
                    </span>
                  </td>
                  <td class="px-6 py-4 font-bold">{movement.movement_type === 'OUT' ? '-' : '+'}{movement.quantity}</td>
                  <td class="px-6 py-4 text-text-muted">{movement.notes || '-'}</td>
                </tr>
              {/each}
            {/if}
          </tbody>
        </table>
      </div>
    {/if}
    
    <!-- Pagination Footer -->
    <div class="p-4 border-t border-border bg-base/50 flex items-center justify-between text-sm text-text-muted">
      <span>Mostrando página {currentPage} de {Math.max(1, Math.ceil((currentTab === 'PRODUCTS' ? filteredProducts.length : movements.length) / itemsPerPage))}</span>
      <div class="flex gap-2">
        <button 
          onclick={() => currentPage = Math.max(1, currentPage - 1)} 
          disabled={currentPage === 1}
          class="px-3 py-1 border border-border rounded-md bg-surface hover:bg-surface-hover disabled:opacity-50"
        >Anterior</button>
        <button 
          onclick={() => currentPage = Math.min(Math.ceil((currentTab === 'PRODUCTS' ? filteredProducts.length : movements.length) / itemsPerPage), currentPage + 1)}
          disabled={currentPage >= Math.ceil((currentTab === 'PRODUCTS' ? filteredProducts.length : movements.length) / itemsPerPage)}
          class="px-3 py-1 border border-border rounded-md bg-surface hover:bg-surface-hover disabled:opacity-50"
        >Siguiente</button>
      </div>
    </div>
  </div>
</div>

{#if showCreateModal}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm" onclick={(e) => { if (e.target === e.currentTarget) showCreateModal = false }}>
    <div class="bg-surface border border-border rounded-xl shadow-2xl p-6 w-full max-w-md">
      <h3 class="text-xl font-bold mb-4">{editingProductId ? 'Editar Producto' : 'Crear Nuevo Producto'}</h3>
      <div class="space-y-4">
        <div>
          <label class="block text-sm font-medium text-text-muted mb-1" for="name">Nombre</label>
          <input id="name" type="text" bind:value={newProduct.name} class="w-full px-3 py-2 border border-border rounded-md bg-base text-text-main" />
        </div>
        <div>
          <label class="block text-sm font-medium text-text-muted mb-1" for="sku">SKU</label>
          <input id="sku" type="text" bind:value={newProduct.sku} class="w-full px-3 py-2 border border-border rounded-md bg-base text-text-main" />
        </div>
        <div>
          <label class="block text-sm font-medium text-text-muted mb-1" for="category">Categoría</label>
          <input id="category" type="text" bind:value={newProduct.category} class="w-full px-3 py-2 border border-border rounded-md bg-base text-text-main" />
        </div>
        <div class="grid grid-cols-3 gap-4">
          <div>
            <label class="block text-sm font-medium text-text-muted mb-1" for="stock">Stock</label>
            <input id="stock" type="number" bind:value={newProduct.current_stock} class="w-full px-3 py-2 border border-border rounded-md bg-base text-text-main" />
          </div>
          <div>
            <label class="block text-sm font-medium text-text-muted mb-1" for="cost">Costo</label>
            <input id="cost" type="number" step="0.01" bind:value={newProduct.cost} class="w-full px-3 py-2 border border-border rounded-md bg-base text-text-main" />
          </div>
          <div>
            <label class="block text-sm font-medium text-text-muted mb-1" for="price">Precio Venta</label>
            <input id="price" type="number" step="0.01" bind:value={newProduct.price} class="w-full px-3 py-2 border border-border rounded-md bg-base text-text-main" />
          </div>
        </div>
        {#if editingProductId}
          <div class="flex gap-4 items-center justify-center border border-border p-4 rounded-md bg-base">
            <div class="flex flex-col items-center">
              <span class="text-xs text-text-muted mb-2">Código de Barras</span>
              <svg id="barcode-canvas" class="bg-white rounded p-1"></svg>
            </div>
            <div class="flex flex-col items-center border-l border-border pl-4">
              <span class="text-xs text-text-muted mb-2">Código QR</span>
              <canvas id="qrcode-canvas" class="bg-white rounded p-1"></canvas>
            </div>
          </div>
        {/if}
      </div>
      <div class="mt-6 flex justify-end gap-3">
        <button onclick={() => showCreateModal = false} class="px-4 py-2 border border-border rounded-md font-medium hover:bg-surface-hover">Cancelar</button>
        <button onclick={handleCreateProduct} class="px-4 py-2 bg-accent text-primary-fg rounded-md font-bold hover:bg-accent-hover">Guardar</button>
      </div>
    </div>
  </div>
{/if}
