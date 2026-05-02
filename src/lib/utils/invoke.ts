import { invoke } from "@tauri-apps/api/core";
import type { Job, JobStatus, Priority, Company, Person } from "../types";

export interface ReportSummary {
  total_jobs: number;
  completed: number;
  active: number;
  overdue: number;
  disputed: number;
  resolved: number;
  pending: number;
  completion_rate: number;
  avg_completion_days: number;
}

export interface JobsByCompany {
  company_id: string;
  company_name: string;
  total: number;
  completed: number;
  overdue: number;
  active: number;
}

export interface JobsByPerson {
  person_id: string;
  person_name: string;
  initials: string;
  avatar_color?: string;
  total: number;
  completed: number;
  overdue: number;
}

export interface StatusOverTime {
  month: string;
  completed: number;
  overdue: number;
  created: number;
}

export interface PaginatedJobs {
  jobs: Job[];
  total: number;
}

const RETRYABLE = ["database is locked", "i/o error"];

async function invokeWithRetry<T>(
  cmd: string,
  args?: Record<string, unknown>,
  maxRetries = 3,
  baseDelayMs = 200,
): Promise<T> {
  for (let attempt = 0; attempt <= maxRetries; attempt++) {
    try {
      return await invoke<T>(cmd, args);
    } catch (err) {
      const msg = String(err).toLowerCase();
      const retryable = RETRYABLE.some((e) => msg.includes(e));
      if (!retryable || attempt === maxRetries) throw err;
      await new Promise((r) => setTimeout(r, baseDelayMs * 2 ** attempt));
    }
  }
  // unreachable, but satisfies TypeScript
  throw new Error("invokeWithRetry exhausted");
}

export const api = {
  jobs: {
    list: (statusFilter?: string, limit?: number, offset?: number) =>
      invokeWithRetry<PaginatedJobs>("get_jobs", { statusFilter, limit, offset }),
    create: (payload: {
      title: string; description?: string; priority: Priority;
      company_id: string; assigned_person_id: string; deadline: string;
    }) => invokeWithRetry<string>("create_job", payload),
    updateStatus: (jobId: string, status: JobStatus) =>
      invokeWithRetry<void>("update_job_status", { jobId, status }),
    dispute: (jobId: string, reason: string) =>
      invokeWithRetry<void>("dispute_job", { jobId, reason }),
    resolve: (jobId: string) =>
      invokeWithRetry<void>("resolve_job", { jobId }),
    getDetails: (jobId: string) =>
      invokeWithRetry<{ job: Job; proofs: unknown[]; audit_log: unknown[] }>("get_job_details", { jobId }),
    syncOverdue: () => invokeWithRetry<number>("manual_sync_overdue"),
  },

  companies: {
    list: () => invokeWithRetry<Company[]>("get_companies"),
    create: (name: string, logoUrl?: string) =>
      invokeWithRetry<string>("create_company", { name, logoUrl }),
    update: (id: string, name: string, logoUrl?: string) =>
      invokeWithRetry<void>("update_company", { id, name, logoUrl }),
    delete: (id: string) => invokeWithRetry<void>("delete_company", { id }),
  },

  people: {
    list: () => invokeWithRetry<Person[]>("get_people"),
    create: (name: string, email: string | undefined, phone: string | undefined, companyIds: string[]) =>
      invokeWithRetry<string>("create_person", { name, email, phone, companyIds }),
    update: (id: string, name: string, email: string | undefined, phone: string | undefined, companyIds: string[]) =>
      invokeWithRetry<void>("update_person", { id, name, email, phone, companyIds }),
    delete: (id: string) => invokeWithRetry<void>("delete_person", { id }),
    getCompanies: (personId: string) =>
      invokeWithRetry<string[]>("get_person_companies", { personId }),
  },

  proofs: {
    save: (payload: { jobId: string; proofType: string; sourcePath: string; submittedBy: string }) =>
      invokeWithRetry<string>("save_proof_file", payload),
  },

  drafts: {
    get: (id: string) => invokeWithRetry<string | null>("get_draft", { id }),
    save: (id: string, payload: string) => invokeWithRetry<void>("save_draft", { id, payload }),
    delete: (id: string) => invokeWithRetry<void>("delete_draft", { id }),
  },

  reports: {
    summary: () => invokeWithRetry<ReportSummary>("get_report_summary"),
    byCompany: () => invokeWithRetry<JobsByCompany[]>("get_jobs_by_company"),
    byPerson: () => invokeWithRetry<JobsByPerson[]>("get_jobs_by_person"),
    overTime: () => invokeWithRetry<StatusOverTime[]>("get_jobs_by_status_over_time"),
  },

  settings: {
    getConfig: () => invokeWithRetry<{ key: string; value: string }[]>("get_app_config"),
    setConfig: (key: string, value: string) => invokeWithRetry<void>("set_app_config", { key, value }),
    resetJobData: (confirmation: string) => invokeWithRetry<void>("reset_job_data", { confirmation }),
  },
};
