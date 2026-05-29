<script lang="ts">
  import { Truck, Plus } from '@lucide/svelte';
  import { invoke } from '@tauri-apps/api/core';

  let suppliers = $state<any[]>([]);
  
  $effect(() => {
    async function loadSuppliers() {
      try {
        suppliers = await invoke('get_suppliers');
      } catch (e) {
        console.error(e);
      }
    }
    loadSuppliers();
  });

  let showCreateModal = $state(false);
  let newSupplier = $state({ name: '', phone: '', email: '' });

  async function handleCreateSupplier() {
    try {
      await invoke('create_supplier', { newSupplier });
      showCreateModal = false;
      newSupplier = { name: '', phone: '', email: '' };
      suppliers = await invoke('get_suppliers');
    } catch (e) {
      console.error(e);
      alert("Error al crear suplidor");
    }
  }
</script>

<div class="h-full flex flex-col space-y-6">
  <div class="flex items-center justify-between">
    <div>
      <h2 class="text-2xl font-bold tracking-tight">Suplidores</h2>
      <p class="text-text-muted text-sm mt-1">Gestión de proveedores.</p>
    </div>
    <button onclick={() => showCreateModal = true} class="flex items-center gap-2 px-4 py-2 bg-accent text-primary-fg rounded-lg font-bold hover:bg-accent-hover transition-colors shadow-sm">
      <Plus size={18} /> Nuevo Suplidor
    </button>
  </div>

  <div class="flex-1 bg-surface border border-border rounded-xl shadow-sm flex flex-col overflow-hidden p-4">
    <table class="w-full text-sm text-left">
      <thead class="text-xs text-text-muted uppercase bg-base sticky top-0">
        <tr>
          <th class="px-6 py-4 font-medium">Nombre</th>
          <th class="px-6 py-4 font-medium">Teléfono</th>
          <th class="px-6 py-4 font-medium">Balance</th>
        </tr>
      </thead>
      <tbody>
        {#each suppliers as supplier}
          <tr class="border-b border-border hover:bg-surface-hover/50">
            <td class="px-6 py-4 font-bold">{supplier.name}</td>
            <td class="px-6 py-4">{supplier.phone}</td>
            <td class="px-6 py-4">${supplier.balance.toFixed(2)}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</div>

{#if showCreateModal}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm" onclick={(e) => { if (e.target === e.currentTarget) showCreateModal = false }}>
    <div class="bg-surface border border-border rounded-xl shadow-2xl p-6 w-full max-w-md">
      <h3 class="text-xl font-bold mb-4">Crear Suplidor</h3>
      <div class="space-y-4">
        <div>
          <label class="block text-sm font-medium text-text-muted mb-1" for="name">Nombre</label>
          <input id="name" type="text" bind:value={newSupplier.name} class="w-full px-3 py-2 border border-border rounded-md bg-base text-text-main" />
        </div>
        <div>
          <label class="block text-sm font-medium text-text-muted mb-1" for="phone">Teléfono</label>
          <input id="phone" type="text" bind:value={newSupplier.phone} class="w-full px-3 py-2 border border-border rounded-md bg-base text-text-main" />
        </div>
      </div>
      <div class="mt-6 flex justify-end gap-3">
        <button onclick={() => showCreateModal = false} class="px-4 py-2 border border-border rounded-md font-medium hover:bg-surface-hover">Cancelar</button>
        <button onclick={handleCreateSupplier} class="px-4 py-2 bg-accent text-primary-fg rounded-md font-bold hover:bg-accent-hover">Guardar</button>
      </div>
    </div>
  </div>
{/if}
