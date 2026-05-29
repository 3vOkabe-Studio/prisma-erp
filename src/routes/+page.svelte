<script lang="ts">
  import { DollarSign, TrendingUp, Package, Users } from '@lucide/svelte';

  const stats = [
    { name: 'Ventas de Hoy', value: '$2,450.00', change: '+12%', icon: DollarSign },
    { name: 'Ganancias', value: '$840.00', change: '+5%', icon: TrendingUp },
    { name: 'Productos', value: '1,240', change: 'En stock', icon: Package },
    { name: 'Nuevos Clientes', value: '12', change: '+2', icon: Users },
  ];

  const recentInvoices = [
    { id: 'FAC-001', customer: 'Juan Pérez', amount: '$150.00', status: 'Pagada', time: 'hace 5 min' },
    { id: 'FAC-002', customer: 'Empresa ABC', amount: '$450.00', status: 'Pendiente', time: 'hace 2 horas' },
    { id: 'FAC-003', customer: 'María Gómez', amount: '$85.00', status: 'Pagada', time: 'hace 4 horas' },
    { id: 'FAC-004', customer: 'Hotel Plaza', amount: '$1,200.00', status: 'Pagada', time: 'hace 5 horas' },
  ];
</script>

<div class="space-y-6">
  <div>
    <h2 class="text-2xl font-bold tracking-tight">Dashboard</h2>
    <p class="text-text-muted text-sm mt-1">Resumen general de tu negocio hoy.</p>
  </div>

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
          <span class="text-xs font-medium text-green-500">{stat.change}</span>
          <span class="text-xs text-text-muted ml-1">vs ayer</span>
        </div>
      </div>
    {/each}
  </div>

  <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
    <!-- Facturas Recientes -->
    <div class="col-span-2 bg-surface border border-border rounded-xl p-6 shadow-sm">
      <div class="flex items-center justify-between mb-4">
        <h3 class="text-lg font-bold">Ventas Recientes</h3>
        <button class="text-sm text-accent hover:text-accent-hover font-medium">Ver todas</button>
      </div>
      <div class="overflow-x-auto">
        <table class="w-full text-sm text-left">
          <thead class="text-xs text-text-muted uppercase bg-base">
            <tr>
              <th class="px-4 py-3 font-medium rounded-tl-lg">Factura</th>
              <th class="px-4 py-3 font-medium">Cliente</th>
              <th class="px-4 py-3 font-medium">Monto</th>
              <th class="px-4 py-3 font-medium">Estado</th>
              <th class="px-4 py-3 font-medium rounded-tr-lg">Tiempo</th>
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
    </div>

    <!-- Alertas de Stock -->
    <div class="bg-surface border border-border rounded-xl p-6 shadow-sm flex flex-col">
      <h3 class="text-lg font-bold mb-4">Alertas de Inventario</h3>
      <div class="flex-1 flex flex-col items-center justify-center text-center p-6 border-2 border-dashed border-border rounded-lg">
        <Package class="text-text-muted mb-3" size={32} />
        <p class="font-medium text-text-main">Todo en orden</p>
        <p class="text-sm text-text-muted mt-1">Ningún producto está por debajo del stock mínimo.</p>
      </div>
    </div>
  </div>
</div>
