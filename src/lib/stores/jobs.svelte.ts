import { api } from "../utils/invoke";
import type { Job, DashboardStats } from "../types";

class JobsStore {
  jobs = $state<Job[]>([]);
  isLoading = $state(false);
  error = $state<string | null>(null);

  async fetchJobs() {
    this.isLoading = true;
    try {
      this.jobs = await api.jobs.list();
      this.error = null;
    } catch (e) {
      this.error = e as string;
    } finally {
      this.isLoading = false;
    }
  }

  // Derived stats using Svelte 5 runes
  stats = $derived.by<DashboardStats>(() => {
    const active = this.jobs.filter(j => j.status === 'active').length;
    const overdue = this.jobs.filter(j => j.status === 'overdue').length;
    const completed = this.jobs.filter(j => j.status === 'completed').length;
    const total = this.jobs.length;
    
    // Unique people in active/overdue jobs
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
