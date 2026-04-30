<script lang="ts">
  import { onMount } from "svelte";
  import { api, type ReportSummary, type JobsByCompany, type JobsByPerson, type StatusOverTime } from "$lib/utils/invoke";
  import { ChartBar, Buildings, User, CheckCircle, Warning, Clock, ArrowClockwise } from "phosphor-svelte";

  let summary = $state<ReportSummary | null>(null);
  let byCompany = $state<JobsByCompany[]>([]);
  let byPerson = $state<JobsByPerson[]>([]);
  let overTime = $state<StatusOverTime[]>([]);
  let loading = $state(true);
  let error = $state<string | null>(null);

  async function load() {
    loading = true;
    error = null;
    try {
      [summary, byCompany, byPerson, overTime] = await Promise.all([
        api.reports.summary(),
        api.reports.byCompany(),
        api.reports.byPerson(),
        api.reports.overTime(),
      ]);
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  onMount(load);

  // SVG bar chart helpers
  const CHART_W = 480;
  const CHART_H = 120;
  const BAR_GAP = 8;

  function barChart(data: StatusOverTime[]) {
    if (!data.length) return { bars: [], labels: [] };
    const maxVal = Math.max(...data.map(d => d.created), 1);
    const barW = (CHART_W - BAR_GAP * (data.length - 1)) / data.length;
    const bars = data.map((d, i) => {
      const x = i * (barW + BAR_GAP);
      const completedH = (d.completed / maxVal) * CHART_H;
      const overdueH = (d.overdue / maxVal) * CHART_H;
      const createdH = (d.created / maxVal) * CHART_H;
      return { x, barW, completedH, overdueH, createdH, d };
    });
    return { bars, maxVal };
  }

  const chart = $derived(barChart(overTime));

  function fmtMonth(m: string) {
    const [y, mo] = m.split("-");
    return new Date(Number(y), Number(mo) - 1).toLocaleString("en-IN", { month: "short", year: "2-digit" });
  }

  function pct(n: number, total: number) {
    return total > 0 ? Math.round((n / total) * 100) : 0;
  }
</script>

<div class="max-w-6xl mx-auto space-y-12">
  <!-- Header -->
  <section class="flex items-start justify-between">
    <div>
      <h1 class="font-display text-5xl font-normal tracking-tight mb-4">
        Reports <span class="text-muted italic">& Analytics</span>
      </h1>
      <p class="text-slate text-lg max-w-2xl leading-relaxed">
        Operational insights across jobs, teams, and companies.
      </p>
    </div>
    <button
      onclick={load}
      class="flex items-center gap-2 text-sm text-muted hover:text-ink border border-hairline px-4 py-2 rounded-sm hover:bg-soft-stone transition-colors"
    >
      <ArrowClockwise size={16} class={loading ? "animate-spin" : ""} />
      Refresh
    </button>
  </section>

  {#if error}
    <div class="bg-error/5 border border-error/20 text-error p-4 rounded-sm text-sm">{error}</div>
  {/if}

  {#if loading && !summary}
    <div class="grid grid-cols-2 lg:grid-cols-4 gap-6">
      {#each Array(4) as _}
        <div class="h-28 bg-soft-stone animate-pulse rounded-sm"></div>
      {/each}
    </div>
  {:else if summary}
    <!-- Summary Cards -->
    <div class="grid grid-cols-2 lg:grid-cols-4 gap-6">
      <div class="border border-hairline p-6 rounded-sm space-y-2">
        <div class="flex items-center gap-2 text-muted text-xs font-mono uppercase tracking-widest">
          <ChartBar size={14} /> Total Jobs
        </div>
        <div class="font-display text-4xl font-normal">{summary.total_jobs}</div>
        <div class="text-slate text-xs">{summary.completion_rate.toFixed(1)}% completion rate</div>
      </div>

      <div class="border border-hairline p-6 rounded-sm space-y-2">
        <div class="flex items-center gap-2 text-muted text-xs font-mono uppercase tracking-widest">
          <CheckCircle size={14} /> Completed
        </div>
        <div class="font-display text-4xl font-normal text-deep-green">{summary.completed}</div>
        <div class="text-slate text-xs">Avg {summary.avg_completion_days.toFixed(1)} days to complete</div>
      </div>

      <div class="border border-hairline p-6 rounded-sm space-y-2">
        <div class="flex items-center gap-2 text-muted text-xs font-mono uppercase tracking-widest">
          <Warning size={14} /> Overdue
        </div>
        <div class="font-display text-4xl font-normal text-error">{summary.overdue}</div>
        <div class="text-slate text-xs">{summary.disputed} disputed · {summary.resolved} resolved</div>
      </div>

      <div class="border border-hairline p-6 rounded-sm space-y-2">
        <div class="flex items-center gap-2 text-muted text-xs font-mono uppercase tracking-widest">
          <Clock size={14} /> Active
        </div>
        <div class="font-display text-4xl font-normal">{summary.active}</div>
        <div class="text-slate text-xs">{summary.pending} pending start</div>
      </div>
    </div>

    <!-- Status Distribution Bar -->
    <section class="border border-hairline rounded-sm p-6 space-y-4">
      <h3 class="font-display text-xl">Status Distribution</h3>
      <div class="flex h-3 rounded-full overflow-hidden gap-px">
        {#if summary.total_jobs > 0}
          {#each [
            { status: 'completed', count: summary.completed, color: 'bg-deep-green' },
            { status: 'active', count: summary.active, color: 'bg-action-blue' },
            { status: 'pending', count: summary.pending, color: 'bg-hairline' },
            { status: 'overdue', count: summary.overdue, color: 'bg-error' },
            { status: 'disputed', count: summary.disputed, color: 'bg-coral' },
            { status: 'resolved', count: summary.resolved, color: 'bg-slate' },
          ] as seg}
            {#if seg.count > 0}
              <div
                class="{seg.color} transition-all"
                style="width: {pct(seg.count, summary.total_jobs)}%"
                title="{seg.status}: {seg.count}"
              ></div>
            {/if}
          {/each}
        {:else}
          <div class="bg-hairline w-full"></div>
        {/if}
      </div>
      <div class="flex flex-wrap gap-4 text-xs text-slate">
        {#each [
          { label: 'Completed', count: summary.completed, dot: 'bg-deep-green' },
          { label: 'Active', count: summary.active, dot: 'bg-action-blue' },
          { label: 'Pending', count: summary.pending, dot: 'bg-hairline border border-slate/30' },
          { label: 'Overdue', count: summary.overdue, dot: 'bg-error' },
          { label: 'Disputed', count: summary.disputed, dot: 'bg-coral' },
          { label: 'Resolved', count: summary.resolved, dot: 'bg-slate' },
        ] as leg}
          <span class="flex items-center gap-1.5">
            <span class="w-2 h-2 rounded-full {leg.dot} inline-block"></span>
            {leg.label} ({leg.count})
          </span>
        {/each}
      </div>
    </section>

    <!-- Activity Over Time (SVG bar chart) -->
    {#if overTime.length > 0}
      <section class="border border-hairline rounded-sm p-6 space-y-4">
        <h3 class="font-display text-xl">Activity Over Time <span class="text-muted text-sm font-sans font-normal">(last 6 months)</span></h3>
        <div class="overflow-x-auto">
          <svg viewBox="0 0 {CHART_W} {CHART_H + 28}" class="w-full max-w-2xl" aria-label="Jobs over time bar chart">
            {#each chart.bars as bar}
              <!-- Created (background) -->
              <rect
                x={bar.x}
                y={CHART_H - bar.createdH}
                width={bar.barW}
                height={bar.createdH}
                fill="#eeece7"
                rx="2"
              />
              <!-- Completed (foreground) -->
              <rect
                x={bar.x}
                y={CHART_H - bar.completedH}
                width={bar.barW * 0.5}
                height={bar.completedH}
                fill="#003c33"
                rx="2"
              />
              <!-- Overdue -->
              <rect
                x={bar.x + bar.barW * 0.5 + 2}
                y={CHART_H - bar.overdueH}
                width={bar.barW * 0.45}
                height={bar.overdueH}
                fill="#b30000"
                rx="2"
              />
              <!-- Month label -->
              <text
                x={bar.x + bar.barW / 2}
                y={CHART_H + 18}
                text-anchor="middle"
                font-size="10"
                fill="#93939f"
                font-family="monospace"
              >{fmtMonth(bar.d.month)}</text>
            {/each}
          </svg>
        </div>
        <div class="flex gap-4 text-xs text-slate">
          <span class="flex items-center gap-1.5"><span class="w-3 h-2 rounded-sm bg-soft-stone border border-hairline inline-block"></span>Created</span>
          <span class="flex items-center gap-1.5"><span class="w-3 h-2 rounded-sm bg-deep-green inline-block"></span>Completed</span>
          <span class="flex items-center gap-1.5"><span class="w-3 h-2 rounded-sm bg-error inline-block"></span>Overdue</span>
        </div>
      </section>
    {/if}

    <!-- By Company Table -->
    {#if byCompany.length > 0}
      <section class="border border-hairline rounded-sm overflow-hidden">
        <div class="px-6 py-4 border-b border-hairline flex items-center gap-3">
          <Buildings size={18} class="text-muted" />
          <h3 class="font-display text-xl">Jobs by Company</h3>
        </div>
        <table class="w-full text-sm">
          <thead>
            <tr class="border-b border-hairline bg-soft-stone/40">
              <th class="text-left px-6 py-3 text-xs font-mono uppercase tracking-widest text-muted">Company</th>
              <th class="text-right px-4 py-3 text-xs font-mono uppercase tracking-widest text-muted">Total</th>
              <th class="text-right px-4 py-3 text-xs font-mono uppercase tracking-widest text-muted">Completed</th>
              <th class="text-right px-4 py-3 text-xs font-mono uppercase tracking-widest text-muted">Active</th>
              <th class="text-right px-6 py-3 text-xs font-mono uppercase tracking-widest text-muted">Overdue</th>
              <th class="px-6 py-3 text-xs font-mono uppercase tracking-widest text-muted">Progress</th>
            </tr>
          </thead>
          <tbody>
            {#each byCompany as row}
              <tr class="border-b border-hairline last:border-0 hover:bg-soft-stone/20 transition-colors">
                <td class="px-6 py-4 font-medium text-ink">{row.company_name}</td>
                <td class="px-4 py-4 text-right text-slate">{row.total}</td>
                <td class="px-4 py-4 text-right text-deep-green font-medium">{row.completed}</td>
                <td class="px-4 py-4 text-right {row.active > 0 ? 'text-action-blue' : 'text-slate'}">{row.active}</td>
                <td class="px-6 py-4 text-right {row.overdue > 0 ? 'text-error font-medium' : 'text-slate'}">{row.overdue}</td>
                <td class="px-6 py-4 w-32">
                  <div class="h-1.5 bg-hairline rounded-full overflow-hidden">
                    <div
                      class="h-full bg-deep-green rounded-full transition-all"
                      style="width: {pct(row.completed, row.total)}%"
                    ></div>
                  </div>
                  <span class="text-[10px] text-muted mt-1 block">{pct(row.completed, row.total)}%</span>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </section>
    {/if}

    <!-- By Person Table -->
    {#if byPerson.length > 0}
      <section class="border border-hairline rounded-sm overflow-hidden">
        <div class="px-6 py-4 border-b border-hairline flex items-center gap-3">
          <User size={18} class="text-muted" />
          <h3 class="font-display text-xl">Jobs by Team Member</h3>
        </div>
        <table class="w-full text-sm">
          <thead>
            <tr class="border-b border-hairline bg-soft-stone/40">
              <th class="text-left px-6 py-3 text-xs font-mono uppercase tracking-widest text-muted">Person</th>
              <th class="text-right px-4 py-3 text-xs font-mono uppercase tracking-widest text-muted">Total</th>
              <th class="text-right px-4 py-3 text-xs font-mono uppercase tracking-widest text-muted">Completed</th>
              <th class="text-right px-6 py-3 text-xs font-mono uppercase tracking-widest text-muted">Overdue</th>
              <th class="px-6 py-3 text-xs font-mono uppercase tracking-widest text-muted">Completion</th>
            </tr>
          </thead>
          <tbody>
            {#each byPerson as row}
              <tr class="border-b border-hairline last:border-0 hover:bg-soft-stone/20 transition-colors">
                <td class="px-6 py-4">
                  <div class="flex items-center gap-3">
                    <div
                      class="w-7 h-7 rounded-full flex items-center justify-center text-[10px] font-bold text-white shrink-0"
                      style="background-color: {row.avatar_color ?? '#93939f'}"
                    >{row.initials}</div>
                    <span class="font-medium text-ink">{row.person_name}</span>
                  </div>
                </td>
                <td class="px-4 py-4 text-right text-slate">{row.total}</td>
                <td class="px-4 py-4 text-right text-deep-green font-medium">{row.completed}</td>
                <td class="px-6 py-4 text-right {row.overdue > 0 ? 'text-error font-medium' : 'text-slate'}">{row.overdue}</td>
                <td class="px-6 py-4 w-32">
                  <div class="h-1.5 bg-hairline rounded-full overflow-hidden">
                    <div
                      class="h-full bg-deep-green rounded-full transition-all"
                      style="width: {pct(row.completed, row.total)}%"
                    ></div>
                  </div>
                  <span class="text-[10px] text-muted mt-1 block">{pct(row.completed, row.total)}%</span>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </section>
    {/if}
  {/if}
</div>
