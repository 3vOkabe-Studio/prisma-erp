<script lang="ts">
  import '../app.css';
  import Sidebar from '../components/Sidebar.svelte';
  import Header from '../components/Header.svelte';
  let { children } = $props();
</script>

<div class="flex h-screen overflow-hidden bg-base text-text-main">
  <Sidebar />
  <div class="flex-1 flex flex-col overflow-hidden">
    <Header />
    <main class="flex-1 overflow-y-auto p-6 bg-base">
      {@render children()}
    </main>
  </div>
</div>
