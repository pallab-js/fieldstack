<script lang="ts">
  import { onMount } from "svelte";
  import { api } from "$lib/utils/invoke";
  import { uiStore } from "$lib/stores/ui.svelte";
  import Button from "$lib/components/primitives/Button.svelte";
  import { Gear, Database, Warning } from "phosphor-svelte";

  let isLoading = $state(true);
  let showResetModal = $state(false);
  let isResetting = $state(false);
  let resetConfirmation = $state('');

  onMount(async () => { await loadConfig(); });

  async function loadConfig() {
    isLoading = true;
    try {
      await api.settings.getConfig();
    } catch (e) {
      uiStore.notify('Failed to load settings', 'error');
    } finally {
      isLoading = false;
    }
  }

  async function confirmReset() {
    isResetting = true;
    try {
      await api.settings.resetJobData(resetConfirmation);
      uiStore.notify('All job data has been reset', 'success');
      showResetModal = false;
      resetConfirmation = '';
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
      Manage your local database and application preferences.
    </p>
  </section>

  <!-- Database Section -->
  <section class="space-y-6">
    <div class="flex items-center gap-3 border-b border-hairline pb-4">
      <Gear size={24} class="text-muted" />
      <h3 class="font-display text-2xl">Database</h3>
    </div>

    {#if isLoading}
      <div class="space-y-4">
        {#each Array(3) as _}
          <div class="h-16 bg-soft-stone animate-pulse rounded-sm"></div>
        {/each}
      </div>
    {:else}
      <div class="bg-soft-stone/30 border border-dashed border-hairline p-8 rounded-lg text-center">
        <p class="text-slate text-sm">Configuration is managed locally. Settings are persisted in the app database.</p>
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
          Permanently delete all jobs, proofs, and drafts. Companies, people, and audit log will be preserved.
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
  <div style="position:fixed;top:0;left:0;right:0;bottom:0;width:100vw;height:100vh;z-index:200;background:rgba(23,23,28,0.8);display:flex;align-items:center;justify-content:center;transform:translateZ(0);-webkit-transform:translateZ(0);-webkit-backface-visibility:hidden;contain:none">
    <div style="background:#fff;width:100%;min-width:0;max-width:28rem;border-radius:0.5rem;border:1px solid #d9d9dd;overflow:hidden;margin:1rem;transform:translateZ(0);-webkit-transform:translateZ(0)">
      <div class="p-6 border-b border-hairline">
        <h3 class="font-display text-xl flex items-center gap-2">
          <Warning size={24} class="text-error" weight="fill" />
          Confirm Reset
        </h3>
      </div>
      <div class="p-6 space-y-4">
        <p class="text-slate text-sm">This action will permanently delete:</p>
        <ul class="text-slate text-sm space-y-1 list-disc list-inside">
          <li>All jobs and their proofs</li>
          <li>All saved drafts</li>
        </ul>
        <p class="text-error text-sm font-medium">This action cannot be undone.</p>
        <div class="space-y-2 pt-2">
          <label for="reset-confirm" class="text-xs text-slate">
            Type <code class="text-error font-mono font-bold">DELETE ALL JOBS</code> to confirm:
          </label>
          <input
            id="reset-confirm"
            type="text"
            bind:value={resetConfirmation}
            placeholder="DELETE ALL JOBS"
            class="w-full bg-canvas border border-error/30 rounded-sm px-3 py-2 text-sm text-error outline-none focus:ring-1 focus:ring-error"
          />
        </div>
      </div>
      <div class="p-6 border-t border-hairline flex gap-3 justify-end">
        <Button variant="outline" size="sm" onclick={() => showResetModal = false} disabled={isResetting}>
          Cancel
        </Button>
        <Button variant="coral" size="sm" onclick={confirmReset} disabled={isResetting || resetConfirmation !== 'DELETE ALL JOBS'} class="gap-2">
          {#if isResetting}Resetting...{:else}<Warning size={16} weight="fill" /> Confirm Reset{/if}
        </Button>
      </div>
    </div>
  </div>
{/if}
