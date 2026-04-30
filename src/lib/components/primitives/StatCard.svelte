<script lang="ts">
  interface Props {
    label: string;
    value: string | number;
    trend?: string;
    variant?: 'default' | 'overdue';
  }

  let { label, value, trend, variant = 'default' }: Props = $props();
</script>

<div class="bg-canvas border border-card-border p-6 rounded-sm transition-all hover:border-hairline group">
  <div class="flex justify-between items-start mb-4">
    <span class="text-[12px] font-mono uppercase tracking-[0.28px] text-muted group-hover:text-ink transition-colors">
      {label}
    </span>
    {#if trend}
      <span class="text-[12px] font-medium text-action-blue bg-action-blue/5 px-2 py-0.5 rounded-full">
        {trend}
      </span>
    {/if}
  </div>
  
  <div class="flex items-baseline gap-2">
    <span class={variant === 'overdue' ? 'text-4xl font-display text-error' : 'text-4xl font-display text-primary'}>
      {value}
    </span>
    {#if variant === 'overdue' && Number(value) > 0}
      <span class="animate-pulse w-2 h-2 rounded-full bg-error"></span>
    {/if}
  </div>
</div>
