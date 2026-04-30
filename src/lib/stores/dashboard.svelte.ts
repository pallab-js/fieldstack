import { api, type ReportSummary, type JobsByCompany, type JobsByPerson, type StatusOverTime } from "../utils/invoke";

class DashboardStore {
  summary = $state<ReportSummary | null>(null);
  byCompany = $state<JobsByCompany[]>([]);
  byPerson = $state<JobsByPerson[]>([]);
  overTime = $state<StatusOverTime[]>([]);
  isLoading = $state(false);
  error = $state<string | null>(null);

  async fetch() {
    this.isLoading = true;
    this.error = null;
    try {
      [this.summary, this.byCompany, this.byPerson, this.overTime] = await Promise.all([
        api.reports.summary(),
        api.reports.byCompany(),
        api.reports.byPerson(),
        api.reports.overTime(),
      ]);
    } catch (e) {
      this.error = String(e);
    } finally {
      this.isLoading = false;
    }
  }

  get topCompanies(): JobsByCompany[] {
    return [...this.byCompany].sort((a, b) => b.total - a.total).slice(0, 3);
  }

  get topPeople(): JobsByPerson[] {
    return [...this.byPerson].sort((a, b) => b.total - a.total).slice(0, 5);
  }
}

export const dashboardStore = new DashboardStore();
