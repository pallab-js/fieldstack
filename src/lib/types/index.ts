export type Priority = 'low' | 'medium' | 'high' | 'critical';
export type JobStatus = 'pending' | 'active' | 'overdue' | 'disputed' | 'resolved' | 'completed';

export interface Company {
  id: string;
  name: string;
  logo_url?: string;
  created_at: string;
}

export interface Person {
  id: string;
  name: string;
  email?: string;
  phone?: string;
  avatar_color?: string;
  initials: string;
}

export interface Job {
  id: string; // JOB-XXX
  title: string;
  description: string;
  status: JobStatus;
  priority: Priority;
  company_id: string;
  assigned_person_id: string;
  deadline: string;
  created_at: string;
  updated_at: string;
  completion_date?: string;
}

export interface Proof {
  id: string;
  job_id: string;
  file_path: string;
  file_type: 'photo' | 'video' | 'audio' | 'document';
  submitted_by: string;
  submitted_at: string;
  is_locked: boolean;
  dispute_reason?: string;
}

export interface AuditLog {
  id: string;
  job_id: string;
  event_type: string;
  description: string;
  actor: string;
  timestamp: string;
  metadata?: string; // JSON string
}

export interface AppConfig {
  key: string;
  value: string;
}

export interface DashboardStats {
  activeJobs: number;
  overdueCount: number;
  completionRate: number;
  activeTeamSize: number;
}
