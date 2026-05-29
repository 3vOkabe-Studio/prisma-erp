<script lang="ts">
  import { BarChart3, TrendingUp, Package, Users, FileText } from '@lucide/svelte';
  import { invoke } from '@tauri-apps/api/core';

  let stats = $state<any>(null);
  let weeklySales = $state<any[]>([]);
  let isLoading = $state(true);

  $effect(() => {
    async function loadStats() {
      try {
        stats = await invoke('get_dashboard_stats');
        weeklySales = await invoke('get_weekly_sales');
      } catch (e) {
        console.error(e);
      } finally {
        isLoading = false;
      }
    }
    loadStats();
  });
</script>

<div class="h-full flex flex-col space-y-6">
  <!-- Header -->
  <div>
    <h2 class="text-2xl font-bold tracking-tight">Dashboard y Reportes</h2>
    <p class="text-text-muted text-sm mt-1">Resumen general de tu negocio en tiempo real.</p>
  </div>

  {#if isLoading}
    <div class="flex-1 flex items-center justify-center">
      <p class="text-text-muted">Cargando estadísticas...</p>
    </div>
  {:else if stats}
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
      <!-- Total Sales -->
      <div class="bg-surface border border-border p-5 rounded-xl shadow-sm hover:border-accent transition-colors">
        <div class="flex items-center justify-between mb-4">
          <h3 class="text-text-muted font-medium">Ventas Totales (Cobradas)</h3>
          <div class="p-2 bg-green-500/10 text-green-500 rounded-lg">
            <TrendingUp size={20} />
          </div>
        </div>
        <p class="text-3xl font-black text-text-main">${stats.total_sales.toFixed(2)}</p>
      </div>

      <!-- Total Products -->
      <div class="bg-surface border border-border p-5 rounded-xl shadow-sm hover:border-accent transition-colors">
        <div class="flex items-center justify-between mb-4">
          <h3 class="text-text-muted font-medium">Productos Registrados</h3>
          <div class="p-2 bg-blue-500/10 text-blue-500 rounded-lg">
            <Package size={20} />
          </div>
        </div>
        <p class="text-3xl font-black text-text-main">{stats.total_products}</p>
      </div>

      <!-- Total Customers -->
      <div class="bg-surface border border-border p-5 rounded-xl shadow-sm hover:border-accent transition-colors">
        <div class="flex items-center justify-between mb-4">
          <h3 class="text-text-muted font-medium">Clientes Corporativos</h3>
          <div class="p-2 bg-purple-500/10 text-purple-500 rounded-lg">
            <Users size={20} />
          </div>
        </div>
        <p class="text-3xl font-black text-text-main">{stats.total_customers}</p>
      </div>

      <!-- Pending Quotes -->
      <div class="bg-surface border border-border p-5 rounded-xl shadow-sm hover:border-accent transition-colors">
        <div class="flex items-center justify-between mb-4">
          <h3 class="text-text-muted font-medium">Cotizaciones Pendientes</h3>
          <div class="p-2 bg-orange-500/10 text-orange-500 rounded-lg">
            <FileText size={20} />
          </div>
        </div>
        <p class="text-3xl font-black text-text-main">{stats.pending_invoices}</p>
      </div>
    </div>

    <!-- Advanced Charts Section -->
    <div class="bg-surface border border-border p-6 rounded-xl shadow-sm flex-1 flex flex-col">
      <h3 class="text-lg font-bold text-text-main mb-6 flex items-center gap-2">
        <BarChart3 size={20} class="text-accent" />
        Ventas de los Últimos 7 Días
      </h3>
      
      {#if !weeklySales || weeklySales.length === 0}
        <div class="flex-1 flex items-center justify-center text-text-muted">
          No hay datos suficientes para graficar.
        </div>
      {:else}
        <div class="flex-1 flex items-end justify-between gap-2 h-64 mt-auto border-b border-border pb-2">
          {#each weeklySales as day}
            <!-- Calculate height percentage relative to max sales -->
            <div class="flex flex-col items-center flex-1 group">
              <div class="relative w-full flex justify-center h-full items-end pb-2">
                <div 
                  class="w-full max-w-[40px] bg-accent/80 hover:bg-accent rounded-t-md transition-all cursor-pointer relative"
                  style="height: {Math.max((day.total / Math.max(...weeklySales.map(d => d.total))) * 100, 5)}%;"
                >
                  <!-- Tooltip -->
                  <div class="absolute -top-10 left-1/2 -translate-x-1/2 bg-base border border-border px-2 py-1 rounded shadow-lg text-xs font-bold opacity-0 group-hover:opacity-100 transition-opacity whitespace-nowrap z-10 pointer-events-none">
                    ${day.total.toFixed(2)}
                  </div>
                </div>
              </div>
              <span class="text-xs text-text-muted mt-2 font-medium">
                {new Date(day.date).toLocaleDateString('es-ES', { weekday: 'short' })}
              </span>
            </div>
          {/each}
        </div>
      {/if}
    </div>
  {/if}
</div>
