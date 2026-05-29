<script lang="ts">
  import { Plus, Search, Filter, Phone, Mail, MapPin, MoreVertical } from '@lucide/svelte';
  import { invoke } from '@tauri-apps/api/core';

  // State using Runes
  let customers = $state<any[]>([]);
  let isLoading = $state(true);

  $effect(() => {
    async function loadCustomers() {
      isLoading = true;
      try {
        customers = await invoke('get_customers');
      } catch (e) {
        console.error("Error loading customers:", e);
      } finally {
        isLoading = false;
      }
    }
    loadCustomers();
  });

  // Create Modal
  let showCreateModal = $state(false);
  let newCustomer = $state({ name: '', phone: '', email: '', address: '', credit_limit: 0 });

  async function handleCreateCustomer() {
    try {
      if (editingCustomerId) {
        await invoke('update_customer', { customer: { ...newCustomer, id: editingCustomerId } });
      } else {
        await invoke('create_customer', { newCustomer });
      }
      showCreateModal = false;
      editingCustomerId = null;
      newCustomer = { name: '', phone: '', email: '', address: '', credit_limit: 0 };
      customers = await invoke('get_customers');
    } catch (e) {
      console.error(e);
      alert("Error al guardar cliente");
    }
  }

  let editingCustomerId = $state<number | null>(null);

  function openEditModal(customer: any) {
    editingCustomerId = customer.id;
    newCustomer = {
      name: customer.name,
      phone: customer.phone || '',
      email: customer.email || '',
      address: customer.address || '',
      credit_limit: customer.credit_limit || 0
    };
    showCreateModal = true;
  }

  let rawSearchQuery = $state('');
  let searchQuery = $state('');
  let searchTimeout: any;

  $effect(() => {
    clearTimeout(searchTimeout);
    searchTimeout = setTimeout(() => {
      searchQuery = rawSearchQuery;
    }, 250);
  });

  let filteredCustomers = $derived(
    customers.filter(c => 
      c.name.toLowerCase().includes(searchQuery.toLowerCase()) || 
      c.email.toLowerCase().includes(searchQuery.toLowerCase())
    )
  );
</script>

<div class="h-full flex flex-col space-y-6">
  <div class="flex items-center justify-between">
    <div>
      <h2 class="text-2xl font-bold tracking-tight">Directorio de Clientes</h2>
      <p class="text-text-muted text-sm mt-1">Administra tus clientes corporativos y recurrentes.</p>
    </div>
    <div class="flex items-center gap-3">
      <button onclick={() => { editingCustomerId = null; newCustomer = { name: '', phone: '', email: '', address: '', credit_limit: 0 }; showCreateModal = true; }} class="flex items-center gap-2 px-4 py-2 bg-accent text-primary-fg rounded-lg font-bold hover:bg-accent-hover transition-colors shadow-sm">
        <Plus size={18} /> Nuevo Cliente
      </button>
    </div>
  </div>

  <!-- Search Bar -->
  <div class="relative max-w-md">
    <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
      <Search size={18} class="text-text-muted" />
    </div>
    <input
      type="text"
      bind:value={rawSearchQuery}
      placeholder="Buscar por nombre o email..."
      class="block w-full pl-10 pr-3 py-2.5 border border-border rounded-lg leading-5 bg-surface text-text-main placeholder-text-muted focus:outline-none focus:ring-1 focus:ring-accent focus:border-accent sm:text-sm transition-all shadow-sm"
    />
  </div>

  <!-- Customers Grid -->
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 overflow-y-auto pb-4">
    {#each filteredCustomers as customer (customer.id)}
      <div class="bg-surface border border-border rounded-xl p-5 shadow-sm hover:border-accent transition-colors group">
        <div class="flex justify-between items-start mb-4">
          <div class="flex items-center gap-3">
            <div class="w-12 h-12 rounded-full bg-base border border-border flex items-center justify-center font-bold text-xl text-accent">
              {customer.name.charAt(0)}
            </div>
            <div>
              <h3 class="font-bold text-text-main">{customer.name}</h3>
              <p class="text-sm text-text-muted">{customer.email}</p>
            </div>
          </div>
          <button onclick={() => openEditModal(customer)} class="text-text-muted hover:text-text-main opacity-0 group-hover:opacity-100 transition-opacity" title="Editar Cliente">
            <MoreVertical size={18} />
          </button>
        </div>
        
        <div class="space-y-2 mt-4 text-sm text-text-muted">
          <div class="flex items-center gap-2">
            <Phone size={14} /> {customer.phone}
          </div>
          <div class="flex items-center gap-2">
            <MapPin size={14} /> {customer.address}
          </div>
        </div>

        <div class="mt-5 pt-4 border-t border-border flex items-center justify-between">
          <span class="text-sm font-medium">Deuda / Balance</span>
          <span class={`font-bold ${customer.balance > 0 ? 'text-red-500' : 'text-green-500'}`}>
            ${customer.balance.toFixed(2)}
          </span>
        </div>
      </div>
    {/each}
  </div>
</div>

{#if showCreateModal}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm" onclick={(e) => { if (e.target === e.currentTarget) showCreateModal = false }}>
    <div class="bg-surface border border-border rounded-xl shadow-2xl p-6 w-full max-w-md">
      <h3 class="text-xl font-bold mb-4">{editingCustomerId ? 'Editar Cliente' : 'Crear Nuevo Cliente'}</h3>
      <div class="space-y-4">
        <div>
          <label class="block text-sm font-medium text-text-muted mb-1" for="name">Nombre / Empresa</label>
          <input id="name" type="text" bind:value={newCustomer.name} class="w-full px-3 py-2 border border-border rounded-md bg-base text-text-main" />
        </div>
        <div class="grid grid-cols-2 gap-4">
          <div>
            <label class="block text-sm font-medium text-text-muted mb-1" for="phone">Teléfono</label>
            <input id="phone" type="text" bind:value={newCustomer.phone} class="w-full px-3 py-2 border border-border rounded-md bg-base text-text-main" />
          </div>
          <div>
            <label class="block text-sm font-medium text-text-muted mb-1" for="email">Email</label>
            <input id="email" type="email" bind:value={newCustomer.email} class="w-full px-3 py-2 border border-border rounded-md bg-base text-text-main" />
          </div>
        </div>
        <div>
          <label class="block text-sm font-medium text-text-muted mb-1" for="address">Dirección</label>
          <input id="address" type="text" bind:value={newCustomer.address} class="w-full px-3 py-2 border border-border rounded-md bg-base text-text-main" />
        </div>
      </div>
      <div class="mt-6 flex justify-end gap-3">
        <button onclick={() => showCreateModal = false} class="px-4 py-2 border border-border rounded-md font-medium hover:bg-surface-hover">Cancelar</button>
        <button onclick={handleCreateCustomer} class="px-4 py-2 bg-accent text-primary-fg rounded-md font-bold hover:bg-accent-hover">Guardar</button>
      </div>
    </div>
  </div>
{/if}
