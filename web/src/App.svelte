<script lang="ts">
  import { onMount } from 'svelte';
  let ready = $state(false);

  onMount(() => {
    ready = true;
  });
</script>

<main class="min-h-screen flex flex-col bg-[#09090b] text-zinc-100">
  <div class="p-8 text-center">
    <h1 class="text-3xl font-bold tracking-tight text-white flex items-center justify-center gap-3">
      <span class="text-emerald-400">🏡 Homecalc</span>
      <span class="text-xs px-2 py-0.5 rounded-full bg-emerald-950 text-emerald-300 border border-emerald-800/60 font-mono">v2.0</span>
    </h1>
    <p class="mt-2 text-sm text-zinc-400">The Reactive Web Mortgage & Scenario Engine</p>
  </div>
</main>
