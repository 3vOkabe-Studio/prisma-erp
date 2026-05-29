<script lang="ts">
  import { ReceiptText, Plus } from '@lucide/svelte';
  import { invoke } from '@tauri-apps/api/core';

  let expenses = $state<any[]>([]);
  
  $effect(() => {
    async function loadExpenses() {
      try {
        expenses = await invoke('get_expenses');
      } catch (e) {
        console.error(e);
      }
    }
    loadExpenses();
  });

  let showCreateModal = $state(false);
  let newExpense = $state({ category: 'Operativo', amount: 0, notes: '' });

  async function handleCreateExpense() {
    try {
      await invoke('create_expense', { newExpense });
      showCreateModal = false;
      newExpense = { category: 'Operativo', amount: 0, notes: '' };
      expenses = await invoke('get_expenses');
    } catch (e) {
      console.error(e);
      alert("Error al crear gasto");
    }
  }
</script>

<div class="h-full flex flex-col space-y-6">
  <div class="flex items-center justify-between">
    <div>
      <h2 class="text-2xl font-bold tracking-tight">Gastos</h2>
      <p class="text-text-muted text-sm mt-1">Registro de egresos operativos.</p>
    </div>
    <button onclick={() => showCreateModal = true} class="flex items-center gap-2 px-4 py-2 bg-accent text-primary-fg rounded-lg font-bold hover:bg-accent-hover transition-colors shadow-sm">
      <Plus size={18} /> Nuevo Gasto
    </button>
  </div>

  <div class="flex-1 bg-surface border border-border rounded-xl shadow-sm flex flex-col overflow-hidden p-4">
    <table class="w-full text-sm text-left">
      <thead class="text-xs text-text-muted uppercase bg-base sticky top-0">
        <tr>
          <th class="px-6 py-4 font-medium">Fecha</th>
          <th class="px-6 py-4 font-medium">Categoría</th>
          <th class="px-6 py-4 font-medium">Notas</th>
          <th class="px-6 py-4 font-medium">Monto</th>
        </tr>
      </thead>
      <tbody>
        {#each expenses as exp}
          <tr class="border-b border-border hover:bg-surface-hover/50">
            <td class="px-6 py-4">{exp.date}</td>
            <td class="px-6 py-4 font-medium">{exp.category}</td>
            <td class="px-6 py-4 text-text-muted">{exp.notes}</td>
            <td class="px-6 py-4 font-bold text-red-500">${exp.amount.toFixed(2)}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</div>

{#if showCreateModal}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm" onclick={(e) => { if (e.target === e.currentTarget) showCreateModal = false }}>
    <div class="bg-surface border border-border rounded-xl shadow-2xl p-6 w-full max-w-md">
      <h3 class="text-xl font-bold mb-4">Registrar Gasto</h3>
      <div class="space-y-4">
        <div>
          <label class="block text-sm font-medium text-text-muted mb-1" for="category">Categoría</label>
          <input id="category" type="text" bind:value={newExpense.category} class="w-full px-3 py-2 border border-border rounded-md bg-base text-text-main" />
        </div>
        <div>
          <label class="block text-sm font-medium text-text-muted mb-1" for="amount">Monto</label>
          <input id="amount" type="number" step="0.01" bind:value={newExpense.amount} class="w-full px-3 py-2 border border-border rounded-md bg-base text-text-main" />
        </div>
        <div>
          <label class="block text-sm font-medium text-text-muted mb-1" for="notes">Notas</label>
          <input id="notes" type="text" bind:value={newExpense.notes} class="w-full px-3 py-2 border border-border rounded-md bg-base text-text-main" />
        </div>
      </div>
      <div class="mt-6 flex justify-end gap-3">
        <button onclick={() => showCreateModal = false} class="px-4 py-2 border border-border rounded-md font-medium hover:bg-surface-hover">Cancelar</button>
        <button onclick={handleCreateExpense} class="px-4 py-2 bg-accent text-primary-fg rounded-md font-bold hover:bg-accent-hover">Guardar</button>
      </div>
    </div>
  </div>
{/if}
