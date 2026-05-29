<script lang="ts">
  import { Search, Plus, Minus, Trash2, CreditCard, User, Tag, ShoppingCart } from '@lucide/svelte';
  import { invoke } from '@tauri-apps/api/core';

  // ==========================================
  // ESTADO REACTIVO (Svelte 5 Runes)
  // ==========================================
  let barcodeInput = $state(''); // Entrada de texto del buscador
  let cart = $state<Array<{id: number, name: string, price: number, qty: number}>>([]);
  let discount = $state(0);
  let customerName = $state('Consumidor Final');
  let customerId = $state<number | null>(null);
  let showCustomerModal = $state(false);
  let customers = $state<any[]>([]);
  let customerSearch = $state(''); // Filtro de búsqueda de clientes

  // ==========================================
  // VALORES DERIVADOS (Calculados automáticamente)
  // ==========================================
  let filteredCustomers = $derived(
    customerSearch.trim() === '' 
      ? customers 
      : customers.filter(c => 
          c.name.toLowerCase().includes(customerSearch.toLowerCase()) || 
          (c.email && c.email.toLowerCase().includes(customerSearch.toLowerCase()))
        )
  );

  let subtotal = $derived(cart.reduce((acc, item) => acc + (item.price * item.qty), 0));
  let tax = $derived(subtotal * 0.18); // Ejemplo: 18% de impuestos
  let total = $derived(subtotal + tax - discount);

  let products = $state<any[]>([]);

  // ==========================================
  // EFECTOS DE CICLO DE VIDA
  // ==========================================
  // Se ejecuta al montar el componente para cargar el catálogo inicial
  $effect(() => {
    async function loadProducts() {
      try {
        products = await invoke('get_products');
      } catch (e) {
        console.error(e);
      }
    }
    loadProducts();
  });

  let showSuggestions = $state(false);
  
  let searchSuggestions = $derived(
    barcodeInput.trim() === '' ? [] : products.filter(p => 
      p.barcode === barcodeInput || 
      (p.sku && p.sku.toLowerCase().includes(barcodeInput.toLowerCase())) || 
      p.name.toLowerCase().includes(barcodeInput.toLowerCase())
    ).slice(0, 5) // Limit to 5 suggestions
  );

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter' && barcodeInput.trim() !== '') {
      // If exact match by barcode, add directly
      const exactMatch = products.find(p => p.barcode === barcodeInput || p.sku === barcodeInput);
      if (exactMatch) {
        addToCart({ id: exactMatch.id, name: exactMatch.name, price: exactMatch.price, qty: 1 });
        barcodeInput = '';
        showSuggestions = false;
      } else if (searchSuggestions.length > 0) {
        // Add the first suggestion on enter if no exact barcode match
        const first = searchSuggestions[0];
        addToCart({ id: first.id, name: first.name, price: first.price, qty: 1 });
        barcodeInput = '';
        showSuggestions = false;
      } else {
        alert("Producto no encontrado");
      }
    }
  }

  function handleInputFocus() {
    if (barcodeInput.trim() !== '') {
      showSuggestions = true;
    }
  }

  $effect(() => {
    if (barcodeInput.trim() !== '') {
      showSuggestions = true;
    } else {
      showSuggestions = false;
    }
  });

  function selectSuggestion(product: any) {
    addToCart({ id: product.id, name: product.name, price: product.price, qty: 1 });
    barcodeInput = '';
    showSuggestions = false;
  }

  function addToCart(product: any) {
    const existing = cart.find(i => i.id === product.id);
    if (existing) {
      existing.qty += 1;
    } else {
      cart.push(product);
    }
  }

  function removeFromCart(id: number) {
    cart = cart.filter(i => i.id !== id);
  }

  async function processSale(status: string = 'PAID') {
    if (cart.length === 0) return;
    
    try {
      const items = cart.map(i => ({
        product_id: i.id,
        quantity: i.qty,
        price: i.price,
        subtotal: i.price * i.qty
      }));

      await invoke('create_invoice', {
        newInvoice: {
          customer_id: customerId,
          subtotal,
          tax,
          discount,
          total,
          status,
          items
        }
      });

      if (status === 'QUOTE') {
        alert(`Presupuesto guardado con éxito. Total: $${total.toFixed(2)}`);
      } else {
        alert(`Venta procesada con éxito. Total: $${total.toFixed(2)}`);
      }
      
      cart = [];
      barcodeInput = '';
      discount = 0;
      customerId = null;
      customerName = 'Consumidor Final';
    } catch (e) {
      console.error(e);
      alert("Error al procesar");
    }
  }

  async function loadCustomers() {
    try {
      customers = await invoke('get_customers');
      showCustomerModal = true;
    } catch (e) {
      console.error(e);
    }
  }

  function selectCustomer(customer: any) {
    customerId = customer.id;
    customerName = customer.name;
    showCustomerModal = false;
  }
</script>

<div class="flex h-full gap-6">
  <!-- Left Side: Product Search and List -->
  <div class="flex-1 flex flex-col bg-surface border border-border rounded-xl shadow-sm overflow-hidden">
    <!-- Header/Search -->
    <div class="p-4 border-b border-border bg-surface-hover/50 flex gap-4">
      <div class="relative flex-1">
        <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
          <Search size={18} class="text-text-muted" />
        </div>
        <!-- svelte-ignore a11y_autofocus -->
        <input
          type="text"
          bind:value={barcodeInput}
          onkeydown={handleKeydown}
          onfocus={handleInputFocus}
          onblur={() => setTimeout(() => showSuggestions = false, 200)}
          autofocus
          placeholder="Escanear código de barras o buscar producto... (Presiona Enter)"
          class="block w-full pl-10 pr-3 py-3 border border-border rounded-lg leading-5 bg-base text-text-main placeholder-text-muted focus:outline-none focus:ring-2 focus:ring-accent focus:border-accent font-medium text-lg"
        />
        {#if showSuggestions && searchSuggestions.length > 0}
          <div class="absolute z-20 w-full mt-1 bg-surface border border-border rounded-lg shadow-2xl overflow-hidden">
            {#each searchSuggestions as suggestion}
              <!-- svelte-ignore a11y_click_events_have_key_events -->
              <!-- svelte-ignore a11y_no_static_element_interactions -->
              <div 
                onclick={() => selectSuggestion(suggestion)}
                class="px-4 py-3 hover:bg-surface-hover cursor-pointer border-b border-border last:border-0 flex justify-between items-center transition-colors"
              >
                <div>
                  <p class="font-bold text-text-main">{suggestion.name}</p>
                  <p class="text-xs text-text-muted">SKU: {suggestion.sku || 'N/A'}</p>
                </div>
                <div class="text-right">
                  <p class="font-bold text-accent">${suggestion.price.toFixed(2)}</p>
                  <p class="text-xs text-text-muted">Stock: {suggestion.current_stock}</p>
                </div>
              </div>
            {/each}
          </div>
        {/if}
      </div>
    </div>

    <!-- Cart Items -->
    <div class="flex-1 overflow-y-auto p-4 space-y-3">
      {#if cart.length === 0}
        <div class="h-full flex flex-col items-center justify-center text-text-muted space-y-4">
          <ShoppingCart size={48} class="opacity-20" />
          <p class="text-lg">El carrito está vacío</p>
          <p class="text-sm">Escanea un producto para comenzar</p>
        </div>
      {:else}
        {#each cart as item (item.id)}
          <div class="flex items-center justify-between p-3 border border-border rounded-lg bg-base">
            <div class="flex-1">
              <h4 class="font-bold text-text-main">{item.name}</h4>
              <p class="text-sm text-text-muted">${item.price.toFixed(2)}</p>
            </div>
            <div class="flex items-center gap-4">
              <div class="flex items-center bg-surface border border-border rounded-lg">
                <button 
                  onclick={() => { if(item.qty > 1) { item.qty--; } else removeFromCart(item.id) }} 
                  class="p-2 text-text-muted hover:text-text-main hover:bg-surface-hover rounded-l-lg transition-colors"
                >
                  <Minus size={16} />
                </button>
                <span class="w-10 text-center font-bold">{item.qty}</span>
                <button 
                  onclick={() => { item.qty++; }} 
                  class="p-2 text-text-muted hover:text-text-main hover:bg-surface-hover rounded-r-lg transition-colors"
                >
                  <Plus size={16} />
                </button>
              </div>
              <div class="w-24 text-right font-bold text-lg">
                ${(item.price * item.qty).toFixed(2)}
              </div>
              <button onclick={() => removeFromCart(item.id)} class="p-2 text-red-500 hover:bg-red-500/10 rounded-lg transition-colors">
                <Trash2 size={18} />
              </button>
            </div>
          </div>
        {/each}
      {/if}
    </div>
  </div>

  <!-- Right Side: Checkout Panel -->
  <div class="w-96 flex flex-col bg-surface border border-border rounded-xl shadow-sm overflow-hidden">
    <!-- Customer Info -->
    <div class="p-5 border-b border-border">
      <div class="flex items-center gap-2 text-text-muted mb-2">
        <User size={16} />
        <span class="text-sm font-medium">Cliente</span>
      </div>
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div onclick={loadCustomers} class="flex items-center justify-between bg-base p-3 rounded-lg border border-border cursor-pointer hover:border-accent transition-colors">
        <span class="font-bold text-text-main">{customerName}</span>
        <span class="text-xs text-accent font-medium">Cambiar</span>
      </div>
    </div>

    <!-- Payment Summary -->
    <div class="flex-1 p-5 bg-surface-hover/30">
      <div class="space-y-4">
        <div class="flex justify-between text-text-muted font-medium">
          <span>Subtotal</span>
          <span>${subtotal.toFixed(2)}</span>
        </div>
        <div class="flex justify-between text-text-muted font-medium">
          <span>Impuestos (18%)</span>
          <span>${tax.toFixed(2)}</span>
        </div>
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div onclick={() => { let d = prompt('Ingrese descuento ($):'); if(d) discount = parseFloat(d) || 0; }} class="flex justify-between text-accent font-medium cursor-pointer hover:underline">
          <div class="flex items-center gap-1"><Tag size={14} /> Agregar Descuento</div>
          <span>-${discount.toFixed(2)}</span>
        </div>
        <div class="pt-4 border-t border-border mt-4">
          <div class="flex justify-between items-end">
            <span class="text-lg font-bold">Total</span>
            <span class="text-4xl font-black text-text-main tracking-tight">${total.toFixed(2)}</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Actions -->
    <div class="p-5 border-t border-border bg-base space-y-3">
      <button 
        onclick={() => processSale('PAID')}
        disabled={cart.length === 0}
        class="w-full flex items-center justify-center gap-2 py-4 bg-accent text-primary-fg font-bold rounded-xl text-lg hover:bg-accent-hover transition-colors disabled:opacity-50 disabled:cursor-not-allowed shadow-md shadow-accent/20"
      >
        <CreditCard size={20} />
        Cobrar (F12)
      </button>
      <div class="flex gap-3">
        <button onclick={() => processSale('QUOTE')} disabled={cart.length === 0} class="flex-1 py-3 bg-surface border border-border rounded-lg font-bold text-text-muted hover:text-text-main hover:bg-surface-hover transition-colors disabled:opacity-50">
          Guardar (F9)
        </button>
        <button onclick={() => { cart = []; discount = 0; }} class="flex-1 py-3 bg-surface border border-border rounded-lg font-bold text-red-500 hover:bg-red-500/10 transition-colors">
          Cancelar (Esc)
        </button>
      </div>
    </div>
  </div>
</div>

{#if showCustomerModal}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm" onclick={(e) => { if (e.target === e.currentTarget) showCustomerModal = false }}>
    <div class="bg-surface border border-border rounded-xl shadow-2xl p-6 w-full max-w-md flex flex-col max-h-[80vh]">
      <h3 class="text-xl font-bold mb-4">Seleccionar Cliente</h3>
      
      <div class="mb-4 relative">
        <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
          <Search size={16} class="text-text-muted" />
        </div>
        <!-- svelte-ignore a11y_autofocus -->
        <input
          type="text"
          bind:value={customerSearch}
          autofocus
          placeholder="Buscar por nombre o email..."
          class="block w-full pl-10 pr-3 py-2 border border-border rounded-lg leading-5 bg-base text-text-main placeholder-text-muted focus:outline-none focus:ring-1 focus:ring-accent focus:border-accent sm:text-sm transition-all"
        />
      </div>

      <div class="flex-1 overflow-y-auto space-y-2">
        {#if customerSearch.trim() === '' || 'consumidor final'.includes(customerSearch.toLowerCase())}
          <div 
            onclick={() => selectCustomer({ id: null, name: 'Consumidor Final' })}
            class="p-3 border border-border rounded-lg hover:border-accent cursor-pointer bg-base flex justify-between items-center"
          >
            <span class="font-bold">Consumidor Final</span>
          </div>
        {/if}
        {#each filteredCustomers as customer}
          <div 
            onclick={() => selectCustomer(customer)}
            class="p-3 border border-border rounded-lg hover:border-accent cursor-pointer bg-base flex justify-between items-center"
          >
            <div>
              <p class="font-bold">{customer.name}</p>
              <p class="text-xs text-text-muted">{customer.email || 'Sin email'}</p>
            </div>
            {#if customer.balance > 0}
              <span class="text-xs font-bold text-red-500">Debe: ${customer.balance.toFixed(2)}</span>
            {/if}
          </div>
        {/each}
        {#if filteredCustomers.length === 0 && !('consumidor final'.includes(customerSearch.toLowerCase()))}
          <div class="p-4 text-center text-text-muted text-sm">
            No se encontraron clientes.
          </div>
        {/if}
      </div>
      
      <div class="mt-4 pt-4 border-t border-border flex justify-end">
        <button onclick={() => showCustomerModal = false} class="px-4 py-2 border border-border rounded-md font-medium hover:bg-surface-hover">Cerrar</button>
      </div>
    </div>
  </div>
{/if}
