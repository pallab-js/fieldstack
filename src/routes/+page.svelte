<script lang="ts">
  import { onMount } from "svelte";
  import Sidebar from "$lib/components/organisms/Sidebar.svelte";
  import Dashboard from "./dashboard/DashboardView.svelte";
  import OrgView from "./org/OrgView.svelte";
  import ReportsView from "./reports/ReportsView.svelte";
  import JobWizard from "$lib/components/organisms/JobWizard.svelte";
  import { uiStore } from "$lib/stores/ui.svelte";
  import { jobsStore } from "$lib/stores/jobs.svelte";
  import JobBoardView from "./jobs/JobBoardView.svelte";
  import SettingsView from "./settings/SettingsView.svelte";
  import { Bell } from "phosphor-svelte";

  let wizardOpen = $state(false);
  let notifOpen = $state(false);

  // Sync openNewJobWizard flag from Cmd+N shortcut
  $effect(() => {
    if (uiStore.openNewJobWizard) {
      wizardOpen = true;
      uiStore.openNewJobWizard = false;
    }
  });

  onMount(() => {
    jobsStore.fetchJobs();
    
    // Listen for Overdue Sync events from Rust
    // @ts-ignore
    import("@tauri-apps/api/event").then(({ listen }) => {
      listen("overdue-sync", (event) => {
        uiStore.notify(`Sync complete: ${event.payload} jobs marked overdue`, 'info');
        jobsStore.fetchJobs();
      });
    });
  });

  const formattedDate = new Intl.DateTimeFormat('en-IN', {
    weekday: 'long',
    day: 'numeric',
    month: 'long',
    year: 'numeric'
  }).format(new Date());
</script>

<div class="flex h-screen bg-canvas overflow-hidden">
  <Sidebar onNewJob={() => wizardOpen = true} />

  <div class="flex-1 flex flex-col min-w-0">
    <!-- Header -->
    <header class="h-16 border-b border-hairline flex items-center justify-between px-8 shrink-0">
      <div class="flex flex-col">
        <span class="text-[10px] font-mono uppercase tracking-widest text-muted">
          {formattedDate}
        </span>
        <h2 class="text-sm font-medium text-ink">
          {{ dashboard: 'Dashboard', jobs: 'Job Board', org: 'Organization', reports: 'Reports', settings: 'Settings' }[uiStore.activeTab] ?? uiStore.activeTab}
        </h2>
      </div>

      <div class="flex items-center gap-6">
        <div class="relative">
          <button
            onclick={() => notifOpen = !notifOpen}
            class="text-muted hover:text-ink transition-colors relative"
          >
            <Bell size={20} />
            {#if uiStore.notifications.length > 0}
              <span class="absolute -top-1 -right-1 w-2 h-2 bg-coral rounded-full"></span>
            {/if}
          </button>
          {#if notifOpen}
            <div class="absolute right-0 top-8 w-80 bg-canvas border border-hairline rounded-lg shadow-lg z-[300] overflow-hidden">
              <div class="px-4 py-3 border-b border-hairline flex items-center justify-between">
                <span class="text-xs font-mono uppercase tracking-widest text-muted">Notifications</span>
                {#if uiStore.notifications.length > 0}
                  <button onclick={() => uiStore.notifications = []} class="text-[11px] text-muted hover:text-ink">Clear all</button>
                {/if}
              </div>
              {#if uiStore.notifications.length === 0}
                <p class="text-sm text-muted text-center py-8">No notifications</p>
              {:else}
                <div class="max-h-64 overflow-y-auto divide-y divide-hairline">
                  {#each uiStore.notifications as n (n.id)}
                    <div class="px-4 py-3 flex items-start gap-3">
                      <span class="w-2 h-2 rounded-full mt-1.5 shrink-0 {n.type === 'error' ? 'bg-error' : n.type === 'success' ? 'bg-deep-green' : 'bg-action-blue'}"></span>
                      <p class="text-sm text-ink">{n.message}</p>
                    </div>
                  {/each}
                </div>
              {/if}
            </div>
          {/if}
        </div>
        <div class="flex items-center gap-3 border-l border-hairline pl-6">
          <div class="w-8 h-8 rounded-full bg-soft-stone flex items-center justify-center text-[10px] font-bold text-slate">
            GA
          </div>
          <span class="text-xs font-medium text-ink">Gautam</span>
        </div>
      </div>
    </header>

    <!-- Main Content Area -->
    <div class="flex-1 overflow-y-auto p-8">
      {#if uiStore.activeTab === 'dashboard'}
        <Dashboard />
      {:else if uiStore.activeTab === 'org'}
        <OrgView />
      {:else if uiStore.activeTab === 'jobs'}
        <JobBoardView bind:wizardOpen />
      {:else if uiStore.activeTab === 'reports'}
        <ReportsView />
      {:else if uiStore.activeTab === 'settings'}
        <SettingsView />
      {:else}
        <div class="flex flex-col items-center justify-center h-full text-center">
          <h3 class="font-display text-2xl text-muted mb-2">Under Development</h3>
          <p class="text-slate text-sm">This module is coming soon in Phase 1.</p>
        </div>
      {/if}
    </div>

    <!-- Notifications Overlay -->
    {#if uiStore.notifications.length > 0}
      <div style="position:fixed;bottom:2rem;right:2rem;z-index:100;display:flex;flex-direction:column;gap:0.75rem;transform:translateZ(0);-webkit-transform:translateZ(0)">
        {#each uiStore.notifications as n (n.id)}
          <div style="background:#17171c;color:#fff;padding:1rem 1.5rem;border-radius:0.25rem;display:flex;align-items:center;gap:0.75rem;transform:translateZ(0);-webkit-transform:translateZ(0)">
            <span class="text-sm font-medium">{n.message}</span>
          </div>
        {/each}
      </div>
    {/if}

    <!-- Job Wizard Modal -->
    {#if wizardOpen}
      <JobWizard onClose={() => wizardOpen = false} />
    {/if}
  </div>
</div>

<style>
  :global(body) {
    user-select: none;
    cursor: default;
  }
</style>
