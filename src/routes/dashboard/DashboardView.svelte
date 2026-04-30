<script lang="ts">
  import { onMount } from "svelte";
  import { jobsStore } from "$lib/stores/jobs.svelte";
  import { dashboardStore } from "$lib/stores/dashboard.svelte";
  import StatCard from "$lib/components/primitives/StatCard.svelte";
  import Button from "$lib/components/primitives/Button.svelte";
  import MiniBarChart from "$lib/components/primitives/MiniBarChart.svelte";
  import ActivityFeed from "$lib/components/organisms/ActivityFeed.svelte";
  import JobDetails from "$lib/components/organisms/JobDetails.svelte";
  import { Warning, ArrowRight, Buildings, Users } from "phosphor-svelte";

  let selectedJobId = $state<string | null>(null);
  let statusFilter = $state<string | undefined>(undefined);

  onMount(() => dashboardStore.fetch());

  const stats = $derived(jobsStore.stats);
  const s = $derived(dashboardStore.summary);

  function greeting() {
    const h = new Date().getHours();
    if (h < 12) return 'Good morning';
    if (h < 17) return 'Good afternoon';
    return 'Good evening';
  }
</script>

<div class="max-w-6xl mx-auto space-y-10">

  <!-- Greeting -->
  <section class="pb-2 border-b border-hairline">
    <p class="text-[11px] font-mono uppercase tracking-widest text-muted mb-1">{greeting()}</p>
    <h1 class="font-display text-4xl font-normal tracking-tight text-ink leading-none">
      Gautam.
    </h1>
  </section>

  <!-- Overdue Banner -->
  {#if stats.overdueCount > 0}
    <div class="bg-error/5 border border-error/20 p-5 rounded-lg flex items-center justify-between">
      <div class="flex items-center gap-4">
        <div class="w-9 h-9 rounded-full bg-error/10 flex items-center justify-center text-error shrink-0">
          <Warning size={20} weight="fill" />
        </div>
        <div>
          <p class="text-sm font-medium text-error">
            {stats.overdueCount} job{stats.overdueCount > 1 ? 's' : ''} overdue — immediate review required
          </p>
        </div>
      </div>
      <Button variant="coral" size="sm" onclick={() => { statusFilter = 'overdue'; }} class="gap-1.5 shrink-0">
        View <ArrowRight size={13} />
      </Button>
    </div>
  {/if}

  <!-- Hero Stats -->
  <div class="grid grid-cols-2 lg:grid-cols-4 gap-4">
    <StatCard label="Active Jobs" value={stats.activeJobs} />
    <StatCard label="Overdue" value={stats.overdueCount} variant="overdue" />
    <StatCard label="Completion Rate" value={s ? `${s.completion_rate}%` : `${stats.completionRate}%`} />
    <StatCard label="Active Team" value={stats.activeTeamSize} />
  </div>

  <!-- Middle Row: Company Breakdown + Team Workload -->
  <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">

    <!-- Company Breakdown -->
    <div class="border border-card-border rounded-sm p-6 space-y-4">
      <div class="flex items-center gap-2 mb-2">
        <Buildings size={16} class="text-muted" />
        <h3 class="text-xs font-mono uppercase tracking-widest text-muted">By Company</h3>
      </div>
      {#if dashboardStore.isLoading}
        {#each Array(3) as _}
          <div class="space-y-1.5">
            <div class="h-3 w-32 bg-soft-stone animate-pulse rounded"></div>
            <div class="h-6 w-full bg-soft-stone animate-pulse rounded"></div>
          </div>
        {/each}
      {:else if dashboardStore.topCompanies.length === 0}
        <p class="text-sm text-muted py-4 text-center">No company data yet.</p>
      {:else}
        {#each dashboardStore.topCompanies as co}
          <div class="space-y-1">
            <div class="flex items-center justify-between text-xs">
              <span class="font-medium text-ink truncate">{co.company_name}</span>
              <span class="text-muted shrink-0 ml-2">{co.completed}/{co.total}</span>
            </div>
            <MiniBarChart
              height={24}
              bars={[
                { value: co.completed, color: 'var(--color-action-blue, #1863dc)' },
                { value: co.active,    color: 'var(--color-deep-green, #003c33)' },
                { value: co.overdue,   color: 'var(--color-error, #b30000)' },
              ]}
            />
            {#if co.total > 0}
            <div class="flex gap-3 text-[10px] text-muted">
              <span class="flex items-center gap-1"><span class="w-2 h-2 rounded-full bg-action-blue inline-block"></span>done</span>
              <span class="flex items-center gap-1"><span class="w-2 h-2 rounded-full bg-deep-green inline-block"></span>active</span>
              <span class="flex items-center gap-1"><span class="w-2 h-2 rounded-full bg-error inline-block"></span>overdue</span>
            </div>
            {/if}
          </div>
        {/each}
      {/if}
    </div>

    <!-- Team Workload -->
    <div class="border border-card-border rounded-sm p-6 space-y-3">
      <div class="flex items-center gap-2 mb-2">
        <Users size={16} class="text-muted" />
        <h3 class="text-xs font-mono uppercase tracking-widest text-muted">Team Workload</h3>
      </div>
      {#if dashboardStore.isLoading}
        {#each Array(5) as _}
          <div class="flex items-center gap-3">
            <div class="w-8 h-8 rounded-full bg-soft-stone animate-pulse shrink-0"></div>
            <div class="flex-1 h-3 bg-soft-stone animate-pulse rounded"></div>
          </div>
        {/each}
      {:else if dashboardStore.topPeople.length === 0}
        <p class="text-sm text-muted py-4 text-center">No team data yet.</p>
      {:else}
        {#each dashboardStore.topPeople as person}
          {@const pct = person.total > 0 ? Math.round((person.completed / person.total) * 100) : 0}
          <div class="flex items-center gap-3">
            <div
              class="w-8 h-8 rounded-full flex items-center justify-center text-[10px] font-bold text-on-primary shrink-0"
              style="background-color: {person.avatar_color ?? '#75758a'}"
            >
              {person.initials}
            </div>
            <div class="flex-1 min-w-0">
              <div class="flex items-center justify-between mb-1">
                <span class="text-xs font-medium text-ink truncate">{person.person_name}</span>
                <span class="text-[10px] text-muted shrink-0 ml-2">{person.total} jobs</span>
              </div>
              <div class="h-1.5 bg-soft-stone rounded-full overflow-hidden">
                <div
                  class="h-full rounded-full transition-all duration-500"
                  style="width: {pct}%; background-color: {person.overdue > 0 ? '#b30000' : '#1863dc'}"
                ></div>
              </div>
            </div>
            {#if person.overdue > 0}
              <span class="text-[10px] font-mono text-error shrink-0">{person.overdue} late</span>
            {/if}
          </div>
        {/each}
      {/if}
    </div>
  </div>

  <!-- Recent Activity -->
  <section class="border border-card-border rounded-sm">
    <div class="flex items-center justify-between px-6 py-4 border-b border-hairline">
      <h3 class="text-xs font-mono uppercase tracking-widest text-muted">Recent Activity</h3>
      {#if statusFilter}
        <button
          onclick={() => statusFilter = undefined}
          class="text-[11px] text-muted hover:text-ink underline"
        >
          Clear filter
        </button>
      {/if}
    </div>
    <ActivityFeed onJobClick={(id) => selectedJobId = id} />
  </section>

</div>

{#if selectedJobId}
  <JobDetails jobId={selectedJobId} onClose={() => selectedJobId = null} />
{/if}
