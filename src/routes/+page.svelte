<script lang="ts">
  import { DollarSign, TrendingUp, Package, Users } from '@lucide/svelte';
  import { invoke } from '@tauri-apps/api/core';

  let statsData = $state<any>(null);
  let recentInvoices = $state<any[]>([]);
  let stockAlerts = $state<any[]>([]);
  let isLoading = $state(true);

  $effect(() => {
    async function loadDashboard() {
      try {
        statsData = await invoke('get_dashboard_stats');
        recentInvoices = await invoke('get_recent_invoices');
        stockAlerts = await invoke('get_stock_alerts');
      } catch (e) {
        console.error("Error loading dashboard data:", e);
      } finally {
        isLoading = false;
      }
    }
    loadDashboard();
  });

  // Calculate dynamic stats array based on fetched data
  let stats = $derived([
    { 
      name: 'Ventas de Hoy', 
      value: statsData ? `$${statsData.today_sales.toFixed(2)}` : '$0.00', 
      change: 'Hoy', 
      icon: DollarSign 
    },
    { 
      name: 'Ventas Históricas', 
      value: statsData ? `$${statsData.total_sales.toFixed(2)}` : '$0.00', 
      change: 'Total', 
      icon: TrendingUp 
    },
    { 
      name: 'Productos', 
      value: statsData ? statsData.total_products.toString() : '0', 
      change: 'En catálogo', 
      icon: Package 
    },
    { 
      name: 'Clientes', 
      value: statsData ? statsData.total_customers.toString() : '0', 
      change: 'Registrados', 
      icon: Users 
    },
  ]);
</script>

<div class="space-y-6">
  <div>
    <h2 class="text-2xl font-bold tracking-tight">Dashboard</h2>
    <p class="text-text-muted text-sm mt-1">Resumen general de tu negocio hoy.</p>
  </div>

  {#if isLoading}
    <div class="flex-1 flex items-center justify-center p-12">
      <p class="text-text-muted font-medium">Cargando métricas en tiempo real...</p>
    </div>
  {:else}
    <!-- Stats Grid -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
      {#each stats as stat}
        <div class="bg-surface border border-border rounded-xl p-5 shadow-sm transition-transform hover:scale-[1.02]">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm font-medium text-text-muted">{stat.name}</p>
              <p class="text-2xl font-bold mt-1 text-text-main">{stat.value}</p>
            </div>
            <div class="w-10 h-10 rounded-lg bg-base flex items-center justify-center text-accent">
              <svelte:component this={stat.icon} size={20} />
            </div>
          </div>
          <div class="mt-4">
            <span class="text-xs font-medium text-accent">{stat.change}</span>
          </div>
        </div>
      {/each}
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <!-- Facturas Recientes -->
      <div class="col-span-2 bg-surface border border-border rounded-xl p-6 shadow-sm flex flex-col">
        <div class="flex items-center justify-between mb-4">
          <h3 class="text-lg font-bold">Ventas Recientes</h3>
        </div>
        {#if recentInvoices.length === 0}
          <div class="flex-1 flex items-center justify-center text-text-muted py-8">
            Aún no hay ventas registradas.
          </div>
        {:else}
          <div class="overflow-x-auto">
            <table class="w-full text-sm text-left">
              <thead class="text-xs text-text-muted uppercase bg-base">
                <tr>
                  <th class="px-4 py-3 font-medium rounded-tl-lg">Factura</th>
                  <th class="px-4 py-3 font-medium">Cliente</th>
                  <th class="px-4 py-3 font-medium">Monto</th>
                  <th class="px-4 py-3 font-medium">Estado</th>
                  <th class="px-4 py-3 font-medium rounded-tr-lg">Fecha</th>
                </tr>
              </thead>
              <tbody>
                {#each recentInvoices as invoice}
                  <tr class="border-b border-border hover:bg-surface-hover transition-colors">
                    <td class="px-4 py-3 font-medium text-text-main">{invoice.id}</td>
                    <td class="px-4 py-3">{invoice.customer}</td>
                    <td class="px-4 py-3 font-medium">{invoice.amount}</td>
                    <td class="px-4 py-3">
                      <span class={`px-2.5 py-1 text-xs rounded-full font-medium ${
                        invoice.status === 'Pagada' 
                          ? 'bg-green-100 text-green-700 dark:bg-green-500/10 dark:text-green-400' 
                          : invoice.status === 'Cotización'
                            ? 'bg-blue-100 text-blue-700 dark:bg-blue-500/10 dark:text-blue-400'
                            : 'bg-yellow-100 text-yellow-700 dark:bg-yellow-500/10 dark:text-yellow-400'
                      }`}>
                        {invoice.status}
                      </span>
                    </td>
                    <td class="px-4 py-3 text-text-muted">{invoice.time}</td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        {/if}
      </div>

      <!-- Alertas de Stock -->
      <div class="bg-surface border border-border rounded-xl p-6 shadow-sm flex flex-col">
        <h3 class="text-lg font-bold mb-4">Alertas de Inventario</h3>
        {#if stockAlerts.length === 0}
          <div class="flex-1 flex flex-col items-center justify-center text-center p-6 border-2 border-dashed border-border rounded-lg">
            <Package class="text-green-500 mb-3" size={32} />
            <p class="font-medium text-text-main">Todo en orden</p>
            <p class="text-sm text-text-muted mt-1">Ningún producto está por debajo del stock mínimo.</p>
          </div>
        {:else}
          <div class="flex-1 flex flex-col gap-3">
            {#each stockAlerts as alert}
              <div class="p-3 border border-red-500/20 bg-red-500/5 rounded-lg flex flex-col">
                <span class="font-bold text-sm text-text-main">{alert.name}</span>
                <div class="flex justify-between items-center mt-1 text-xs font-medium">
                  <span class="text-red-500">Stock actual: {alert.current_stock}</span>
                  <span class="text-text-muted">Mínimo: {alert.min_stock}</span>
                </div>
              </div>
            {/each}
          </div>
        {/if}
      </div>
    </div>
  {/if}
</div>
