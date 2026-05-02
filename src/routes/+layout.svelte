<script lang="ts">
  import "../app.css";
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount, onDestroy } from "svelte";
  import { uiStore } from "$lib/stores/ui.svelte";

  let { children } = $props();

  // App readiness gate — true only when DB is initialized OR error is detected
  let appReady = $state(false);
  let dbError = $state<string | null>(null);
  let initTimeout = $state(false);

  const unlisteners: (() => void)[] = [];
  let initTimeoutId: ReturnType<typeof setTimeout> | undefined;

  onMount(async () => {
    // TASK 5.5: restore persisted sidebar state
    await uiStore.loadPersistedState();

    // Register event listeners first, then check if already ready (avoids the race
    // where app-ready fires before the listener is registered on fast machines).
    const [unlistenReady, unlistenError] = await Promise.all([
      listen("app-ready", () => { appReady = true; }),
      listen("db-init-error", (event) => {
        dbError = event.payload as string;
        appReady = true;
      }),
    ]);
    unlisteners.push(unlistenReady, unlistenError);

    // Check if the backend already finished init before our listeners were ready
    const alreadyReady = await invoke<boolean>("check_app_ready").catch(() => false);
    if (alreadyReady) {
      appReady = true;
    }

    if (!appReady) {
      // Fallback timeout: if neither event fires within 3s, something is wrong
      initTimeoutId = setTimeout(() => {
        if (!appReady && !dbError) {
          initTimeout = true;
          dbError = "Application initialization timed out. Please restart the app.";
          appReady = true;
        }
      }, 3000);
    }

    // Global error handlers — route to uiStore so errors are visible to the user
    window.addEventListener('unhandledrejection', handleUnhandledRejection);
    window.addEventListener('error', handleUncaughtError);
  });

  onDestroy(() => {
    unlisteners.forEach(fn => fn());
    clearTimeout(initTimeoutId);
    window.removeEventListener('unhandledrejection', handleUnhandledRejection);
    window.removeEventListener('error', handleUncaughtError);
  });

  function handleUnhandledRejection(e: PromiseRejectionEvent) {
    console.error('Unhandled promise rejection:', e.reason);
    uiStore.notify('An unexpected error occurred. Please restart if the app behaves incorrectly.', 'error');
  }

  function handleUncaughtError(e: ErrorEvent) {
    console.error('Uncaught error:', e.error);
    uiStore.notify('An unexpected error occurred. Please restart if the app behaves incorrectly.', 'error');
  }

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
{:else if dbError || initTimeout}
  <div style="position:fixed;top:0;left:0;right:0;bottom:0;width:100vw;height:100vh;display:flex;align-items:center;justify-content:center;background:#fff;z-index:50;transform:translateZ(0);-webkit-transform:translateZ(0)">
    <div style="max-width:24rem;text-align:center;padding:2rem;">
      <div style="width:3rem;height:3rem;border-radius:50%;background:#fee2e2;display:flex;align-items:center;justify-content:center;margin:0 auto 1rem;">
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#b91c1c" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="12" cy="12" r="10"/><line x1="15" y1="9" x2="9" y2="15"/><line x1="9" y1="9" x2="15" y2="15"/>
        </svg>
      </div>
      <h1 style="font-size:1.25rem;font-weight:600;color:#17171c;margin-bottom:0.5rem;">Failed to Start</h1>
      <p style="font-size:0.875rem;color:#71717a;line-height:1.5;margin-bottom:1.5rem;">{dbError}</p>
      <button 
        onclick={() => window.location.reload()}
        style="background:#17171c;color:#fff;padding:0.5rem 1.5rem;border:none;border-radius:0.25rem;font-size:0.875rem;cursor:pointer;"
      >
        Restart App
      </button>
    </div>
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
