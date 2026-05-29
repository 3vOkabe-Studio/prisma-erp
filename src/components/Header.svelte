<script lang="ts">
  import { Search, Bell, Sun, Moon, Package, Users, FileText } from '@lucide/svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { goto } from '$app/navigation';

  let isDark = $state(false);
  let searchQuery = $state('');
  let searchResults = $state<any[]>([]);
  let isSearching = $state(false);
  let showResults = $state(false);
  let searchTimeout: any;

  let showNotifications = $state(false);
  let lowStockProducts = $state<any[]>([]);

  $effect(() => {
    async function loadNotifications() {
      try {
        lowStockProducts = await invoke('get_low_stock_products');
      } catch(e) { console.error(e); }
    }
    loadNotifications();
  });

  function handleSearchInput() {
    clearTimeout(searchTimeout);
    if (searchQuery.trim() === '') {
      searchResults = [];
      showResults = false;
      return;
    }
    
    searchTimeout = setTimeout(async () => {
      isSearching = true;
      try {
        searchResults = await invoke('global_search', { query: searchQuery });
        showResults = true;
      } catch (e) {
        console.error(e);
      } finally {
        isSearching = false;
      }
    }, 300);
  }

  function handleResultClick(url: string) {
    goto(url);
    showResults = false;
    searchQuery = '';
  }

  // Initialize theme based on document class
  $effect(() => {
    isDark = document.documentElement.classList.contains('dark');
  });

  function toggleTheme() {
    isDark = !isDark;
    if (isDark) {
      document.documentElement.classList.add('dark');
      localStorage.theme = 'dark';
    } else {
      document.documentElement.classList.remove('dark');
      localStorage.theme = 'light';
    }
  }
</script>

<header class="h-16 border-b border-border bg-base/80 backdrop-blur-md sticky top-0 z-10 flex items-center justify-between px-6">
  <!-- Global Search -->
  <div class="flex-1 max-w-md">
    <div class="relative">
      <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
        <Search size={16} class="text-text-muted" />
      </div>
      <input
        type="text"
        bind:value={searchQuery}
        oninput={handleSearchInput}
        onfocus={() => { if(searchQuery.trim() !== '') showResults = true; }}
        onblur={() => setTimeout(() => showResults = false, 200)}
        placeholder="Buscar productos, clientes, facturas... (Ctrl+K)"
        class="block w-full pl-10 pr-3 py-2 border border-border rounded-lg leading-5 bg-surface text-text-main placeholder-text-muted focus:outline-none focus:ring-1 focus:ring-accent focus:border-accent sm:text-sm transition-all"
      />
      
      <!-- Search Results Dropdown -->
      {#if showResults && (searchResults.length > 0 || isSearching)}
        <div class="absolute top-full left-0 right-0 mt-2 bg-surface border border-border rounded-xl shadow-2xl max-h-96 overflow-y-auto z-50">
          {#if isSearching}
            <div class="p-4 text-center text-text-muted text-sm">Buscando...</div>
          {:else}
            {#each searchResults as result}
              <!-- svelte-ignore a11y_click_events_have_key_events -->
              <!-- svelte-ignore a11y_no_static_element_interactions -->
              <div 
                onclick={() => handleResultClick(result.url)}
                class="flex items-center gap-3 p-3 hover:bg-surface-hover cursor-pointer border-b border-border last:border-0 transition-colors"
              >
                <div class="p-2 bg-base rounded-md text-accent">
                  {#if result.result_type === 'PRODUCT'}
                    <Package size={16} />
                  {:else if result.result_type === 'CUSTOMER'}
                    <Users size={16} />
                  {:else}
                    <FileText size={16} />
                  {/if}
                </div>
                <div>
                  <p class="text-sm font-bold text-text-main">{result.title}</p>
                  <p class="text-xs text-text-muted">{result.subtitle}</p>
                </div>
              </div>
            {/each}
          {/if}
        </div>
      {/if}
    </div>
  </div>

  <!-- Actions -->
  <div class="flex items-center gap-4 relative">
    <button onclick={toggleTheme} class="p-2 rounded-md text-text-muted hover:bg-surface-hover hover:text-text-main transition-colors">
      {#if isDark}
        <Sun size={20} />
      {:else}
        <Moon size={20} />
      {/if}
    </button>
    <div class="relative">
      <button 
        onclick={() => showNotifications = !showNotifications} 
        onblur={() => setTimeout(() => showNotifications = false, 200)}
        class="p-2 rounded-md text-text-muted hover:bg-surface-hover hover:text-text-main transition-colors relative"
      >
        <Bell size={20} />
        {#if lowStockProducts.length > 0}
          <span class="absolute top-1.5 right-1.5 block h-2 w-2 rounded-full bg-red-500 ring-2 ring-base"></span>
        {/if}
      </button>

      {#if showNotifications}
        <div class="absolute right-0 mt-2 w-72 bg-surface border border-border rounded-xl shadow-2xl z-50 overflow-hidden">
          <div class="p-3 border-b border-border bg-base/50">
            <h3 class="font-bold text-sm">Notificaciones ({lowStockProducts.length})</h3>
          </div>
          <div class="max-h-64 overflow-y-auto">
            {#if lowStockProducts.length === 0}
              <div class="p-4 text-center text-text-muted text-sm">
                Todo está en orden.
              </div>
            {:else}
              {#each lowStockProducts as p}
                <div class="p-3 border-b border-border hover:bg-surface-hover transition-colors last:border-0 cursor-pointer" onclick={() => goto('/inventory')}>
                  <p class="text-sm font-bold text-text-main">{p.name}</p>
                  <p class="text-xs text-red-500">Stock Crítico: {p.current_stock} (Min: {p.min_stock})</p>
                </div>
              {/each}
            {/if}
          </div>
        </div>
      {/if}
    </div>
  </div>
</header>
