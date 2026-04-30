<script lang="ts">
  import { jobsStore } from "$lib/stores/jobs.svelte";
  import { api } from "$lib/utils/invoke";
  import Button from "$lib/components/primitives/Button.svelte";
  import Badge from "$lib/components/primitives/Badge.svelte";
  import JobDetails from "$lib/components/organisms/JobDetails.svelte";
  import JobWizard from "$lib/components/organisms/JobWizard.svelte";
  import { MagnifyingGlass, Plus, Funnel, Calendar, User, Building, ArrowsClockwise, Briefcase } from "phosphor-svelte";
  import type { Job, JobStatus, Priority } from "$lib/types";

  let searchQuery = $state("");
  let statusFilter = $state<JobStatus | "all">("all");
  let priorityFilter = $state<Priority | "all">("all");
  let companyFilter = $state<string | "all">("all");
  let personFilter = $state<string | "all">("all");
  let selectedJobId = $state<string | null>(null);
  let wizardOpen = $state(false);
  let companies = $state<any[]>([]);
  let people = $state<any[]>([]);

  const filteredJobs = $derived.by(() => {
    let jobs = jobsStore.jobs;

    if (searchQuery) {
      const q = searchQuery.toLowerCase();
      jobs = jobs.filter(j => 
        j.title.toLowerCase().includes(q) || 
        j.description.toLowerCase().includes(q) ||
        j.id.toLowerCase().includes(q)
      );
    }

    if (statusFilter !== "all") {
      jobs = jobs.filter(j => j.status === statusFilter);
    }

    if (priorityFilter !== "all") {
      jobs = jobs.filter(j => j.priority === priorityFilter);
    }

    if (companyFilter !== "all") {
      jobs = jobs.filter(j => j.company_id === companyFilter);
    }

    if (personFilter !== "all") {
      jobs = jobs.filter(j => j.assigned_person_id === personFilter);
    }

    return jobs;
  });

  const groupedJobs = $derived.by(() => {
    const groups: Record<JobStatus, Job[]> = {
      pending: [],
      active: [],
      overdue: [],
      disputed: [],
      resolved: [],
      completed: []
    };

    filteredJobs.forEach(job => {
      groups[job.status].push(job);
    });

    return groups;
  });

  const hasActiveFilters = $derived(
    !!(searchQuery || statusFilter !== "all" || priorityFilter !== "all" || companyFilter !== "all" || personFilter !== "all")
  );

  const allJobsGrouped = $derived.by(() => {
    const groups: Record<JobStatus, number> = {
      pending: 0, active: 0, overdue: 0, disputed: 0, resolved: 0, completed: 0
    };
    jobsStore.jobs.forEach(job => { groups[job.status]++; });
    return groups;
  });

  async function loadFilters() {
    companies = await api.companies.list();
    people = await api.people.list();
  }

  async function syncOverdue() {
    const count = await api.jobs.syncOverdue();
    jobsStore.fetchJobs();
  }

  $effect(() => {
    loadFilters();
  });

  function formatDate(dateStr: string): string {
    return new Date(dateStr).toLocaleDateString('en-IN', { 
      day: 'numeric', 
      month: 'short', 
      year: 'numeric' 
    });
  }
</script>

<div class="max-w-7xl mx-auto space-y-8">
  <!-- Header -->
  <section class="flex items-center justify-between">
    <div>
      <h1 class="font-display text-5xl font-normal tracking-tight mb-2">
        Job <span class="text-muted italic">Board</span>
      </h1>
      <p class="text-slate text-lg">
        Manage all field operations and track job progress across your organization.
      </p>
    </div>
    <div class="flex items-center gap-3">
      <Button variant="outline" size="sm" onclick={syncOverdue} class="gap-2">
        <ArrowsClockwise size={16} /> Sync Overdue
      </Button>
      <Button variant="primary" size="md" onclick={() => wizardOpen = true} class="gap-2">
        <Plus size={18} weight="bold" /> New Job
      </Button>
    </div>
  </section>

  <!-- Filters -->
  <section class="bg-soft-stone/30 border border-hairline rounded-lg p-6 space-y-4">
    <div class="flex items-center gap-2 text-ink font-medium">
      <Funnel size={18} />
      <span class="text-sm">Filters</span>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-5 gap-4">
      <!-- Search -->
      <div class="relative">
        <MagnifyingGlass size={16} class="absolute left-3 top-1/2 -translate-y-1/2 text-muted" />
        <input
          type="text"
          bind:value={searchQuery}
          placeholder="Search jobs..."
          class="w-full pl-10 pr-4 py-2 text-sm border border-hairline rounded-sm focus:outline-none focus:ring-2 focus:ring-form-focus"
        />
      </div>

      <!-- Status Filter -->
      <select
        bind:value={statusFilter}
        class="px-4 py-2 text-sm border border-hairline rounded-sm focus:outline-none focus:ring-2 focus:ring-form-focus"
      >
        <option value="all">All Statuses</option>
        <option value="pending">Pending</option>
        <option value="active">Active</option>
        <option value="overdue">Overdue</option>
        <option value="disputed">Disputed</option>
        <option value="resolved">Resolved</option>
        <option value="completed">Completed</option>
      </select>

      <!-- Priority Filter -->
      <select
        bind:value={priorityFilter}
        class="px-4 py-2 text-sm border border-hairline rounded-sm focus:outline-none focus:ring-2 focus:ring-form-focus"
      >
        <option value="all">All Priorities</option>
        <option value="low">Low</option>
        <option value="medium">Medium</option>
        <option value="high">High</option>
        <option value="critical">Critical</option>
      </select>

      <!-- Company Filter -->
      <select
        bind:value={companyFilter}
        class="px-4 py-2 text-sm border border-hairline rounded-sm focus:outline-none focus:ring-2 focus:ring-form-focus"
      >
        <option value="all">All Companies</option>
        {#each companies as company}
          <option value={company.id}>{company.name}</option>
        {/each}
      </select>

      <!-- Person Filter -->
      <select
        bind:value={personFilter}
        class="px-4 py-2 text-sm border border-hairline rounded-sm focus:outline-none focus:ring-2 focus:ring-form-focus"
      >
        <option value="all">All People</option>
        {#each people as person}
          <option value={person.id}>{person.name}</option>
        {/each}
      </select>
    </div>

    {#if searchQuery || statusFilter !== "all" || priorityFilter !== "all" || companyFilter !== "all" || personFilter !== "all"}
      <div class="flex items-center gap-2">
        <span class="text-xs text-muted">Active filters:</span>
        <button
          onclick={() => {
            searchQuery = "";
            statusFilter = "all";
            priorityFilter = "all";
            companyFilter = "all";
            personFilter = "all";
          }}
          class="text-xs text-action-blue hover:underline"
        >
          Clear all
        </button>
      </div>
    {/if}
  </section>

  <!-- Stats Summary -->
  <section class="grid grid-cols-2 md:grid-cols-6 gap-4">
    {#each Object.entries(allJobsGrouped) as [status, count]}
      <div class="bg-canvas border border-hairline rounded-sm p-4">
        <div class="text-2xl font-display mb-1">{count}</div>
        <div class="text-xs text-muted uppercase tracking-wider">{status}</div>
      </div>
    {/each}
  </section>

  <!-- Job Groups -->
  <section class="space-y-8">
    {#each Object.entries(groupedJobs) as [status, jobs]}
      {#if jobs.length > 0}
        <div class="space-y-4">
          <div class="flex items-center gap-3 border-b border-hairline pb-2">
            <h3 class="font-display text-xl capitalize">{status}</h3>
            <span class="text-xs text-muted">({jobs.length})</span>
          </div>

          <div class="grid grid-cols-1 gap-3">
            {#each jobs as job}
              <button
                onclick={() => selectedJobId = job.id}
                class="bg-canvas border border-hairline hover:border-action-blue rounded-sm p-4 text-left transition-all group"
              >
                <div class="flex items-start justify-between gap-4">
                  <div class="flex-1 space-y-2">
                    <div class="flex items-center gap-3">
                      <span class="text-[10px] font-mono text-muted">{job.id}</span>
                      <Badge text={job.priority} variant={`priority-${job.priority === 'critical' ? 'high' : job.priority}` as any} />
                      <Badge text={job.status} variant={job.status} />
                    </div>
                    <h4 class="font-medium text-ink group-hover:text-action-blue transition-colors">
                      {job.title}
                    </h4>
                    {#if job.description}
                      <p class="text-sm text-slate line-clamp-2">{job.description}</p>
                    {/if}
                  </div>

                  <div class="flex flex-col items-end gap-2 text-xs text-muted">
                    <div class="flex items-center gap-2">
                      <Calendar size={14} />
                      <span>{formatDate(job.deadline)}</span>
                    </div>
                    <div class="flex items-center gap-2">
                      <Building size={14} />
                      <span>{companies.find(c => c.id === job.company_id)?.name || "Unknown"}</span>
                    </div>
                    <div class="flex items-center gap-2">
                      <User size={14} />
                      <span>{people.find(p => p.id === job.assigned_person_id)?.name || "Unassigned"}</span>
                    </div>
                  </div>
                </div>
              </button>
            {/each}
          </div>
        </div>
      {/if}
    {/each}

    {#if filteredJobs.length === 0}
      <div class="bg-soft-stone/30 border border-dashed border-hairline h-64 rounded-lg flex flex-col items-center justify-center text-center">
        <div class="w-12 h-12 rounded-full bg-hairline flex items-center justify-center text-muted mb-4">
          {#if hasActiveFilters}
            <MagnifyingGlass size={24} />
          {:else}
            <Briefcase size={24} />
          {/if}
        </div>
        <h4 class="text-ink font-medium">{hasActiveFilters ? "No jobs match your filters" : "No jobs yet"}</h4>
        <p class="text-slate text-sm w-64 mt-1">
          {#if hasActiveFilters}
            Try adjusting your filters to see more results.
          {:else}
            Create a new job to get started.
          {/if}
        </p>
      </div>
    {/if}
  </section>
</div>

{#if selectedJobId}
  <JobDetails jobId={selectedJobId} onClose={() => selectedJobId = null} />
{/if}

{#if wizardOpen}
  <JobWizard onClose={() => {
    wizardOpen = false;
    jobsStore.fetchJobs();
  }} />
{/if}
