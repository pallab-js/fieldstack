<script lang="ts">
  import { uiStore } from "../../stores/ui.svelte";
  import { 
    Layout, 
    Briefcase, 
    Users, 
    ChartBar, 
    Gear,
    CaretLeft,
    CaretRight,
    PlusCircle
  } from "phosphor-svelte";
  import { clsx, type ClassValue } from "clsx";
  import { twMerge } from "tailwind-merge";

  function cn(...inputs: ClassValue[]) {
    return twMerge(clsx(inputs));
  }

  const navItems = [
    { id: 'dashboard', label: 'Dashboard', icon: Layout },
    { id: 'jobs', label: 'Job Board', icon: Briefcase },
    { id: 'org', label: 'Organization', icon: Users },
    { id: 'reports', label: 'Reports', icon: ChartBar },
    { id: 'settings', label: 'Settings', icon: Gear },
  ] as const;

  interface Props {
    onNewJob: () => void;
  }

  let { onNewJob }: Props = $props();

  let isCollapsed = $state(false);
</script>

<aside 
  class={cn(
    "flex flex-col border-r border-hairline bg-canvas h-screen transition-all duration-300 relative pt-12",
    isCollapsed ? "w-20" : "w-64"
  )}
>
  <!-- Toggle Button -->
  <button 
    onclick={() => isCollapsed = !isCollapsed}
    class="absolute -right-3 top-20 bg-canvas border border-hairline rounded-full p-1 hover:bg-soft-stone transition-colors z-50"
  >
    {#if isCollapsed}
      <CaretRight size={14} weight="bold" />
    {:else}
      <CaretLeft size={14} weight="bold" />
    {/if}
  </button>

  <!-- Logo Section -->
  <div class="px-6 mb-12 flex items-center gap-3">
    <div class="w-8 h-8 bg-primary rounded-sm flex items-center justify-center shrink-0">
      <span class="text-on-primary font-display font-bold text-lg">F</span>
    </div>
    {#if !isCollapsed}
      <h1 class="font-display text-xl font-medium tracking-tight overflow-hidden whitespace-nowrap">
        Fieldstack
      </h1>
    {/if}
  </div>

  <!-- Navigation Items -->
  <nav class="flex-1 px-3 space-y-1">
    {#each navItems as item}
      <button
        onclick={() => uiStore.activeTab = item.id}
        class={cn(
          "w-full flex items-center gap-3 px-3 py-2.5 rounded-sm transition-all duration-200 group",
          uiStore.activeTab === item.id 
            ? "bg-primary text-on-primary" 
            : "text-muted hover:bg-soft-stone hover:text-ink"
        )}
      >
        <item.icon 
          size={20} 
          weight={uiStore.activeTab === item.id ? "fill" : "regular"}
          class="shrink-0"
        />
        {#if !isCollapsed}
          <span class="text-sm font-medium overflow-hidden whitespace-nowrap">
            {item.label}
          </span>
        {/if}
      </button>
    {/each}
  </nav>

  <!-- Bottom Action -->
  <div class="p-4 mt-auto">
    <button 
      onclick={onNewJob}
      class={cn(
        "w-full flex items-center justify-center gap-2 bg-primary text-on-primary py-3 rounded-pill hover:bg-primary/90 transition-colors",
        isCollapsed ? "px-0" : "px-4"
      )}
    >
      <PlusCircle size={20} weight="bold" />
      {#if !isCollapsed}
        <span class="text-sm font-medium">New Job</span>
      {/if}
    </button>
  </div>
</aside>
