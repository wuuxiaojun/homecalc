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

<div class="flex-1 flex flex-col md:flex-row min-h-0 md:overflow-hidden w-full md:h-full">
  <!-- Left Pane (Configuration & Parameters) with independent scrollbar on desktop -->
  <aside class="w-full md:w-[440px] xl:w-[480px] shrink-0 border-b md:border-b-0 md:border-r border-zinc-800/80 bg-zinc-950/70 md:overflow-y-auto md:max-h-full flex flex-col custom-scrollbar">
    {@render left()}
  </aside>

  <!-- Right Pane (Analytical Workspace) with independent scrollbar on desktop -->
  <main class="flex-1 bg-[#09090b] md:overflow-y-auto min-w-0 md:h-full p-4 md:p-6 custom-scrollbar flex flex-col">
    <div class="w-full space-y-6 flex-1">
      {@render right()}
    </div>
  </main>
</div>
