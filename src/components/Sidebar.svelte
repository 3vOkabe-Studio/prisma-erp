<script lang="ts">
  import {
    LayoutDashboard,
    ShoppingCart,
    Package,
    Users,
    Truck,
    Wallet,
    ReceiptText,
    BarChart3
  } from '@lucide/svelte';

  const menuItems = [
    { name: 'Dashboard', icon: LayoutDashboard, href: '/' },
    { name: 'Ventas (POS)', icon: ShoppingCart, href: '/sales' },
    { name: 'Inventario', icon: Package, href: '/inventory' },
    { name: 'Clientes', icon: Users, href: '/customers' },
    { name: 'Suplidores', icon: Truck, href: '/suppliers' },
    { name: 'Caja', icon: Wallet, href: '/cash' },
    { name: 'Gastos', icon: ReceiptText, href: '/expenses' },
    { name: 'Reportes', icon: BarChart3, href: '/reports' },
  ];
</script>

<aside class="w-64 h-screen bg-surface border-r border-border flex flex-col sticky top-0">
  <div class="p-6 flex items-center gap-3">
    <div class="w-8 h-8 rounded-lg bg-accent text-primary-fg flex items-center justify-center font-bold text-xl">
      P
    </div>
    <h1 class="text-xl font-bold tracking-tight">Prisma</h1>
  </div>

  <nav class="flex-1 px-4 py-2 space-y-1 overflow-y-auto">
    {#each menuItems as item}
      <a
        href={item.href}
        class="flex items-center gap-3 px-3 py-2 rounded-md text-text-muted hover:text-text-main hover:bg-surface-hover transition-colors font-medium text-sm"
      >
        <svelte:component this={item.icon} size={18} />
        {item.name}
      </a>
    {/each}
  </nav>

  <div class="p-4 border-t border-border">
    <div class="flex items-center gap-3 px-3 py-2">
      <div class="w-8 h-8 rounded-full bg-border flex items-center justify-center text-xs font-medium">
        AD
      </div>
      <div>
        <p class="text-sm font-medium">Admin</p>
        <p class="text-xs text-text-muted">admin@prisma.erp</p>
      </div>
    </div>
  </div>
</aside>
