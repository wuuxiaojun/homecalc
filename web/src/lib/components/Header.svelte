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

<header class="sticky top-0 z-30 border-b border-zinc-800/80 bg-zinc-950/80 backdrop-blur-md px-4 lg:px-6 py-2.5">
  <div class="max-w-[1920px] mx-auto flex items-center justify-between gap-3">
    <!-- Brand Title & WASM Engine Badge -->
    <div class="flex items-center gap-3">
      <div class="flex items-center gap-2.5">
        <div class="w-8 h-8 rounded-lg bg-emerald-500/10 border border-emerald-500/30 flex items-center justify-center text-emerald-400 font-bold text-lg shadow-sm shadow-emerald-950/50">
          🏡
        </div>
        <div>
          <div class="flex items-center gap-2">
            <span class="font-bold text-base tracking-tight text-white">Homecalc</span>
            <span class="text-[10px] uppercase tracking-wider px-1.5 py-0.5 rounded font-mono font-semibold bg-zinc-800 text-zinc-300 border border-zinc-700/60">v2.0.0</span>
            <span class="inline-flex items-center gap-1 text-[10px] px-2 py-0.5 rounded-full font-mono bg-emerald-950/70 text-emerald-300 border border-emerald-800/50">
              <span class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse"></span>
              Rust WASM
            </span>
          </div>
          <p class="text-[11px] text-zinc-400 hidden sm:block">Reactive Scenario Engine</p>
        </div>
      </div>
    </div>

    <!-- Right Controls: Slot Switcher & Action Tools -->
    <div class="flex items-center gap-2.5">
      <!-- Slot Switcher -->
      <div class="flex items-center gap-1 bg-zinc-900/90 p-1 rounded-xl border border-zinc-800/80">
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
