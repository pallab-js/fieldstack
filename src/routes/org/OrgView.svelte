<script lang="ts">
  import { onMount } from "svelte";
  import { api } from "$lib/utils/invoke";
  import { uiStore } from "$lib/stores/ui.svelte";
  import type { Company, Person } from "$lib/types";
  import { Plus, Pencil, Trash, Buildings, User, X, Check } from "phosphor-svelte";

  let companies = $state<Company[]>([]);
  let people = $state<Person[]>([]);
  let personCompanies = $state<Record<string, string[]>>({});
  let activeTab = $state<"companies" | "people">("companies");
  let isLoading = $state(true);

  // Modal state
  let modal = $state<{
    type: "company" | "person";
    mode: "create" | "edit";
    id?: string;
    name: string;
    email: string;
    phone: string;
    logoUrl: string;
    selectedCompanyIds: string[];
  } | null>(null);

  let deleteConfirm = $state<{ type: "company" | "person"; id: string; name: string } | null>(null);

  async function load() {
    isLoading = true;
    try {
      [companies, people] = await Promise.all([api.companies.list(), api.people.list()]);
      // Load company memberships for each person
      const entries = await Promise.all(
        people.map(async (p) => [p.id, await api.people.getCompanies(p.id)] as [string, string[]])
      );
      personCompanies = Object.fromEntries(entries);
    } catch (e) {
      uiStore.notify(e as string, "error");
    } finally {
      isLoading = false;
    }
  }

  onMount(load);

  function openCreateCompany() {
    modal = { type: "company", mode: "create", name: "", email: "", phone: "", logoUrl: "", selectedCompanyIds: [] };
  }

  function openEditCompany(c: Company) {
    modal = { type: "company", mode: "edit", id: c.id, name: c.name, email: "", phone: "", logoUrl: c.logo_url ?? "", selectedCompanyIds: [] };
  }

  function openCreatePerson() {
    modal = { type: "person", mode: "create", name: "", email: "", phone: "", logoUrl: "", selectedCompanyIds: [] };
  }

  function openEditPerson(p: Person) {
    modal = { type: "person", mode: "edit", id: p.id, name: p.name, email: p.email ?? "", phone: p.phone ?? "", logoUrl: "", selectedCompanyIds: personCompanies[p.id] ?? [] };
  }

  async function handleSave() {
    if (!modal) return;
    try {
      if (modal.type === "company") {
        if (modal.mode === "create") {
          await api.companies.create(modal.name, modal.logoUrl || undefined);
          uiStore.notify("Company created", "success");
        } else {
          await api.companies.update(modal.id!, modal.name, modal.logoUrl || undefined);
          uiStore.notify("Company updated", "success");
        }
      } else {
        if (modal.mode === "create") {
          await api.people.create(modal.name, modal.email || undefined, modal.phone || undefined, modal.selectedCompanyIds);
          uiStore.notify("Person added", "success");
        } else {
          await api.people.update(modal.id!, modal.name, modal.email || undefined, modal.phone || undefined, modal.selectedCompanyIds);
          uiStore.notify("Person updated", "success");
        }
      }
      modal = null;
      await load();
    } catch (e) {
      uiStore.notify(e as string, "error");
    }
  }

  async function handleDelete() {
    if (!deleteConfirm) return;
    try {
      if (deleteConfirm.type === "company") {
        await api.companies.delete(deleteConfirm.id);
        uiStore.notify("Company deleted", "success");
      } else {
        await api.people.delete(deleteConfirm.id);
        uiStore.notify("Person removed", "success");
      }
      deleteConfirm = null;
      await load();
    } catch (e) {
      uiStore.notify(e as string, "error");
    }
  }

  function toggleCompany(id: string) {
    if (!modal) return;
    if (modal.selectedCompanyIds.includes(id)) {
      modal.selectedCompanyIds = modal.selectedCompanyIds.filter((c) => c !== id);
    } else {
      modal.selectedCompanyIds = [...modal.selectedCompanyIds, id];
    }
  }

  function getCompanyName(id: string) {
    return companies.find((c) => c.id === id)?.name ?? id;
  }

  function getPersonJobCount(personId: string) {
    return 0; // placeholder — could be derived from jobsStore if needed
  }
</script>

<div class="max-w-5xl mx-auto space-y-10">
  <!-- Header -->
  <section>
    <h1 class="font-display text-5xl font-normal tracking-tight mb-4">
      Organisation <span class="text-muted italic">Builder</span>
    </h1>
    <p class="text-slate text-lg max-w-2xl leading-relaxed">
      Manage your companies and field personnel. Assign people across entities and track team composition.
    </p>
  </section>

  <!-- Tab Bar -->
  <div class="flex items-center gap-1 border-b border-hairline">
    {#each [{ id: "companies", label: "Companies", count: companies.length }, { id: "people", label: "People", count: people.length }] as tab}
      <button
        onclick={() => (activeTab = tab.id as any)}
        class="px-6 py-3 text-sm font-medium border-b-2 transition-colors {activeTab === tab.id
          ? 'border-primary text-ink'
          : 'border-transparent text-muted hover:text-ink'}"
      >
        {tab.label}
        <span class="ml-2 text-[10px] font-mono bg-soft-stone px-1.5 py-0.5 rounded">{tab.count}</span>
      </button>
    {/each}
  </div>

  {#if isLoading}
    <div class="grid grid-cols-1 gap-4">
      {#each Array(3) as _}
        <div class="h-20 bg-soft-stone animate-pulse rounded-sm"></div>
      {/each}
    </div>

  {:else if activeTab === "companies"}
    <!-- Companies Panel -->
    <div class="space-y-4">
      <div class="flex justify-end">
        <button
          onclick={openCreateCompany}
          class="flex items-center gap-2 bg-primary text-on-primary px-4 py-2 rounded-pill text-sm font-medium hover:bg-primary/90 transition-colors"
        >
          <Plus size={16} weight="bold" /> Add Company
        </button>
      </div>

      {#if companies.length === 0}
        <div class="border-2 border-dashed border-hairline rounded-lg h-48 flex flex-col items-center justify-center text-center">
          <Buildings size={32} class="text-muted mb-3" />
          <p class="text-slate text-sm">No companies yet. Add your first entity.</p>
        </div>
      {:else}
        <div class="space-y-3">
          {#each companies as company}
            {@const memberCount = people.filter((p) => (personCompanies[p.id] ?? []).includes(company.id)).length}
            <div class="bg-canvas border border-hairline rounded-sm p-5 flex items-center justify-between group hover:border-muted transition-colors">
              <div class="flex items-center gap-4">
                <div class="w-10 h-10 rounded-sm bg-soft-stone flex items-center justify-center text-sm font-bold text-slate">
                  {company.name.charAt(0)}
                </div>
                <div>
                  <h3 class="font-medium text-ink">{company.name}</h3>
                  <p class="text-[11px] text-muted font-mono mt-0.5">{memberCount} member{memberCount !== 1 ? "s" : ""}</p>
                </div>
              </div>
              <div class="flex items-center gap-2 opacity-0 group-hover:opacity-100 transition-opacity">
                <button
                  onclick={() => openEditCompany(company)}
                  class="p-2 text-muted hover:text-ink hover:bg-soft-stone rounded-sm transition-colors"
                >
                  <Pencil size={16} />
                </button>
                <button
                  onclick={() => (deleteConfirm = { type: "company", id: company.id, name: company.name })}
                  class="p-2 text-muted hover:text-error hover:bg-error/5 rounded-sm transition-colors"
                >
                  <Trash size={16} />
                </button>
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </div>

  {:else}
    <!-- People Panel -->
    <div class="space-y-4">
      <div class="flex justify-end">
        <button
          onclick={openCreatePerson}
          class="flex items-center gap-2 bg-primary text-on-primary px-4 py-2 rounded-pill text-sm font-medium hover:bg-primary/90 transition-colors"
        >
          <Plus size={16} weight="bold" /> Add Person
        </button>
      </div>

      {#if people.length === 0}
        <div class="border-2 border-dashed border-hairline rounded-lg h-48 flex flex-col items-center justify-center text-center">
          <User size={32} class="text-muted mb-3" />
          <p class="text-slate text-sm">No people yet. Add your first team member.</p>
        </div>
      {:else}
        <div class="space-y-3">
          {#each people as person}
            {@const memberOf = (personCompanies[person.id] ?? []).map(getCompanyName)}
            <div class="bg-canvas border border-hairline rounded-sm p-5 flex items-center justify-between group hover:border-muted transition-colors">
              <div class="flex items-center gap-4">
                <div
                  class="w-10 h-10 rounded-full flex items-center justify-center text-sm font-bold text-white shrink-0"
                  style="background-color: {person.avatar_color ?? '#5e6ad2'}"
                >
                  {person.initials}
                </div>
                <div>
                  <h3 class="font-medium text-ink">{person.name}</h3>
                  <div class="flex items-center gap-3 mt-0.5">
                    {#if person.email}
                      <span class="text-[11px] text-muted">{person.email}</span>
                    {/if}
                    {#if memberOf.length > 0}
                      <span class="text-[10px] font-mono text-slate">· {memberOf.join(", ")}</span>
                    {/if}
                  </div>
                </div>
              </div>
              <div class="flex items-center gap-2 opacity-0 group-hover:opacity-100 transition-opacity">
                <button
                  onclick={() => openEditPerson(person)}
                  class="p-2 text-muted hover:text-ink hover:bg-soft-stone rounded-sm transition-colors"
                >
                  <Pencil size={16} />
                </button>
                <button
                  onclick={() => (deleteConfirm = { type: "person", id: person.id, name: person.name })}
                  class="p-2 text-muted hover:text-error hover:bg-error/5 rounded-sm transition-colors"
                >
                  <Trash size={16} />
                </button>
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </div>
  {/if}
</div>

<!-- Create/Edit Modal -->
{#if modal}
  <div class="fixed inset-0 bg-primary/20 backdrop-blur-sm z-[200] flex items-center justify-center p-6">
    <div class="bg-canvas w-full max-w-md rounded-lg border border-hairline overflow-hidden">
      <header class="p-6 border-b border-hairline flex items-center justify-between bg-soft-stone/30">
        <h2 class="font-display text-lg">
          {modal.mode === "create" ? "Add" : "Edit"}
          {modal.type === "company" ? "Company" : "Person"}
        </h2>
        <button onclick={() => (modal = null)} class="text-muted hover:text-ink transition-colors">
          <X size={20} />
        </button>
      </header>

      <div class="p-6 space-y-4">
        <label class="block">
          <span class="text-[10px] font-mono uppercase tracking-widest text-muted block mb-2">Name *</span>
          <input
            type="text"
            bind:value={modal.name}
            class="w-full bg-canvas border border-hairline rounded-sm px-4 py-3 focus:border-form-focus outline-none transition-colors"
            placeholder={modal.type === "company" ? "e.g. Fieldstack Fertilizers" : "e.g. Ravi Kumar"}
          />
        </label>

        {#if modal.type === "company"}
          <label class="block">
            <span class="text-[10px] font-mono uppercase tracking-widest text-muted block mb-2">Logo URL</span>
            <input
              type="text"
              bind:value={modal.logoUrl}
              class="w-full bg-canvas border border-hairline rounded-sm px-4 py-3 focus:border-form-focus outline-none transition-colors"
              placeholder="https://..."
            />
          </label>
        {:else}
          <label class="block">
            <span class="text-[10px] font-mono uppercase tracking-widest text-muted block mb-2">Email</span>
            <input
              type="email"
              bind:value={modal.email}
              class="w-full bg-canvas border border-hairline rounded-sm px-4 py-3 focus:border-form-focus outline-none transition-colors"
              placeholder="name@fieldstack.in"
            />
          </label>
          <label class="block">
            <span class="text-[10px] font-mono uppercase tracking-widest text-muted block mb-2">Phone</span>
            <input
              type="tel"
              bind:value={modal.phone}
              class="w-full bg-canvas border border-hairline rounded-sm px-4 py-3 focus:border-form-focus outline-none transition-colors"
              placeholder="+91 98765 43210"
            />
          </label>
          <div>
            <span class="text-[10px] font-mono uppercase tracking-widest text-muted block mb-2">Companies</span>
            <div class="space-y-2">
              {#each companies as c}
                <button
                  type="button"
                  onclick={() => toggleCompany(c.id)}
                  class="w-full flex items-center justify-between px-4 py-2.5 border rounded-sm transition-colors {modal.selectedCompanyIds.includes(c.id)
                    ? 'border-primary bg-primary/5 text-ink'
                    : 'border-hairline text-muted hover:border-muted'}"
                >
                  <span class="text-sm">{c.name}</span>
                  {#if modal.selectedCompanyIds.includes(c.id)}
                    <Check size={16} class="text-primary" />
                  {/if}
                </button>
              {/each}
            </div>
          </div>
        {/if}
      </div>

      <footer class="p-6 border-t border-hairline flex items-center justify-end gap-3">
        <button
          onclick={() => (modal = null)}
          class="px-4 py-2 text-sm text-muted hover:text-ink transition-colors"
        >
          Cancel
        </button>
        <button
          onclick={handleSave}
          disabled={!modal.name.trim()}
          class="px-5 py-2 bg-primary text-on-primary text-sm font-medium rounded-sm hover:bg-primary/90 transition-colors disabled:opacity-40 disabled:cursor-not-allowed"
        >
          {modal.mode === "create" ? "Create" : "Save Changes"}
        </button>
      </footer>
    </div>
  </div>
{/if}

<!-- Delete Confirm Modal -->
{#if deleteConfirm}
  <div class="fixed inset-0 bg-primary/20 backdrop-blur-sm z-[200] flex items-center justify-center p-6">
    <div class="bg-canvas w-full max-w-sm rounded-lg border border-hairline overflow-hidden">
      <div class="p-6 space-y-3">
        <h2 class="font-display text-lg">Delete {deleteConfirm.type === "company" ? "Company" : "Person"}?</h2>
        <p class="text-slate text-sm">
          <strong class="text-ink">{deleteConfirm.name}</strong> will be permanently removed.
          {#if deleteConfirm.type === "company"}
            All associated jobs and team links will also be deleted.
          {/if}
        </p>
      </div>
      <footer class="p-6 border-t border-hairline flex items-center justify-end gap-3">
        <button
          onclick={() => (deleteConfirm = null)}
          class="px-4 py-2 text-sm text-muted hover:text-ink transition-colors"
        >
          Cancel
        </button>
        <button
          onclick={handleDelete}
          class="px-5 py-2 bg-error text-white text-sm font-medium rounded-sm hover:bg-error/90 transition-colors"
        >
          Delete
        </button>
      </footer>
    </div>
  </div>
{/if}
