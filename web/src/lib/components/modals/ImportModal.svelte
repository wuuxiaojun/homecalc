<script lang="ts">
  import { appState } from '../../state/appState.svelte';
  import { importPurchaseFromFile, parseAndValidatePurchaseJson } from '../../services/importExport';
  import type { Purchase, SlotId } from '../../state/types';

  let { onClose }: { onClose: () => void } = $props();

  let targetSlotId = $state<SlotId>(appState.activeSlotId);
  let rawJsonText = $state<string>('');
  let errorMessage = $state<string | null>(null);
  let isDragging = $state(false);

  function handleJsonTextImport() {
    errorMessage = null;
    try {
      const p = parseAndValidatePurchaseJson(rawJsonText);
      appState.loadPurchaseIntoSlot(targetSlotId, p);
      appState.setActiveSlot(targetSlotId);
      onClose();
    } catch (err: any) {
      errorMessage = err?.message || 'Failed to parse JSON.';
    }
  }

  async function handleFileInput(e: Event) {
    const target = e.target as HTMLInputElement;
    const file = target.files?.[0];
    if (!file) return;
    errorMessage = null;
    try {
      const p = await importPurchaseFromFile(file);
      appState.loadPurchaseIntoSlot(targetSlotId, p);
      appState.setActiveSlot(targetSlotId);
      onClose();
    } catch (err: any) {
      errorMessage = err?.message || 'Failed to import file.';
    }
  }

  async function handleDrop(e: DragEvent) {
    e.preventDefault();
    isDragging = false;
    const file = e.dataTransfer?.files?.[0];
    if (!file) return;
    errorMessage = null;
    try {
      const p = await importPurchaseFromFile(file);
      appState.loadPurchaseIntoSlot(targetSlotId, p);
      appState.setActiveSlot(targetSlotId);
      onClose();
    } catch (err: any) {
      errorMessage = err?.message || 'Failed to import dropped file.';
    }
  }
</script>

<div class="fixed inset-0 z-50 flex items-center justify-center p-3 sm:p-4 bg-black/80 backdrop-blur-sm animate-in fade-in duration-200">
  <div class="relative w-full max-w-xl max-h-[90vh] rounded-2xl bg-zinc-950 border border-zinc-800 shadow-2xl flex flex-col overflow-hidden">
    <!-- Header -->
    <div class="p-4 sm:p-5 border-b border-zinc-800/80 flex items-center justify-between gap-2">
      <div class="flex items-center gap-2.5 sm:gap-3 min-w-0">
        <span class="text-xl sm:text-2xl shrink-0">📥</span>
        <div class="min-w-0">
          <h2 class="text-sm sm:text-base font-bold text-white truncate">Import Scenario File</h2>
          <p class="text-[11px] sm:text-xs text-zinc-400 truncate">Load a CLI .json scenario file into any workspace slot</p>
        </div>
      </div>
      <button
        class="w-8 h-8 rounded-lg bg-zinc-900 border border-zinc-800 hover:bg-zinc-800 text-zinc-400 hover:text-white flex items-center justify-center transition-colors text-base shrink-0"
        onclick={onClose}
      >
        ✕
      </button>
    </div>

    <div class="p-4 sm:p-6 space-y-4 sm:space-y-5 overflow-y-auto">
      <!-- Target Slot Selection -->
      <div class="flex flex-wrap sm:flex-nowrap items-center justify-between gap-2 p-3 rounded-xl bg-zinc-900/60 border border-zinc-800/80 text-xs">
        <span class="font-semibold text-zinc-300">Target Workspace Slot:</span>
        <div class="flex items-center gap-1.5 font-mono">
          <button
            class="px-2.5 py-1 rounded transition-colors {targetSlotId === 1 ? 'bg-emerald-600 text-white font-bold' : 'bg-zinc-950 text-zinc-400 hover:text-zinc-200'}"
            onclick={() => targetSlotId = 1}
          >
            Slot 1
          </button>
          <button
            class="px-2.5 py-1 rounded transition-colors {targetSlotId === 2 ? 'bg-emerald-600 text-white font-bold' : 'bg-zinc-950 text-zinc-400 hover:text-zinc-200'}"
            onclick={() => targetSlotId = 2}
          >
            Slot 2
          </button>
          <button
            class="px-2.5 py-1 rounded transition-colors {targetSlotId === 3 ? 'bg-emerald-600 text-white font-bold' : 'bg-zinc-950 text-zinc-400 hover:text-zinc-200'}"
            onclick={() => targetSlotId = 3}
          >
            Slot 3
          </button>
        </div>
      </div>

      <!-- Drag & Drop Zone -->
      <div
        class="border-2 border-dashed rounded-2xl p-4 sm:p-6 text-center transition-all cursor-pointer {isDragging ? 'border-emerald-500 bg-emerald-950/20' : 'border-zinc-800 hover:border-zinc-700 bg-zinc-900/20'}"
        ondragover={(e) => { e.preventDefault(); isDragging = true; }}
        ondragleave={() => isDragging = false}
        ondrop={handleDrop}
        role="region"
        aria-label="Scenario JSON file drop zone"
      >
        <label for="scenario-file-input" class="cursor-pointer space-y-2 block">
          <div class="text-2xl sm:text-3xl">📁</div>
          <div class="text-xs font-semibold text-zinc-200">
            Click to browse or drag & drop scenario <span class="font-mono text-emerald-400">.json</span> file
          </div>
          <p class="text-[11px] text-zinc-500 font-mono">Supports all standard CLI scenario formats</p>
          <input
            id="scenario-file-input"
            type="file"
            accept=".json,application/json"
            class="hidden"
            onchange={handleFileInput}
          />
        </label>
      </div>

      <!-- Textarea Paste Option -->
      <div class="space-y-2">
        <label for="json-paste-area" class="text-xs font-semibold text-zinc-400 block">Or Paste Scenario JSON Text:</label>
        <textarea
          id="json-paste-area"
          rows="4"
          placeholder="Paste raw scenario JSON here..."
          class="w-full p-3 rounded-xl bg-zinc-950 border border-zinc-800 text-xs font-mono text-zinc-300 focus:border-emerald-500 focus:outline-none placeholder:text-zinc-700"
          bind:value={rawJsonText}
        ></textarea>
      </div>

      {#if errorMessage}
        <div class="p-3 rounded-xl bg-rose-950/80 border border-rose-800 text-xs text-rose-300 font-mono">
          ⚠️ {errorMessage}
        </div>
      {/if}

      <div class="flex justify-end gap-2 pt-2">
        <button
          class="px-4 py-2 rounded-lg bg-zinc-900 hover:bg-zinc-800 text-zinc-300 text-xs font-medium transition-colors"
          onclick={onClose}
        >
          Cancel
        </button>
        <button
          class="px-5 py-2 rounded-lg bg-emerald-600 hover:bg-emerald-500 text-white text-xs font-semibold shadow-sm transition-colors disabled:opacity-40"
          disabled={!rawJsonText.trim()}
          onclick={handleJsonTextImport}
        >
          Import JSON
        </button>
      </div>
    </div>
  </div>
</div>
