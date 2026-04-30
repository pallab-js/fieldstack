<script lang="ts">
  import { jobsStore } from "../../stores/jobs.svelte";
  import Badge from "../primitives/Badge.svelte";
  import Skeleton from "../primitives/Skeleton.svelte";
  import EmptyState from "../primitives/EmptyState.svelte";
  import { MagnifyingGlass, Funnel, DotsThreeOutlineVertical, Briefcase } from "phosphor-svelte";
  import { clsx, type ClassValue } from "clsx";
  import { twMerge } from "tailwind-merge";

  function cn(...inputs: ClassValue[]) {
    return twMerge(clsx(inputs));
  }

  interface Props {
    onJobClick: (id: string) => void;
    statusFilter?: string;
  }

  let { onJobClick, statusFilter: externalFilter }: Props = $props();

  let searchQuery = $state("");
  let statusFilter = $state("all");

  // Sync external filter into internal state
  $effect(() => {
    if (externalFilter) statusFilter = externalFilter;
  });

  const filteredJobs = $derived(
    jobsStore.jobs.filter(job => {
      const matchesSearch = job.title.toLowerCase().includes(searchQuery.toLowerCase()) || 
                           job.id.toLowerCase().includes(searchQuery.toLowerCase());
      const matchesStatus = statusFilter === 'all' || job.status === statusFilter;
      return matchesSearch && matchesStatus;
    })
  );

  const tableHeaders = [
    { label: "Job ID", class: "w-24" },
    { label: "Description", class: "flex-1" },
    { label: "Status", class: "w-32" },
    { label: "Priority", class: "w-32" },
    { label: "Deadline", class: "w-40" },
    { label: "", class: "w-10" },
  ];
</script>

<div class="space-y-6">
  <!-- Filters Bar -->
  <div class="flex items-center justify-between gap-4">
    <div class="relative flex-1 max-w-md">
      <MagnifyingGlass size={18} class="absolute left-3 top-1/2 -translate-y-1/2 text-muted" />
      <input 
        type="text" 
        bind:value={searchQuery}
        placeholder="Search by ID or title..."
        class="w-full bg-soft-stone/30 border border-hairline rounded-sm pl-10 pr-4 py-2 text-sm focus:border-form-focus outline-none transition-colors"
      />
    </div>

    <div class="flex items-center gap-2">
      <Funnel size={18} class="text-muted" />
      <select 
        bind:value={statusFilter}
        class="bg-canvas border border-hairline rounded-sm px-3 py-2 text-xs font-medium focus:border-form-focus outline-none"
      >
        <option value="all">All Statuses</option>
        <option value="active">Active</option>
        <option value="overdue">Overdue</option>
        <option value="completed">Completed</option>
        <option value="disputed">Disputed</option>
      </select>
    </div>
  </div>

  <!-- Table -->
  <div class="border border-hairline rounded-sm overflow-hidden bg-canvas">
    <table class="w-full text-left border-collapse">
      <thead>
        <tr class="bg-soft-stone/30 border-b border-hairline">
          {#each tableHeaders as header}
            <th class={cn("px-6 py-4 text-[10px] font-mono uppercase tracking-widest text-muted", header.class)}>
              {header.label}
            </th>
          {/each}
        </tr>
      </thead>
      <tbody>
        {#if jobsStore.isLoading}
          {#each Array(5) as _}
            <tr class="border-b border-hairline">
              <td class="px-6 py-5"><Skeleton class="h-4 w-16" /></td>
              <td class="px-6 py-5"><Skeleton class="h-5 w-48" /></td>
              <td class="px-6 py-5"><Skeleton class="h-6 w-20 rounded-full" /></td>
              <td class="px-6 py-5"><Skeleton class="h-6 w-20 rounded-full" /></td>
              <td class="px-6 py-5"><Skeleton class="h-4 w-24" /></td>
              <td class="px-6 py-5 text-right"><Skeleton class="h-4 w-4 ml-auto" /></td>
            </tr>
          {/each}
        {:else}
          {#each filteredJobs as job (job.id)}
            <tr 
              onclick={() => onJobClick(job.id)}
              class="group border-b border-hairline last:border-0 hover:bg-soft-stone/10 transition-colors cursor-pointer"
            >
              <td class="px-6 py-5">
                <span class="text-xs font-mono font-medium text-slate">{job.id}</span>
              </td>
              <td class="px-6 py-5">
                <div class="flex flex-col">
                  <span class="text-sm font-medium text-ink group-hover:text-action-blue transition-colors">
                    {job.title}
                  </span>
                  {#if job.description}
                    <span class="text-xs text-muted truncate max-w-xs mt-0.5">{job.description}</span>
                  {/if}
                </div>
              </td>
              <td class="px-6 py-5">
                <Badge variant={job.status as any} text={job.status} />
              </td>
              <td class="px-6 py-5">
                <Badge variant={`priority-${job.priority}` as any} text={job.priority} />
              </td>
              <td class="px-6 py-5">
                <span class="text-xs text-slate">
                  {new Date(job.deadline).toLocaleDateString('en-IN', { day: '2-digit', month: 'short', year: 'numeric' })}
                </span>
              </td>
              <td class="px-6 py-5 text-right">
                <button class="text-muted hover:text-ink transition-colors">
                  <DotsThreeOutlineVertical weight="fill" size={18} />
                </button>
              </td>
            </tr>
          {:else}
            <tr>
              <td colspan="6" class="p-0">
                <EmptyState 
                  title="No jobs found" 
                  description={searchQuery ? `No results for "${searchQuery}". Try a different term or filter.` : "Start by creating your first field operation job using the wizard."}
                  icon={Briefcase}
                  class="border-0 rounded-none bg-transparent py-20"
                />
              </td>
            </tr>
          {/each}
        {/if}
      </tbody>
    </table>
  </div>
</div>
