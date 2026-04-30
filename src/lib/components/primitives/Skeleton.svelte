<script lang="ts">
  import { clsx, type ClassValue } from "clsx";
  import { twMerge } from "tailwind-merge";

  function cn(...inputs: ClassValue[]) {
    return twMerge(clsx(inputs));
  }

  interface Props {
    class?: string;
  }

  let { class: className }: Props = $props();
</script>

<div class={cn("animate-pulse bg-soft-stone/50 rounded-sm", className)}></div>
