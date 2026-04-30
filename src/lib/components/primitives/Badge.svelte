<script lang="ts">
  import { clsx, type ClassValue } from "clsx";
  import { twMerge } from "tailwind-merge";

  function cn(...inputs: ClassValue[]) {
    return twMerge(clsx(inputs));
  }

  interface Props {
    text: string;
    variant?: 'pending' | 'active' | 'overdue' | 'completed' | 'disputed' | 'resolved' | 'priority-high' | 'priority-medium' | 'priority-low';
    class?: string;
  }

  let { text, variant = 'active', class: className }: Props = $props();

  const variants = {
    active: "bg-pale-green text-deep-green border-deep-green/10",
    pending: "bg-soft-stone text-muted border-hairline",
    overdue: "bg-error/10 text-error border-error/20",
    completed: "bg-action-blue/10 text-action-blue border-action-blue/20",
    disputed: "bg-coral/10 text-coral border-coral/20",
    resolved: "bg-focus-blue/10 text-focus-blue border-focus-blue/20",
    'priority-high': "bg-error text-on-primary",
    'priority-medium': "bg-focus-blue text-on-primary",
    'priority-low': "bg-slate text-on-primary",
  };

  const base = "px-2.5 py-0.5 rounded-full text-[11px] font-mono uppercase tracking-wider border transition-colors";
</script>

<span class={cn(base, variants[variant as keyof typeof variants], className)}>
  {text}
</span>
