<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { X, Printer } from '@lucide/svelte';
  import { onMount } from 'svelte';

  let { invoice, onClose } = $props();
  
  let settings = $state<any>(null);
  let items = $state<any[]>([]);
  let isLoading = $state(true);

  onMount(async () => {
    try {
      settings = await invoke('get_settings');
      items = await invoke('get_invoice_items', { invoiceId: invoice.id });
    } catch(e) {
      console.error(e);
    } finally {
      isLoading = false;
    }
  });

  function printInvoice() {
    window.print();
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm print:bg-white print:backdrop-blur-none" onclick={(e) => { if (e.target === e.currentTarget) onClose() }}>
  <div class="bg-surface print:bg-white border border-border print:border-none rounded-xl shadow-2xl w-full max-w-3xl max-h-[90vh] overflow-y-auto print:max-h-none print:shadow-none print:w-full print:absolute print:top-0 print:left-0 print:h-full">
    
    <!-- Controles (No imprimibles) -->
    <div class="sticky top-0 bg-surface border-b border-border p-4 flex justify-between items-center print:hidden z-10">
      <h3 class="font-bold text-lg">Factura #{invoice.id.toString().padStart(6, '0')}</h3>
      <div class="flex gap-2">
        <button onclick={printInvoice} class="flex items-center gap-2 px-4 py-2 border border-accent text-accent hover:bg-accent hover:text-white rounded transition-colors">
          <Printer size={18} /> Imprimir
        </button>
        <button onclick={onClose} class="p-2 text-text-muted hover:text-text-main rounded hover:bg-surface-hover">
          <X size={20} />
        </button>
      </div>
    </div>

    <!-- Documento Factura (Imprimible) -->
    <div class="p-8 print:p-0 bg-white text-black min-h-[800px] print:min-h-0 text-sm" id="printable-invoice">
      {#if isLoading}
        <div class="text-center py-12 text-gray-500">Cargando datos de la factura...</div>
      {:else}
        
        <!-- Cabecera de Empresa -->
        <div class="flex justify-between items-start border-b-2 border-black pb-6 mb-6">
          <div class="flex gap-4 items-center">
            {#if settings?.logo_base64}
              <img src={settings.logo_base64} alt="Logo" class="max-h-24 max-w-48 object-contain" />
            {/if}
            <div>
              <h1 class="text-2xl font-bold uppercase tracking-wider">{settings?.company_name || 'MI EMPRESA'}</h1>
              {#if settings?.company_address}<p class="text-gray-700">{settings.company_address}</p>{/if}
              {#if settings?.company_phone}<p class="text-gray-700">Tel: {settings.company_phone}</p>{/if}
              {#if settings?.company_email}<p class="text-gray-700">{settings.company_email}</p>{/if}
            </div>
          </div>
          <div class="text-right">
            <h2 class="text-3xl font-light text-gray-400 uppercase tracking-widest mb-2">Factura</h2>
            <div class="flex justify-end gap-4 font-mono">
              <div class="text-gray-500">Número:</div>
              <div class="font-bold">#{invoice.id.toString().padStart(6, '0')}</div>
            </div>
            <div class="flex justify-end gap-4 font-mono">
              <div class="text-gray-500">Fecha:</div>
              <div>{new Date(invoice.created_at).toLocaleDateString()}</div>
            </div>
            <div class="flex justify-end gap-4 font-mono">
              <div class="text-gray-500">Estado:</div>
              <div class="font-bold">{invoice.status}</div>
            </div>
          </div>
        </div>

        <!-- Datos del Cliente -->
        <div class="mb-8 border border-black p-4">
          <h3 class="font-bold border-b border-black pb-2 mb-2 uppercase text-xs tracking-wider">Facturar A:</h3>
          <p class="font-bold text-lg">{invoice.customer_name || 'Consumidor Final'}</p>
        </div>

        <!-- Tabla de Artículos -->
        <table class="w-full mb-8 border-collapse border border-black">
          <thead class="bg-gray-100">
            <tr>
              <th class="border border-black p-2 text-left uppercase text-xs">Descripción del Artículo</th>
              <th class="border border-black p-2 text-center uppercase text-xs w-24">Cant.</th>
              <th class="border border-black p-2 text-right uppercase text-xs w-32">Precio Unit.</th>
              <th class="border border-black p-2 text-right uppercase text-xs w-32">Subtotal</th>
            </tr>
          </thead>
          <tbody>
            {#each items as item}
              <tr>
                <td class="border border-black p-2">{item.product_name || 'Artículo Desconocido'}</td>
                <td class="border border-black p-2 text-center">{item.quantity}</td>
                <td class="border border-black p-2 text-right">${item.price.toFixed(2)}</td>
                <td class="border border-black p-2 text-right">${item.subtotal.toFixed(2)}</td>
              </tr>
            {/each}
          </tbody>
        </table>

        <!-- Totales -->
        <div class="flex justify-end mb-16">
          <div class="w-64">
            <div class="flex justify-between border-b border-black py-2">
              <span class="font-medium">Subtotal:</span>
              <span>${invoice.subtotal.toFixed(2)}</span>
            </div>
            <div class="flex justify-between border-b border-black py-2">
              <span class="font-medium">Descuento:</span>
              <span>${invoice.discount.toFixed(2)}</span>
            </div>
            <div class="flex justify-between border-b border-black py-2">
              <span class="font-medium">Impuestos:</span>
              <span>${invoice.tax.toFixed(2)}</span>
            </div>
            <div class="flex justify-between border-b-4 border-black py-2 font-bold text-lg">
              <span>Total:</span>
              <span>${invoice.total.toFixed(2)}</span>
            </div>
          </div>
        </div>

        <!-- Firma -->
        <div class="mt-auto grid grid-cols-2 gap-8 text-center pt-16">
          <div>
            <div class="border-t border-black pt-2 max-w-xs mx-auto">
              Firma Autorizada
            </div>
          </div>
          <div>
            <div class="border-t border-black pt-2 max-w-xs mx-auto">
              Firma del Cliente
            </div>
          </div>
        </div>

        <!-- Pie de página -->
        {#if settings?.signature_text}
          <div class="mt-12 text-center text-sm text-gray-500 border-t border-gray-300 pt-4">
            {settings.signature_text}
          </div>
        {/if}

      {/if}
    </div>
  </div>
</div>

<style>
  @media print {
    :global(body) {
      visibility: hidden;
      background: white !important;
    }
    #printable-invoice {
      visibility: visible;
      position: absolute;
      left: 0;
      top: 0;
      width: 100%;
    }
    /* Estilos minimalistas forzados en impresión */
    * {
      -webkit-print-color-adjust: exact !important;
      print-color-adjust: exact !important;
    }
  }
</style>
