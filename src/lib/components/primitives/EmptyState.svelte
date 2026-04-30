<script lang="ts">
  import type { Component } from "svelte";
  import { clsx, type ClassValue } from "clsx";
  import { twMerge } from "tailwind-merge";

  function cn(...inputs: ClassValue[]) {
    return twMerge(clsx(inputs));
  }

  interface Props {
    title: string;
    description: string;
    icon?: Component<any>;
    class?: string;
  }

  let { title, description, icon: Icon, class: className }: Props = $props();
</script>

<div class={cn("flex flex-col items-center justify-center p-12 text-center bg-soft-stone/10 border border-dashed border-hairline rounded-lg", className)}>
  {#if Icon}
    <div class="w-12 h-12 rounded-full bg-hairline flex items-center justify-center text-muted mb-4">
      <Icon size={24} />
    </div>
  {/if}
  <h4 class="text-ink font-medium">{title}</h4>
  <p class="text-slate text-sm max-w-xs mt-1">{description}</p>
</div>
