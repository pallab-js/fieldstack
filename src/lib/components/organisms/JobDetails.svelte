<script lang="ts">
  import { onMount } from "svelte";
  import { api } from "../../utils/invoke";
  import { jobsStore } from "../../stores/jobs.svelte";
  import { uiStore } from "../../stores/ui.svelte";
  import Badge from "../primitives/Badge.svelte";
  import Button from "../primitives/Button.svelte";
  import { 
    X, Calendar, User, FileImage, FileVideo, FileAudio, FileText,
    Warning, CheckCircle, Eye, ArrowCounterClockwise
  } from "phosphor-svelte";
  import { convertFileSrc } from "@tauri-apps/api/core";
  import { openPath } from "@tauri-apps/plugin-opener";

  interface Props {
    jobId: string;
    onClose: () => void;
  }

  let { jobId, onClose }: Props = $props();
  let details = $state<{job: any, proofs: any[], audit_log: any[]} | null>(null);
  let isLoading = $state(true);
  let disputeReason = $state("");
  let showDisputeInput = $state(false);

  onMount(async () => {
    try {
      details = await api.jobs.getDetails(jobId);
    } catch (e) {
      uiStore.notify(e as string, "error");
    } finally {
      isLoading = false;
    }
  });

  async function refresh() {
    details = await api.jobs.getDetails(jobId);
    await jobsStore.fetchJobs();
  }

  async function markCompleted() {
    try {
      await api.jobs.updateStatus(jobId, "completed");
      uiStore.notify("Job marked as completed", "success");
      await refresh();
    } catch (e) { uiStore.notify(e as string, "error"); }
  }

  async function submitDispute() {
    if (!disputeReason.trim()) return;
    try {
      await api.jobs.dispute(jobId, disputeReason);
      uiStore.notify("Job disputed", "success");
      disputeReason = "";
      showDisputeInput = false;
      await refresh();
    } catch (e) { uiStore.notify(e as string, "error"); }
  }

  async function resolveDispute() {
    try {
      await api.jobs.resolve(jobId);
      uiStore.notify("Dispute resolved", "success");
      await refresh();
    } catch (e) { uiStore.notify(e as string, "error"); }
  }

  async function openProof(filePath: string) {
    try {
      await openPath(filePath);
    } catch (e) {
      uiStore.notify("Could not open file", "error");
    }
  }

  const getIcon = (type: string) => {
    switch(type) {
      case 'photo': return FileImage;
      case 'video': return FileVideo;
      case 'audio': return FileAudio;
      default: return FileText;
    }
  };
</script>

<div class="fixed inset-0 bg-primary/20 backdrop-blur-sm z-[200] flex items-center justify-end">
  <div class="bg-canvas w-full max-w-3xl h-full border-l border-hairline flex flex-col animate-in slide-in-from-right duration-300">
    <!-- Header -->
    <header class="p-8 border-b border-hairline flex items-center justify-between bg-soft-stone/10">
      <div class="flex items-center gap-4">
        <span class="text-xs font-mono font-medium text-slate">{jobId}</span>
        <Badge variant={details?.job.status} text={details?.job.status || 'Loading...'} />
      </div>
      <button onclick={onClose} class="text-muted hover:text-ink transition-colors">
        <X size={24} />
      </button>
    </header>

    {#if isLoading}
      <div class="flex-1 flex items-center justify-center">
        <div class="animate-spin w-8 h-8 border-2 border-hairline border-t-primary rounded-full"></div>
      </div>
    {:else if details}
      <div class="flex-1 overflow-y-auto">
        <!-- Job Hero -->
        <section class="p-12 space-y-6">
          <h1 class="text-4xl font-display tracking-tight">{details.job.title}</h1>
          <p class="text-slate text-lg leading-relaxed">{details.job.description || 'No description provided.'}</p>
          
          <div class="grid grid-cols-2 gap-6 pt-6">
            <div class="flex items-center gap-3">
              <div class="w-10 h-10 rounded-sm bg-soft-stone flex items-center justify-center text-muted">
                <User size={20} />
              </div>
              <div class="flex flex-col">
                <span class="text-[10px] font-mono uppercase tracking-widest text-muted">Assignee</span>
                <span class="text-sm font-medium">{details.job.assigned_person_id}</span>
              </div>
            </div>
            <div class="flex items-center gap-3">
              <div class="w-10 h-10 rounded-sm bg-soft-stone flex items-center justify-center text-muted">
                <Calendar size={20} />
              </div>
              <div class="flex flex-col">
                <span class="text-[10px] font-mono uppercase tracking-widest text-muted">Deadline</span>
                <span class="text-sm font-medium">{new Date(details.job.deadline).toLocaleDateString()}</span>
              </div>
            </div>
          </div>

          <!-- Dispute reason display -->
          {#if details.job.status === 'disputed'}
            {@const disputedProof = details.proofs.find((p: any) => p.dispute_reason)}
            {#if disputedProof?.dispute_reason}
              <div class="bg-coral/5 border border-coral/20 rounded-sm p-4">
                <p class="text-[10px] font-mono uppercase tracking-widest text-coral mb-1">Dispute Reason</p>
                <p class="text-sm text-ink">{disputedProof.dispute_reason}</p>
              </div>
            {/if}
          {/if}
        </section>

        <!-- Proof Viewer -->
        <section class="p-12 bg-soft-stone/10 space-y-8">
          <div class="flex items-center justify-between border-b border-hairline pb-4">
            <h3 class="font-display text-2xl">Proof Evidence</h3>
            <span class="text-[10px] font-mono uppercase tracking-widest text-muted">{details.proofs.length} Files</span>
          </div>

          {#if details.proofs.length === 0}
            <div class="py-12 text-center border-2 border-dashed border-hairline rounded-lg">
              <p class="text-slate text-sm">No proof files have been submitted for this job yet.</p>
            </div>
          {:else}
            <div class="grid grid-cols-2 gap-6">
              {#each details.proofs as proof}
                <div class="bg-canvas border border-hairline rounded-lg overflow-hidden group">
                  <div class="aspect-video bg-soft-stone relative flex items-center justify-center overflow-hidden">
                    {#if proof.file_type === 'photo'}
                      <img src={convertFileSrc(proof.file_path)} alt="Proof" class="w-full h-full object-cover transition-transform group-hover:scale-105" />
                    {:else}
                      {@const Icon = getIcon(proof.file_type)}
                      <Icon size={48} class="text-muted" />
                    {/if}
                    <div class="absolute inset-0 bg-primary/40 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center">
                      <button
                        onclick={() => openProof(proof.file_path)}
                        class="flex items-center gap-2 bg-canvas text-ink text-xs font-medium px-3 py-2 rounded-sm hover:bg-soft-stone transition-colors"
                      >
                        <Eye size={14} /> Open File
                      </button>
                    </div>
                  </div>
                  <div class="p-4 flex items-center justify-between">
                    <div class="flex flex-col">
                      <span class="text-[10px] font-mono uppercase tracking-widest text-muted">{proof.file_type}</span>
                      <span class="text-[11px] text-slate">{new Date(proof.submitted_at).toLocaleString()}</span>
                    </div>
                    {#if proof.is_locked}
                      <div class="text-deep-green bg-pale-green px-2 py-1 rounded text-[10px] font-bold">LOCKED</div>
                    {/if}
                  </div>
                </div>
              {/each}
            </div>
          {/if}
        </section>

        <!-- Audit Trail -->
        <section class="p-12 space-y-8">
          <h3 class="font-display text-2xl border-b border-hairline pb-4">Audit Timeline</h3>
          <div class="space-y-8 relative">
            <div class="absolute left-4 top-0 bottom-0 w-px bg-hairline"></div>
            {#each details.audit_log as log}
              <div class="relative pl-12">
                <div class="absolute left-2.5 top-1.5 w-3 h-3 rounded-full bg-canvas border-2 border-primary z-10"></div>
                <div class="flex flex-col">
                  <div class="flex items-center gap-3 mb-1">
                    <span class="text-[10px] font-mono uppercase tracking-widest text-muted">{log.event_type}</span>
                    <span class="text-[10px] text-slate">{new Date(log.timestamp).toLocaleString()}</span>
                  </div>
                  <p class="text-sm text-ink">{log.description}</p>
                  <span class="text-[11px] text-muted mt-1">by {log.actor}</span>
                </div>
              </div>
            {/each}
          </div>
        </section>
      </div>

      <!-- Footer Actions -->
      <footer class="p-8 border-t border-hairline bg-canvas space-y-3">
        {#if showDisputeInput}
          <div class="flex items-center gap-3">
            <input
              type="text"
              bind:value={disputeReason}
              placeholder="Reason for dispute..."
              class="flex-1 bg-canvas border border-hairline rounded-sm px-4 py-2.5 text-sm focus:border-form-focus outline-none"
              onkeydown={(e) => e.key === 'Enter' && submitDispute()}
            />
            <Button variant="coral" size="sm" onclick={submitDispute} disabled={!disputeReason.trim()}>
              Submit
            </Button>
            <button onclick={() => showDisputeInput = false} class="text-muted hover:text-ink transition-colors">
              <X size={18} />
            </button>
          </div>
        {:else}
          <div class="flex items-center gap-3">
            {#if details.job.status === 'completed' || details.job.status === 'resolved'}
              <p class="text-deep-green font-medium flex items-center gap-2 text-sm">
                <CheckCircle size={20} weight="fill" /> This job is finalized and locked.
              </p>
            {:else if details.job.status === 'disputed'}
              <Button variant="primary" onclick={resolveDispute} class="gap-2">
                <ArrowCounterClockwise size={18} /> Resolve Dispute
              </Button>
            {:else}
              <Button variant="primary" onclick={markCompleted} class="gap-2">
                <CheckCircle size={18} /> Mark Completed
              </Button>
              <Button variant="outline" onclick={() => showDisputeInput = true} class="gap-2 text-coral border-coral/20 hover:bg-coral/5">
                <Warning size={18} /> Dispute Job
              </Button>
            {/if}
          </div>
        {/if}
      </footer>
    {/if}
  </div>
</div>
