import { api } from "../utils/invoke";
import type { Job, DashboardStats } from "../types";

class JobsStore {
  jobs = $state<Job[]>([]);
  total = $state(0);
  page = $state(1);
  pageSize = $state(50);
  isLoading = $state(false);
  error = $state<string | null>(null);

  async fetchJobs(statusFilter?: string) {
    this.isLoading = true;
    try {
      const result = await api.jobs.list(statusFilter, this.pageSize, (this.page - 1) * this.pageSize);
      this.jobs = result.jobs;
      this.total = result.total;
      this.error = null;
    } catch (e) {
      this.error = e as string;
    } finally {
      this.isLoading = false;
    }
  }

  setPage(n: number) {
    this.page = n;
    this.fetchJobs().catch(() => {});
  }

  // Derived stats using Svelte 5 runes
  stats = $derived.by<DashboardStats>(() => {
    const active = this.jobs.filter(j => j.status === 'active').length;
    const overdue = this.jobs.filter(j => j.status === 'overdue').length;
    const completed = this.jobs.filter(j => j.status === 'completed').length;
    const total = this.jobs.length;

    const team = new Set(
      this.jobs
        .filter(j => ['active', 'overdue'].includes(j.status))
        .map(j => j.assigned_person_id)
    ).size;

    return {
      activeJobs: active,
      overdueCount: overdue,
      completionRate: total > 0 ? Math.round((completed / total) * 100) : 0,
      activeTeamSize: team
    };
  });
}

export const jobsStore = new JobsStore();
