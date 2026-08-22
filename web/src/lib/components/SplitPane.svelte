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

<div class="flex-1 flex flex-col lg:flex-row min-h-0 overflow-hidden w-full h-full">
  <!-- Left Pane (Configuration & Parameters) with independent scrollbar -->
  <aside class="w-full lg:w-[440px] xl:w-[480px] shrink-0 border-b lg:border-b-0 lg:border-r border-zinc-800/80 bg-zinc-950/70 overflow-y-auto max-h-full flex flex-col custom-scrollbar">
    {@render left()}
  </aside>

  <!-- Right Pane (Analytical Workspace) with independent scrollbar -->
  <main class="flex-1 bg-[#09090b] overflow-y-auto min-w-0 h-full p-4 lg:p-6 custom-scrollbar flex flex-col">
    <div class="w-full space-y-6 flex-1">
      {@render right()}
    </div>
  </main>
</div>
