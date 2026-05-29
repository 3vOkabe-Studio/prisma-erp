<script lang="ts">
  import { ArrowUpRight, ArrowDownRight, Wallet, Lock, Unlock, DollarSign } from '@lucide/svelte';
  import { invoke } from '@tauri-apps/api/core';

  // State
  let isOpen = $state(true);
  let initialAmount = $state(0.00); // Should ideally fetch from latest OPEN
  
  let movements = $state<any[]>([]);
  let currentAmount = $derived(
    initialAmount + movements.reduce((acc, m) => {
      if (m.transaction_type === 'INCOME') return acc + m.amount;
      if (m.transaction_type === 'EXPENSE') return acc - m.amount;
      return acc;
    }, 0)
  );

  $effect(() => {
    async function loadMovements() {
      try {
        movements = await invoke('get_cash_movements');
      } catch (e) {
        console.error("Error loading cash movements:", e);
      }
    }
    loadMovements();
  });

  async function addMovement(type: 'INCOME' | 'EXPENSE') {
    const amountStr = prompt(`Ingrese el monto del ${type === 'INCOME' ? 'INGRESO' : 'EGRESO'}:`);
    if (!amountStr) return;
    const amount = parseFloat(amountStr);
    if (isNaN(amount) || amount <= 0) return alert("Monto inválido");
    
    const notes = prompt("Notas/Motivo del movimiento:");
    
    try {
      await invoke('create_cash_movement', {
        movement: {
          transaction_type: type,
          amount,
          notes: notes || ''
        }
      });
      movements = await invoke('get_cash_movements');
    } catch (e) {
      console.error(e);
      alert("Error al registrar movimiento");
    }
  }
</script>

<div class="space-y-6 h-full flex flex-col">
  <div class="flex items-center justify-between">
    <div>
      <h2 class="text-2xl font-bold tracking-tight">Caja y Efectivo</h2>
      <p class="text-text-muted text-sm mt-1">Control de flujo de caja, ingresos y egresos diarios.</p>
    </div>
    <div class="flex items-center gap-3">
      {#if isOpen}
        <button class="flex items-center gap-2 px-4 py-2 bg-red-500 text-white rounded-lg font-bold hover:bg-red-600 transition-colors shadow-sm">
          <Lock size={18} /> Cerrar Caja
        </button>
      {:else}
        <button class="flex items-center gap-2 px-4 py-2 bg-green-500 text-white rounded-lg font-bold hover:bg-green-600 transition-colors shadow-sm">
          <Unlock size={18} /> Abrir Caja
        </button>
      {/if}
    </div>
  </div>

  <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
    <div class="bg-surface border border-border rounded-xl p-6 shadow-sm flex flex-col justify-between">
      <div class="flex justify-between items-center text-text-muted mb-2">
        <span class="font-medium">Estado Actual</span>
        <Wallet size={20} />
      </div>
      <div>
        <span class={`inline-block px-2.5 py-1 text-xs rounded-full font-bold mb-3 ${isOpen ? 'bg-green-100 text-green-700 dark:bg-green-500/10 dark:text-green-400' : 'bg-red-100 text-red-700 dark:bg-red-500/10 dark:text-red-400'}`}>
          {isOpen ? 'ABIERTA' : 'CERRADA'}
        </span>
        <h3 class="text-4xl font-black text-text-main">${currentAmount.toFixed(2)}</h3>
        <p class="text-sm text-text-muted mt-2">Monto inicial: ${initialAmount.toFixed(2)}</p>
      </div>
    </div>

    <!-- Quick Actions -->
    <div class="md:col-span-2 grid grid-cols-2 gap-4">
      <button onclick={() => addMovement('INCOME')} class="bg-surface border border-border rounded-xl p-6 shadow-sm hover:border-accent hover:bg-surface-hover transition-colors flex items-center justify-center gap-3 flex-col text-text-main group disabled:opacity-50" disabled={!isOpen}>
        <div class="w-12 h-12 rounded-full bg-green-100 dark:bg-green-500/10 flex items-center justify-center text-green-600 dark:text-green-400 group-hover:scale-110 transition-transform">
          <ArrowUpRight size={24} />
        </div>
        <span class="font-bold">Registrar Ingreso</span>
      </button>
      
      <button onclick={() => addMovement('EXPENSE')} class="bg-surface border border-border rounded-xl p-6 shadow-sm hover:border-accent hover:bg-surface-hover transition-colors flex items-center justify-center gap-3 flex-col text-text-main group disabled:opacity-50" disabled={!isOpen}>
        <div class="w-12 h-12 rounded-full bg-red-100 dark:bg-red-500/10 flex items-center justify-center text-red-600 dark:text-red-400 group-hover:scale-110 transition-transform">
          <ArrowDownRight size={24} />
        </div>
        <span class="font-bold">Registrar Egreso (Gasto)</span>
      </button>
    </div>
  </div>

  <!-- Movements History -->
  <div class="flex-1 bg-surface border border-border rounded-xl shadow-sm overflow-hidden flex flex-col">
    <div class="p-4 border-b border-border bg-base/50">
      <h3 class="font-bold text-lg">Historial de Movimientos de Hoy</h3>
    </div>
    <div class="flex-1 overflow-auto p-4 space-y-3">
      {#each movements as movement (movement.id)}
        <div class="flex items-center justify-between p-4 border border-border rounded-lg hover:bg-surface-hover/50 transition-colors bg-base">
          <div class="flex items-center gap-4">
            <div class={`w-10 h-10 rounded-full flex items-center justify-center ${movement.transaction_type === 'INCOME' ? 'bg-green-100 text-green-600 dark:bg-green-500/10 dark:text-green-400' : 'bg-red-100 text-red-600 dark:bg-red-500/10 dark:text-red-400'}`}>
              {#if movement.transaction_type === 'INCOME'}
                <ArrowUpRight size={20} />
              {:else}
                <ArrowDownRight size={20} />
              {/if}
            </div>
            <div>
              <p class="font-bold text-text-main">{movement.notes || 'Movimiento de Caja'}</p>
              <p class="text-xs text-text-muted">{movement.created_at}</p>
            </div>
          </div>
          <div class={`font-bold text-lg ${movement.transaction_type === 'INCOME' ? 'text-green-500' : 'text-red-500'}`}>
            {movement.transaction_type === 'INCOME' ? '+' : '-'}${movement.amount.toFixed(2)}
          </div>
        </div>
      {/each}
    </div>
  </div>
</div>
