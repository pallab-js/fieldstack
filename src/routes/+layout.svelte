<script lang="ts">
  import "../app.css";
  let { children } = $props();
</script>

<div class="titlebar-drag"></div>

<main class="min-h-screen bg-canvas selection:bg-coral/30">
  {@render children()}
</main>

<style>
  :global(html, body) {
    margin: 0;
    padding: 0;
    overflow-x: hidden;
  }
</style>
