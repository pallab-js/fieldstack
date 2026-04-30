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

export const api = {
  jobs: {
    list: (statusFilter?: string) =>
      invoke<Job[]>("get_jobs", { statusFilter }),
    create: (payload: {
      title: string; description?: string; priority: Priority;
      company_id: string; assigned_person_id: string; deadline: string;
    }) => invoke<string>("create_job", payload),
    updateStatus: (jobId: string, status: JobStatus) =>
      invoke<void>("update_job_status", { jobId, status }),
    dispute: (jobId: string, reason: string) =>
      invoke<void>("dispute_job", { jobId, reason }),
    resolve: (jobId: string) =>
      invoke<void>("resolve_job", { jobId }),
    getDetails: (jobId: string) =>
      invoke<{ job: Job; proofs: any[]; audit_log: any[] }>("get_job_details", { jobId }),
    syncOverdue: () => invoke<number>("manual_sync_overdue"),
  },

  companies: {
    list: () => invoke<Company[]>("get_companies"),
    create: (name: string, logoUrl?: string) =>
      invoke<string>("create_company", { name, logoUrl }),
    update: (id: string, name: string, logoUrl?: string) =>
      invoke<void>("update_company", { id, name, logoUrl }),
    delete: (id: string) => invoke<void>("delete_company", { id }),
  },

  people: {
    list: () => invoke<Person[]>("get_people"),
    create: (name: string, email: string | undefined, phone: string | undefined, companyIds: string[]) =>
      invoke<string>("create_person", { name, email, phone, companyIds }),
    update: (id: string, name: string, email: string | undefined, phone: string | undefined, companyIds: string[]) =>
      invoke<void>("update_person", { id, name, email, phone, companyIds }),
    delete: (id: string) => invoke<void>("delete_person", { id }),
    getCompanies: (personId: string) =>
      invoke<string[]>("get_person_companies", { personId }),
  },

  proofs: {
    save: (payload: { jobId: string; proofType: string; sourcePath: string; submittedBy: string }) =>
      invoke<string>("save_proof_file", payload),
  },

  drafts: {
    get: (id: string) => invoke<string | null>("get_draft", { id }),
    save: (id: string, payload: string) => invoke<void>("save_draft", { id, payload }),
    delete: (id: string) => invoke<void>("delete_draft", { id }),
  },

  reports: {
    summary: () => invoke<ReportSummary>("get_report_summary"),
    byCompany: () => invoke<JobsByCompany[]>("get_jobs_by_company"),
    byPerson: () => invoke<JobsByPerson[]>("get_jobs_by_person"),
    overTime: () => invoke<StatusOverTime[]>("get_jobs_by_status_over_time"),
  },

  settings: {
    getConfig: () => invoke<{ key: string; value: string }[]>("get_app_config"),
    setConfig: (key: string, value: string) => invoke<void>("set_app_config", { key, value }),
    resetJobData: () => invoke<void>("reset_job_data"),
  },
};
