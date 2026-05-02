import { describe, it, expect, vi, beforeEach } from 'vitest';

// Mock @tauri-apps/api/core before importing invoke.ts
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

import { invoke } from '@tauri-apps/api/core';

// Import after mock is set up
const { api } = await import('$lib/utils/invoke');

const mockInvoke = vi.mocked(invoke);

beforeEach(() => {
  vi.clearAllMocks();
  vi.useFakeTimers();
});

describe('invokeWithRetry', () => {
  it('resolves immediately on success', async () => {
    mockInvoke.mockResolvedValueOnce([]);
    const result = api.companies.list();
    await vi.runAllTimersAsync();
    expect(await result).toEqual([]);
    expect(mockInvoke).toHaveBeenCalledTimes(1);
  });

  it('retries on "database is locked" error', async () => {
    mockInvoke
      .mockRejectedValueOnce('database is locked')
      .mockRejectedValueOnce('database is locked')
      .mockResolvedValueOnce([]);

    const result = api.companies.list();
    await vi.runAllTimersAsync();
    expect(await result).toEqual([]);
    expect(mockInvoke).toHaveBeenCalledTimes(3);
  });

  it('does not retry on validation errors', async () => {
    mockInvoke.mockRejectedValueOnce("Invalid priority 'bad'");

    await expect(api.companies.list()).rejects.toMatch('Invalid priority');
    expect(mockInvoke).toHaveBeenCalledTimes(1);
  });

  it('throws after exhausting retries', async () => {
    mockInvoke.mockRejectedValue('database is locked');

    let caught: unknown;
    const promise = api.companies.list().catch((e) => { caught = e; });
    await vi.runAllTimersAsync();
    await promise;
    expect(caught).toBeDefined();
    expect(mockInvoke).toHaveBeenCalledTimes(4); // 1 initial + 3 retries
  });
});
