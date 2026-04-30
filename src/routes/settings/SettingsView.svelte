<script lang="ts">
  import { onMount } from "svelte";
  import { api } from "$lib/utils/invoke";
  import { uiStore } from "$lib/stores/ui.svelte";
  import Button from "$lib/components/primitives/Button.svelte";
  import { Gear, Database, Warning, Check } from "phosphor-svelte";

  let configs = $state<{ key: string; value: string }[]>([]);
  let isLoading = $state(true);
  let showResetModal = $state(false);
  let isResetting = $state(false);

  onMount(async () => {
    await loadConfig();
  });

  async function loadConfig() {
    isLoading = true;
    try {
      configs = await api.settings.getConfig();
    } catch (e) {
      uiStore.notify(`Failed to load settings: ${e}`, 'error');
    } finally {
      isLoading = false;
    }
  }

  async function saveConfig(key: string, value: string) {
    try {
      await api.settings.setConfig(key, value);
      uiStore.notify('Setting saved', 'success');
      await loadConfig();
    } catch (e) {
      uiStore.notify(`Failed to save: ${e}`, 'error');
    }
  }

  async function confirmReset() {
    isResetting = true;
    try {
      await api.settings.resetJobData();
      uiStore.notify('All job data has been reset', 'success');
      showResetModal = false;
      // Reload jobs store if needed
      window.location.reload();
    } catch (e) {
      uiStore.notify(`Reset failed: ${e}`, 'error');
    } finally {
      isResetting = false;
    }
  }
</script>

<div class="max-w-4xl mx-auto space-y-12">
  <!-- Hero Section -->
  <section>
    <h1 class="font-display text-5xl font-normal tracking-tight mb-4">
      System <span class="text-muted italic">Settings</span>
    </h1>
    <p class="text-slate text-lg max-w-2xl leading-relaxed">
      Configure application preferences and manage your local database.
    </p>
  </section>

  <!-- Configuration Section -->
  <section class="space-y-6">
    <div class="flex items-center gap-3 border-b border-hairline pb-4">
      <Gear size={24} class="text-muted" />
      <h3 class="font-display text-2xl">Configuration</h3>
    </div>

    {#if isLoading}
      <div class="space-y-4">
        {#each Array(3) as _}
          <div class="h-16 bg-soft-stone animate-pulse rounded-sm"></div>
        {/each}
      </div>
    {:else if configs.length === 0}
      <div class="bg-soft-stone/30 border border-dashed border-hairline p-8 rounded-lg text-center">
        <p class="text-slate text-sm">No configuration keys found. Settings will appear here once created.</p>
      </div>
    {:else}
      <div class="space-y-4">
        {#each configs as config}
          <div class="bg-canvas border border-hairline p-4 rounded-sm flex items-center justify-between">
            <div class="flex-1">
              <label for="config-{config.key}" class="text-[10px] font-mono uppercase tracking-widest text-muted mb-1 block">
                {config.key}
              </label>
              <input
                id="config-{config.key}"
                type="text"
                value={config.value}
                onchange={(e) => saveConfig(config.key, e.currentTarget.value)}
                class="w-full bg-transparent text-ink text-sm border-none outline-none focus:ring-0 p-0"
              />
            </div>
            <Check size={16} class="text-muted" />
          </div>
        {/each}
      </div>
    {/if}
  </section>

  <!-- Danger Zone -->
  <section class="space-y-6">
    <div class="flex items-center gap-3 border-b border-error/20 pb-4">
      <Database size={24} class="text-error" />
      <h3 class="font-display text-2xl text-error">Danger Zone</h3>
    </div>

    <div class="bg-error/5 border border-error/20 p-6 rounded-lg space-y-4">
      <div>
        <h4 class="text-ink font-medium mb-1">Reset Job Data</h4>
        <p class="text-slate text-sm">
          Permanently delete all jobs, proofs, audit logs, and drafts. Companies and people will be preserved.
        </p>
      </div>
      <Button variant="coral" size="sm" onclick={() => showResetModal = true} class="gap-2">
        <Warning size={16} weight="fill" /> Reset Database
      </Button>
    </div>
  </section>
</div>

<!-- Reset Confirmation Modal -->
{#if showResetModal}
  <div class="fixed inset-0 bg-primary/80 flex items-center justify-center z-[200] animate-in fade-in">
    <div class="bg-canvas rounded-lg border border-hairline max-w-md w-full mx-4 animate-in zoom-in-95 duration-200">
      <div class="p-6 border-b border-hairline">
        <h3 class="font-display text-xl flex items-center gap-2">
          <Warning size={24} class="text-error" weight="fill" />
          Confirm Reset
        </h3>
      </div>
      <div class="p-6 space-y-4">
        <p class="text-slate text-sm">
          This action will permanently delete:
        </p>
        <ul class="text-slate text-sm space-y-1 list-disc list-inside">
          <li>All jobs and their proofs</li>
          <li>Complete audit history</li>
          <li>All saved drafts</li>
        </ul>
        <p class="text-error text-sm font-medium">
          This action cannot be undone.
        </p>
      </div>
      <div class="p-6 border-t border-hairline flex gap-3 justify-end">
        <Button variant="outline" size="sm" onclick={() => showResetModal = false} disabled={isResetting}>
          Cancel
        </Button>
        <Button variant="coral" size="sm" onclick={confirmReset} disabled={isResetting} class="gap-2">
          {#if isResetting}
            Resetting...
          {:else}
            <Warning size={16} weight="fill" /> Confirm Reset
          {/if}
        </Button>
      </div>
    </div>
  </div>
{/if}
