<script lang="ts">
  interface Bar {
    value: number;
    label?: string;
    color?: string;
  }

  interface Props {
    bars: Bar[];
    height?: number;
    gap?: number;
  }

  let { bars, height = 32, gap = 3 }: Props = $props();

  const maxVal = $derived(Math.max(...bars.map((b: Bar) => b.value), 1));
  const totalWidth = 100; // percentage-based
  const barWidth = $derived((totalWidth - gap * (bars.length - 1)) / bars.length);
</script>

<svg
  viewBox="0 0 100 {height}"
  preserveAspectRatio="none"
  class="w-full"
  style="height: {height}px"
  aria-hidden="true"
>
  {#each bars as bar, i}
    {@const barH = (bar.value / maxVal) * height}
    {@const x = i * (barWidth + gap)}
    <rect
      x={x}
      y={height - barH}
      width={barWidth}
      height={barH}
      fill={bar.color ?? 'currentColor'}
      rx="1"
      class="transition-all duration-300"
    />
  {/each}
</svg>
