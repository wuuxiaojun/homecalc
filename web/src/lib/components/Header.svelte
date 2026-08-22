<script lang="ts">
  import { appState } from '../state/appState.svelte';
  import type { SlotId } from '../state/types';

  let { onOpenLibrary, onOpenExport, onOpenImport }: {
    onOpenLibrary?: () => void;
    onOpenExport?: () => void;
    onOpenImport?: () => void;
  } = $props();

  const slots: { id: SlotId; label: string }[] = [
    { id: 1, label: 'Slot 1' },
    { id: 2, label: 'Slot 2' },
    { id: 3, label: 'Slot 3' }
  ];

  function handleSlotSelect(id: SlotId) {
    appState.setActiveSlot(id);
  }
</script>

<header class="sticky top-0 z-30 border-b border-zinc-800/80 bg-zinc-950/90 backdrop-blur-md px-2 sm:px-4 lg:px-6 py-2.5">
  <div class="max-w-[1920px] mx-auto flex items-center justify-between gap-4">
    <!-- Brand Title & WASM Engine Badge (Enlarged & positioned closer to left edge) -->
    <div class="flex items-center gap-3">
      <div class="w-11 h-11 sm:w-12 sm:h-12 rounded-xl bg-emerald-500/15 border border-emerald-500/35 flex items-center justify-center text-2xl sm:text-[26px] shadow-lg shadow-emerald-950/50 shrink-0">
        🏡
      </div>
      <div>
        <div class="flex items-center gap-2 sm:gap-2.5">
          <span class="font-black text-xl sm:text-2xl tracking-tight text-white">Homecalc</span>
          <span class="text-xs uppercase tracking-wider px-2 py-0.5 rounded-md font-mono font-bold bg-zinc-800 text-zinc-200 border border-zinc-700/80 shadow-sm">
            v2.0
          </span>
          <span class="inline-flex items-center gap-1.5 text-xs px-2.5 py-0.5 rounded-full font-mono bg-emerald-950/90 text-emerald-300 border border-emerald-800/70 font-semibold shadow-sm">
            <span class="w-2 h-2 rounded-full bg-emerald-400 animate-pulse"></span>
            Rust WASM
          </span>
        </div>
        <p class="text-xs text-zinc-400 font-medium mt-0.5 hidden sm:block">Reactive Mortgage & Scenario Engine</p>
      </div>
    </div>

    <!-- Right Controls: Slot Switcher & Action Tools -->
    <div class="flex items-center gap-2.5">
      <!-- Slot Switcher -->
      <div class="flex items-center gap-1 bg-zinc-900/90 p-1 rounded-xl border border-zinc-800/80 shadow-inner">
        {#each slots as s}
          {@const currentSlot = appState.getSlot(s.id)}
          <button
            class="px-3 py-1 text-xs font-mono font-medium rounded-lg transition-all flex items-center gap-1.5 {appState.activeSlotId === s.id ? 'bg-emerald-500/20 text-emerald-300 border border-emerald-500/40 shadow-sm' : 'text-zinc-400 hover:text-zinc-200'}"
            onclick={() => handleSlotSelect(s.id)}
            title={currentSlot.purchase.name}
          >
            <span class="w-1.5 h-1.5 rounded-full {appState.activeSlotId === s.id ? 'bg-emerald-400' : 'bg-zinc-600'}"></span>
            <span>{s.label}</span>
          </button>
        {/each}
      </div>

      <!-- Action Buttons -->
      <div class="flex items-center gap-1.5">
        <button
          class="px-3 py-1.5 text-xs font-medium rounded-lg bg-zinc-900 border border-zinc-800 hover:bg-zinc-800 text-zinc-300 transition-colors flex items-center gap-1.5 shadow-sm"
          onclick={onOpenLibrary}
          title="Open Scenario Library"
        >
          <span>📚</span>
          <span class="hidden sm:inline">Presets</span>
        </button>

        <button
          class="px-3 py-1.5 text-xs font-medium rounded-lg bg-zinc-900 border border-zinc-800 hover:bg-zinc-800 text-zinc-300 transition-colors flex items-center gap-1.5 shadow-sm"
          onclick={onOpenImport}
          title="Import Scenario JSON"
        >
          <span>📥</span>
          <span class="hidden lg:inline">Import</span>
        </button>

        <button
          class="px-3 py-1.5 text-xs font-medium rounded-lg bg-zinc-900 border border-zinc-800 hover:bg-zinc-800 text-zinc-300 transition-colors flex items-center gap-1.5 shadow-sm"
          onclick={onOpenExport}
          title="Export Scenario JSON / Report"
        >
          <span>📤</span>
          <span class="hidden lg:inline">Export</span>
        </button>
      </div>
    </div>
  </div>
</header>
