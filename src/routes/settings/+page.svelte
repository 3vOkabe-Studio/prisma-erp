<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import { Save, Upload, Building } from '@lucide/svelte';

  let settings = $state({
    company_name: '',
    company_address: '',
    company_phone: '',
    company_email: '',
    logo_base64: '',
    signature_text: '',
  });

  let isLoading = $state(false);
  let successMessage = $state('');

  onMount(async () => {
    try {
      const result = await invoke('get_settings');
      if (result) {
        settings.company_name = result.company_name || '';
        settings.company_address = result.company_address || '';
        settings.company_phone = result.company_phone || '';
        settings.company_email = result.company_email || '';
        settings.logo_base64 = result.logo_base64 || '';
        settings.signature_text = result.signature_text || '';
      }
    } catch (e) {
      console.error('Error cargando configuración:', e);
    }
  });

  async function handleSave() {
    isLoading = true;
    try {
      await invoke('update_settings', { setting: settings });
      successMessage = 'Configuración guardada correctamente.';
      setTimeout(() => successMessage = '', 3000);
    } catch (e) {
      console.error('Error guardando configuración:', e);
      alert('Error guardando configuración');
    } finally {
      isLoading = false;
    }
  }

  function handleLogoUpload(event: Event) {
    const input = event.target as HTMLInputElement;
    if (input.files && input.files[0]) {
      const file = input.files[0];
      const reader = new FileReader();
      reader.onload = (e) => {
        if (e.target && typeof e.target.result === 'string') {
          settings.logo_base64 = e.target.result;
        }
      };
      reader.readAsDataURL(file);
    }
  }
</script>

<div class="p-8 max-w-4xl mx-auto">
  <div class="mb-8 border-b border-border pb-4 flex justify-between items-center">
    <div>
      <h1 class="text-3xl font-light tracking-tight text-text-main flex items-center gap-3">
        <Building class="w-8 h-8 text-text-muted" strokeWidth={1.5} />
        Configuración de Empresa
      </h1>
      <p class="text-text-muted mt-2">Gestiona la información que aparecerá en tus facturas y reportes.</p>
    </div>
    <button 
      onclick={handleSave} 
      disabled={isLoading}
      class="flex items-center gap-2 px-6 py-2 border border-accent text-accent rounded hover:bg-accent hover:text-white transition-colors disabled:opacity-50"
    >
      <Save size={18} />
      {isLoading ? 'Guardando...' : 'Guardar Cambios'}
    </button>
  </div>

  {#if successMessage}
    <div class="mb-6 p-4 border border-green-500 text-green-600 bg-transparent rounded">
      {successMessage}
    </div>
  {/if}

  <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
    <div class="md:col-span-1 border border-border p-6 rounded flex flex-col items-center justify-center gap-4">
      <h3 class="text-lg font-medium text-text-main w-full text-center border-b border-border pb-2 mb-2">Logotipo</h3>
      
      {#if settings.logo_base64}
        <img src={settings.logo_base64} alt="Logo" class="max-w-[200px] max-h-[200px] object-contain border border-border p-2" />
      {:else}
        <div class="w-48 h-48 border border-dashed border-border flex items-center justify-center text-text-muted">
          Sin Logo
        </div>
      {/if}

      <label class="cursor-pointer border border-border px-4 py-2 rounded text-sm hover:border-accent hover:text-accent transition-colors flex items-center gap-2">
        <Upload size={16} />
        Subir Imagen
        <input type="file" accept="image/png, image/jpeg" class="hidden" onchange={handleLogoUpload} />
      </label>
      <p class="text-xs text-text-muted text-center">Recomendado: PNG con fondo transparente.</p>
    </div>

    <div class="md:col-span-2 border border-border p-6 rounded space-y-6">
      <h3 class="text-lg font-medium text-text-main border-b border-border pb-2">Información Fiscal y Contacto</h3>
      
      <div class="grid grid-cols-1 gap-6">
        <div>
          <label class="block text-sm font-medium text-text-muted mb-1">Nombre de la Empresa</label>
          <input 
            type="text" 
            bind:value={settings.company_name} 
            class="w-full bg-transparent border-b border-border px-0 py-2 focus:outline-none focus:border-accent transition-colors text-text-main"
            placeholder="Ej. Prisma ERP S.A."
          />
        </div>

        <div>
          <label class="block text-sm font-medium text-text-muted mb-1">Dirección Principal</label>
          <input 
            type="text" 
            bind:value={settings.company_address} 
            class="w-full bg-transparent border-b border-border px-0 py-2 focus:outline-none focus:border-accent transition-colors text-text-main"
            placeholder="Calle, Número, Ciudad, País"
          />
        </div>

        <div class="grid grid-cols-2 gap-6">
          <div>
            <label class="block text-sm font-medium text-text-muted mb-1">Teléfono</label>
            <input 
              type="text" 
              bind:value={settings.company_phone} 
              class="w-full bg-transparent border-b border-border px-0 py-2 focus:outline-none focus:border-accent transition-colors text-text-main"
              placeholder="+1 234 567 8900"
            />
          </div>
          <div>
            <label class="block text-sm font-medium text-text-muted mb-1">Correo Electrónico</label>
            <input 
              type="email" 
              bind:value={settings.company_email} 
              class="w-full bg-transparent border-b border-border px-0 py-2 focus:outline-none focus:border-accent transition-colors text-text-main"
              placeholder="contacto@empresa.com"
            />
          </div>
        </div>

        <div>
          <label class="block text-sm font-medium text-text-muted mb-1">Texto de Firma / Pie de Factura</label>
          <textarea 
            bind:value={settings.signature_text} 
            rows="3"
            class="w-full bg-transparent border border-border px-3 py-2 rounded focus:outline-none focus:border-accent transition-colors text-text-main resize-none mt-2"
            placeholder="¡Gracias por su compra! Firma autorizada."
          ></textarea>
        </div>
      </div>
    </div>
  </div>
</div>
