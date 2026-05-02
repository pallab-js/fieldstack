<script lang="ts">
  interface Props {
    total: number;
    page: number;
    pageSize: number;
    onPageChange: (p: number) => void;
  }

  let { total, page, pageSize, onPageChange }: Props = $props();

  const totalPages = $derived(Math.max(1, Math.ceil(total / pageSize)));
  const start = $derived((page - 1) * pageSize + 1);
  const end = $derived(Math.min(page * pageSize, total));
</script>

{#if totalPages > 1}
  <div class="flex items-center justify-between text-sm text-muted py-3 border-t border-hairline">
    <span>{start}–{end} of {total}</span>
    <div class="flex items-center gap-2">
      <button
        onclick={() => onPageChange(page - 1)}
        disabled={page <= 1}
        class="px-3 py-1 border border-hairline rounded-sm disabled:opacity-40 hover:border-action-blue hover:text-action-blue transition-colors"
      >
        ← Prev
      </button>
      <span class="px-2">Page {page} of {totalPages}</span>
      <button
        onclick={() => onPageChange(page + 1)}
        disabled={page >= totalPages}
        class="px-3 py-1 border border-hairline rounded-sm disabled:opacity-40 hover:border-action-blue hover:text-action-blue transition-colors"
      >
        Next →
      </button>
    </div>
  </div>
{/if}
