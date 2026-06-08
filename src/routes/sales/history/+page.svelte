<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { Printer, Search, FileText } from '@lucide/svelte';
  import InvoiceModal from './InvoiceModal.svelte';

  let invoices = $state<any[]>([]);
  let isLoading = $state(true);
  let searchQuery = $state('');

  let selectedInvoice = $state<any>(null);

  $effect(() => {
    async function loadInvoices() {
      isLoading = true;
      try {
        invoices = await invoke('get_invoices');
      } catch (e) {
        console.error("Error loading invoices:", e);
      } finally {
        isLoading = false;
      }
    }
    loadInvoices();
  });

  let filteredInvoices = $derived(
    invoices.filter(i => 
      i.id.toString().includes(searchQuery) ||
      (i.customer_name && i.customer_name.toLowerCase().includes(searchQuery.toLowerCase()))
    )
  );

  function viewInvoice(invoice: any) {
    selectedInvoice = invoice;
  }
</script>

<div class="h-full flex flex-col space-y-6">
  <div class="flex items-center justify-between">
    <div>
      <h2 class="text-2xl font-bold tracking-tight">Historial de Ventas</h2>
      <p class="text-text-muted text-sm mt-1">Consulta y reimprime facturas emitidas.</p>
    </div>
  </div>

  <div class="flex-1 bg-surface border border-border rounded-xl shadow-sm flex flex-col overflow-hidden">
    <div class="p-4 border-b border-border bg-base/50 flex gap-4">
      <div class="relative flex-1 max-w-md">
        <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
          <Search size={18} class="text-text-muted" />
        </div>
        <input
          type="text"
          bind:value={searchQuery}
          placeholder="Buscar factura por #ID o Cliente..."
          class="block w-full pl-10 pr-3 py-2 border border-border rounded-lg bg-surface text-text-main focus:outline-none focus:border-accent"
        />
      </div>
    </div>

    <div class="flex-1 overflow-auto">
      <table class="w-full text-sm text-left">
        <thead class="text-xs text-text-muted uppercase bg-base sticky top-0 z-10 shadow-sm">
          <tr>
            <th class="px-6 py-4 font-medium">Factura #</th>
            <th class="px-6 py-4 font-medium">Fecha</th>
            <th class="px-6 py-4 font-medium">Cliente</th>
            <th class="px-6 py-4 font-medium">Total</th>
            <th class="px-6 py-4 font-medium">Estado</th>
            <th class="px-6 py-4 font-medium text-right">Acciones</th>
          </tr>
        </thead>
        <tbody>
          {#if isLoading}
            <tr><td colspan="6" class="px-6 py-12 text-center text-text-muted">Cargando facturas...</td></tr>
          {:else if filteredInvoices.length === 0}
            <tr><td colspan="6" class="px-6 py-12 text-center text-text-muted">No se encontraron facturas.</td></tr>
          {:else}
            {#each filteredInvoices as inv}
              <tr class="border-b border-border hover:bg-surface-hover/50 transition-colors">
                <td class="px-6 py-4 font-mono font-bold text-text-main">#{inv.id.toString().padStart(6, '0')}</td>
                <td class="px-6 py-4 text-text-muted">{new Date(inv.created_at).toLocaleString()}</td>
                <td class="px-6 py-4">{inv.customer_name || 'Consumidor Final'}</td>
                <td class="px-6 py-4 font-medium text-accent">${inv.total.toFixed(2)}</td>
                <td class="px-6 py-4">
                  <span class="px-2.5 py-1 text-xs rounded-full font-medium border bg-green-500/10 text-green-500 border-green-500/20">
                    {inv.status}
                  </span>
                </td>
                <td class="px-6 py-4 text-right">
                  <button onclick={() => viewInvoice(inv)} class="px-3 py-1.5 border border-border rounded text-text-muted hover:text-text-main hover:border-text-muted flex items-center gap-2 ml-auto">
                    <FileText size={14} /> Ver / Imprimir
                  </button>
                </td>
              </tr>
            {/each}
          {/if}
        </tbody>
      </table>
    </div>
  </div>
</div>

{#if selectedInvoice}
  <InvoiceModal invoice={selectedInvoice} onClose={() => selectedInvoice = null} />
{/if}
