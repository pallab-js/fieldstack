<script lang="ts">
  import { clsx, type ClassValue } from "clsx";
  import { twMerge } from "tailwind-merge";

  function cn(...inputs: ClassValue[]) {
    return twMerge(clsx(inputs));
  }

  interface Props {
    children: any;
    variant?: 'primary' | 'secondary' | 'outline' | 'ghost' | 'coral';
    size?: 'sm' | 'md' | 'lg';
    class?: string;
    type?: 'button' | 'submit' | 'reset';
    disabled?: boolean;
    onclick?: (e: MouseEvent) => void;
  }

  let { 
    children, 
    variant = 'primary', 
    size = 'md', 
    class: className,
    type = 'button',
    disabled = false,
    onclick 
  }: Props = $props();

  const variants = {
    primary: "bg-primary text-on-primary hover:bg-cohere-black",
    secondary: "bg-soft-stone text-ink hover:bg-hairline",
    outline: "bg-transparent border border-hairline text-ink hover:bg-canvas",
    ghost: "bg-transparent text-muted hover:text-ink",
    coral: "bg-coral text-on-primary hover:bg-coral/90"
  };

  const sizes = {
    sm: "px-3 py-1.5 text-xs",
    md: "px-6 py-2.5 text-sm",
    lg: "px-8 py-4 text-base"
  };

  const base = "inline-flex items-center justify-center font-medium transition-all duration-200 rounded-pill focus:outline-none focus:ring-2 focus:ring-focus-blue focus:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none";
</script>

<button
  {type}
  {disabled}
  class={cn(base, variants[variant as keyof typeof variants], sizes[size as keyof typeof sizes], className)}
  {onclick}
>
  {@render children()}
</button>
