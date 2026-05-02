import { describe, it, expect, vi, beforeEach } from 'vitest';

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

import { invoke } from '@tauri-apps/api/core';

const mockInvoke = vi.mocked(invoke);

const makePaginatedResponse = (count = 2) => ({
  jobs: Array.from({ length: count }, (_, i) => ({
    id: `JOB-00${i + 1}`,
    title: `Job ${i + 1}`,
    status: 'active',
    priority: 'medium',
    company_id: 'c1',
    assigned_person_id: 'p1',
    deadline: new Date().toISOString(),
    created_at: new Date().toISOString(),
    updated_at: new Date().toISOString(),
  })),
  total: count,
});

beforeEach(() => {
  vi.clearAllMocks();
});

describe('JobsStore', () => {
  it('fetchJobs populates jobs and total', async () => {
    mockInvoke.mockResolvedValueOnce(makePaginatedResponse(2));

    const { jobsStore } = await import('$lib/stores/jobs.svelte');
    await jobsStore.fetchJobs();

    expect(jobsStore.jobs).toHaveLength(2);
    expect(jobsStore.total).toBe(2);
    expect(jobsStore.error).toBeNull();
  });

  it('setPage updates page and triggers fetchJobs with correct offset', async () => {
    mockInvoke.mockResolvedValue(makePaginatedResponse(1));

    const { jobsStore } = await import('$lib/stores/jobs.svelte');
    jobsStore.pageSize = 10;
    jobsStore.setPage(3);

    // Wait for async fetch
    await new Promise((r) => setTimeout(r, 0));

    // offset should be (3-1)*10 = 20
    expect(mockInvoke).toHaveBeenCalledWith(
      'get_jobs',
      expect.objectContaining({ offset: 20, limit: 10 }),
    );
    expect(jobsStore.page).toBe(3);
  });

  it('sets error state on fetch failure', async () => {
    mockInvoke.mockRejectedValueOnce('DB error');

    const { jobsStore } = await import('$lib/stores/jobs.svelte');
    await jobsStore.fetchJobs();

    expect(jobsStore.error).toBe('DB error');
  });
});
