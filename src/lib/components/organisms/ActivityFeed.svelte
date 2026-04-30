<script lang="ts">
  import { jobsStore } from "$lib/stores/jobs.svelte";
  import Badge from "$lib/components/primitives/Badge.svelte";
  import Skeleton from "$lib/components/primitives/Skeleton.svelte";
  import { Briefcase, ArrowRight } from "phosphor-svelte";

  interface Props {
    limit?: number;
    onJobClick: (id: string) => void;
  }

  let { limit = 8, onJobClick }: Props = $props();

  // Most recently updated jobs
  const recentJobs = $derived(
    [...jobsStore.jobs]
      .sort((a, b) => new Date(b.updated_at).getTime() - new Date(a.updated_at).getTime())
      .slice(0, limit)
  );

  function relativeTime(dateStr: string): string {
    const diff = Date.now() - new Date(dateStr).getTime();
    const mins = Math.floor(diff / 60000);
    if (mins < 60) return `${mins}m ago`;
    const hrs = Math.floor(mins / 60);
    if (hrs < 24) return `${hrs}h ago`;
    return `${Math.floor(hrs / 24)}d ago`;
  }
</script>

<div class="space-y-1">
  {#if jobsStore.isLoading}
    {#each Array(5) as _}
      <div class="flex items-center gap-4 px-4 py-3">
        <Skeleton class="h-8 w-8 rounded-full shrink-0" />
        <div class="flex-1 space-y-1.5">
          <Skeleton class="h-3.5 w-48" />
          <Skeleton class="h-3 w-24" />
        </div>
        <Skeleton class="h-5 w-16 rounded-full" />
      </div>
    {/each}
  {:else if recentJobs.length === 0}
    <div class="flex flex-col items-center justify-center py-16 text-center">
      <div class="w-10 h-10 rounded-full bg-soft-stone flex items-center justify-center text-muted mb-3">
        <Briefcase size={20} />
      </div>
      <p class="text-sm text-muted">No jobs yet. Create one to get started.</p>
    </div>
  {:else}
    {#each recentJobs as job (job.id)}
      <button
        onclick={() => onJobClick(job.id)}
        class="w-full flex items-center gap-4 px-4 py-3 rounded-sm hover:bg-soft-stone/30 transition-colors text-left group"
      >
        <!-- Status dot -->
        <div class="shrink-0 w-8 h-8 rounded-full flex items-center justify-center text-[10px] font-mono font-bold
          {job.status === 'overdue' ? 'bg-error/10 text-error' :
           job.status === 'active' ? 'bg-pale-green text-deep-green' :
           job.status === 'completed' ? 'bg-action-blue/10 text-action-blue' :
           job.status === 'disputed' ? 'bg-coral/10 text-coral' :
           'bg-soft-stone text-muted'}">
          {job.id.replace('JOB-', '')}
        </div>

        <div class="flex-1 min-w-0">
          <p class="text-sm font-medium text-ink truncate group-hover:text-action-blue transition-colors">
            {job.title}
          </p>
          <p class="text-[11px] text-muted mt-0.5">
            {relativeTime(job.updated_at)}
          </p>
        </div>

        <div class="flex items-center gap-3 shrink-0">
          <Badge variant={job.status as any} text={job.status} />
          <ArrowRight size={14} class="text-muted opacity-0 group-hover:opacity-100 transition-opacity" />
        </div>
      </button>
    {/each}
  {/if}
</div>
