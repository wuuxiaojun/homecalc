<script lang="ts">
  import type { Snippet } from 'svelte';

  let {
    left,
    right
  }: {
    left: Snippet;
    right: Snippet;
  } = $props();
</script>

<div class="flex-1 flex flex-col lg:flex-row min-h-0 overflow-hidden">
  <!-- Left Pane (Configuration & Parameters) -->
  <aside class="w-full lg:w-[440px] xl:w-[480px] shrink-0 border-b lg:border-b-0 lg:border-r border-zinc-800/80 bg-zinc-950/70 overflow-y-auto max-h-[50vh] lg:max-h-full flex flex-col">
    {@render left()}
  </aside>

  <!-- Right Pane (Analytical Workspace) -->
  <main class="flex-1 bg-[#09090b] overflow-y-auto p-4 sm:p-6 lg:p-8 min-w-0">
    <div class="max-w-7xl mx-auto space-y-6">
      {@render right()}
    </div>
  </main>
</div>
