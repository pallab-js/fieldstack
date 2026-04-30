<script lang="ts">
  import "../app.css";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import { uiStore } from "$lib/stores/ui.svelte";

  let { children } = $props();

  // TASK 5.1: app readiness gate
  let appReady = $state(false);

  onMount(async () => {
    // TASK 5.5: restore persisted sidebar state
    await uiStore.loadPersistedState();

    // Listen for app-ready event from Rust (emitted after DB init)
    await listen("app-ready", () => {
      appReady = true;
    });

    // Fallback: blocking init means pool is always ready before window opens,
    // but set true after short delay in case event fired before listener attached
    setTimeout(() => { appReady = true; }, 500);
  });

  // TASK 5.4: global keyboard shortcut — Cmd+N opens new job wizard
  function handleKeydown(e: KeyboardEvent) {
    if ((e.metaKey || e.ctrlKey) && e.key === 'n') {
      e.preventDefault();
      uiStore.activeTab = 'jobs';
      uiStore.openNewJobWizard = true;
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if !appReady}
  <div style="position:fixed;top:0;left:0;right:0;bottom:0;width:100vw;height:100vh;display:flex;align-items:center;justify-content:center;background:#fff;z-index:50;transform:translateZ(0);-webkit-transform:translateZ(0)">
    <p class="text-sm text-gray-500">Starting Fieldstack...</p>
  </div>
{:else}
  <div class="titlebar-drag"></div>
  <main class="min-h-screen bg-canvas selection:bg-coral/30">
    {@render children()}
  </main>
{/if}

<style>
  :global(html, body) {
    margin: 0;
    padding: 0;
    overflow-x: hidden;
  }
</style>
