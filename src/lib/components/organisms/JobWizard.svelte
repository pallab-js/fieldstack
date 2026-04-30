<script lang="ts">
  import { onMount } from "svelte";
  import { api } from "../../utils/invoke";
  import { jobsStore } from "../../stores/jobs.svelte";
  import { uiStore } from "../../stores/ui.svelte";
  import Button from "../primitives/Button.svelte";
  import { X, ArrowRight, ArrowLeft, CheckCircle } from "phosphor-svelte";
  import { clsx, type ClassValue } from "clsx";
  import { twMerge } from "tailwind-merge";
  import type { Company, Person } from "../../types";

  function cn(...inputs: ClassValue[]) {
    return twMerge(clsx(inputs));
  }

  interface Props {
    onClose: () => void;
  }

  let { onClose }: Props = $props();

  let step = $state(1);
  let isSaving = $state(false);
  let companies = $state<Company[]>([]);
  let people = $state<Person[]>([]);

  // Wizard State
  let formData = $state({
    title: "",
    description: "",
    priority: "medium" as "low" | "medium" | "high" | "critical",
    company_id: "",
    assigned_person_id: "",
    deadline: new Date(Date.now() + 86400000).toISOString().split('T')[0],
  });

  onMount(async () => {
    // Load companies and people from DB
    [companies, people] = await Promise.all([api.companies.list(), api.people.list()]);
    if (companies.length > 0) formData.company_id = companies[0].id;
    if (people.length > 0) formData.assigned_person_id = people[0].id;

    // Try to recover draft
    const draft = await api.drafts.get("main-wizard");
    if (draft) {
      try {
        const parsed = JSON.parse(draft);
        formData = { ...formData, ...parsed.data };
        step = parsed.step || 1;
      } catch (e) {
        console.error("Failed to parse draft", e);
      }
    }
  });

  // Auto-save draft on changes
  $effect(() => {
    const payload = JSON.stringify({ step, data: $state.snapshot(formData) });
    api.drafts.save("main-wizard", payload);
  });

  async function handleCreate() {
    isSaving = true;
    try {
      await api.jobs.create({
        ...formData,
        deadline: new Date(formData.deadline).toISOString()
      });
      await jobsStore.fetchJobs();
      await api.drafts.delete("main-wizard");
      uiStore.notify("Job created successfully", "success");
      onClose();
    } catch (e) {
      uiStore.notify(e as string, "error");
    } finally {
      isSaving = false;
    }
  }

  const steps = [
    { n: 1, label: "Entity" },
    { n: 2, label: "Details" },
    { n: 3, label: "Priority" },
    { n: 4, label: "Timeline" },
    { n: 5, label: "Review" },
  ];
</script>

<div class="fixed inset-0 bg-primary/20 backdrop-blur-sm z-[200] flex items-center justify-center p-6">
  <div class="bg-canvas w-full max-w-2xl rounded-lg border border-hairline overflow-hidden flex flex-col max-h-[90vh]">
    <!-- Header -->
    <header class="p-6 border-b border-hairline flex items-center justify-between bg-soft-stone/30">
      <div>
        <h2 class="font-display text-xl">Create New Job</h2>
        <p class="text-slate text-xs mt-1">Step {step} of 5: {steps[step-1].label}</p>
      </div>
      <button onclick={onClose} class="text-muted hover:text-ink transition-colors">
        <X size={20} />
      </button>
    </header>

    <!-- Progress Bar -->
    <div class="h-1 bg-hairline flex">
      {#each steps as s}
        <div 
          class={cn(
            "h-full transition-all duration-500",
            s.n <= step ? "bg-coral" : "bg-transparent"
          )} 
          style="width: 20%"
        ></div>
      {/each}
    </div>

    <!-- Step Content -->
    <div class="flex-1 overflow-y-auto p-12">
      {#if step === 1}
        <div class="space-y-8 animate-in fade-in slide-in-from-bottom-4">
          <h3 class="text-2xl font-display">Select entity and assignee</h3>
          <div class="space-y-4">
            <label class="block">
              <span class="text-[10px] font-mono uppercase tracking-widest text-muted block mb-2">Company</span>
              <select bind:value={formData.company_id} class="w-full bg-canvas border border-hairline rounded-sm px-4 py-3 focus:border-form-focus outline-none transition-colors">
                {#each companies as c}
                  <option value={c.id}>{c.name}</option>
                {/each}
              </select>
            </label>
            <label class="block">
              <span class="text-[10px] font-mono uppercase tracking-widest text-muted block mb-2">Assignee</span>
              <select bind:value={formData.assigned_person_id} class="w-full bg-canvas border border-hairline rounded-sm px-4 py-3 focus:border-form-focus outline-none transition-colors">
                {#each people as p}
                  <option value={p.id}>{p.name}</option>
                {/each}
              </select>
            </label>
          </div>
        </div>
      {:else if step === 2}
        <div class="space-y-8 animate-in fade-in slide-in-from-bottom-4">
          <h3 class="text-2xl font-display">Describe the operation</h3>
          <div class="space-y-4">
            <label class="block">
              <span class="text-[10px] font-mono uppercase tracking-widest text-muted block mb-2">Job Title</span>
              <input 
                type="text" 
                bind:value={formData.title} 
                placeholder="e.g. Site Audit - Block B"
                class="w-full bg-canvas border border-hairline rounded-sm px-4 py-3 focus:border-form-focus outline-none transition-colors"
              />
            </label>
            <label class="block">
              <span class="text-[10px] font-mono uppercase tracking-widest text-muted block mb-2">Description</span>
              <textarea 
                bind:value={formData.description} 
                rows="4"
                placeholder="Details of the work required..."
                class="w-full bg-canvas border border-hairline rounded-sm px-4 py-3 focus:border-form-focus outline-none transition-colors resize-none"
              ></textarea>
            </label>
          </div>
        </div>
      {:else if step === 3}
        <div class="space-y-8 animate-in fade-in slide-in-from-bottom-4">
          <h3 class="text-2xl font-display">Set operational priority</h3>
          <div class="grid grid-cols-2 gap-4">
            {#each ['low', 'medium', 'high', 'critical'] as p}
              <button 
                onclick={() => formData.priority = p as any}
                class={cn(
                  "p-6 border text-left rounded-sm transition-all",
                  formData.priority === p 
                    ? "border-primary bg-primary text-on-primary" 
                    : "border-hairline hover:border-muted"
                )}
              >
                <span class="block text-[10px] font-mono uppercase tracking-widest opacity-70 mb-1">{p}</span>
                <span class="text-sm font-medium">
                  {p === 'critical' ? 'Requires immediate action' : 
                   p === 'high' ? 'High business impact' : 
                   p === 'medium' ? 'Standard operational flow' : 'Routine task'}
                </span>
              </button>
            {/each}
          </div>
        </div>
      {:else if step === 4}
        <div class="space-y-8 animate-in fade-in slide-in-from-bottom-4">
          <h3 class="text-2xl font-display">Define deadline</h3>
          <div class="space-y-4">
            <label class="block">
              <span class="text-[10px] font-mono uppercase tracking-widest text-muted block mb-2">Target Date</span>
              <input 
                type="date" 
                bind:value={formData.deadline} 
                class="w-full bg-canvas border border-hairline rounded-sm px-4 py-3 focus:border-form-focus outline-none transition-colors"
              />
            </label>
            <p class="text-slate text-xs italic">The overdue engine will trigger a system alert exactly at 11:59 PM on this date.</p>
          </div>
        </div>
      {:else if step === 5}
        <div class="space-y-8 animate-in fade-in slide-in-from-bottom-4">
          <h3 class="text-2xl font-display">Review & Commit</h3>
          <div class="bg-soft-stone/20 rounded-sm border border-hairline p-6 space-y-4">
            <div class="flex justify-between border-b border-hairline pb-2">
              <span class="text-muted text-xs">Title</span>
              <span class="text-ink text-sm font-medium">{formData.title || 'Untitled Job'}</span>
            </div>
            <div class="flex justify-between border-b border-hairline pb-2">
              <span class="text-muted text-xs">Priority</span>
              <span class={cn("text-xs font-mono uppercase", formData.priority === 'critical' ? 'text-error' : 'text-ink')}>
                {formData.priority}
              </span>
            </div>
            <div class="flex justify-between border-b border-hairline pb-2">
              <span class="text-muted text-xs">Deadline</span>
              <span class="text-ink text-sm font-medium">{formData.deadline}</span>
            </div>
            <div class="flex justify-between">
              <span class="text-muted text-xs">Assignee</span>
              <span class="text-ink text-sm font-medium">{people.find(p => p.id === formData.assigned_person_id)?.name ?? formData.assigned_person_id}</span>
            </div>
          </div>
          <div class="flex items-center gap-3 text-action-blue bg-action-blue/5 p-4 rounded-sm border border-action-blue/10">
            <CheckCircle size={20} />
            <p class="text-xs">Once created, this job will be immutable for the next 60 seconds while the audit trail initializes.</p>
          </div>
        </div>
      {/if}
    </div>

    <!-- Footer Actions -->
    <footer class="p-6 border-t border-hairline flex items-center justify-between bg-canvas">
      <Button 
        variant="ghost" 
        size="sm" 
        onclick={() => step > 1 ? step-- : onClose()}
        disabled={isSaving}
      >
        {step === 1 ? 'Cancel' : 'Back'}
      </Button>

      <div class="flex items-center gap-3">
        {#if step < 5}
          <Button 
            variant="primary" 
            onclick={() => step++}
            disabled={step === 2 && !formData.title}
          >
            Continue <ArrowRight size={16} class="ml-2" />
          </Button>
        {:else}
          <Button 
            variant="coral" 
            onclick={handleCreate}
            disabled={isSaving}
          >
            {isSaving ? 'Creating...' : 'Create Job'}
          </Button>
        {/if}
      </div>
    </footer>
  </div>
</div>
