<script lang="ts">
  import { appState } from '../../state/appState.svelte';
  import { exportPurchaseToJson, exportFullReportJson } from '../../services/importExport';
  import { saveCustomScenario } from '../../services/persistence';

  let { onClose }: { onClose: () => void } = $props();

  const slot = $derived(appState.activeSlot);
  const purchase = $derived(slot.purchase);

  let copied = $state(false);
  let savedToStorage = $state(false);

  const jsonSnippet = $derived(purchase ? JSON.stringify(purchase, null, 2) : '');

  function handleCopy() {
    if (!jsonSnippet) return;
    navigator.clipboard.writeText(jsonSnippet);
    copied = true;
    setTimeout(() => copied = false, 2000);
  }

  function handleSaveLocal() {
    if (!purchase) return;
    saveCustomScenario(purchase);
    savedToStorage = true;
    setTimeout(() => savedToStorage = false, 2000);
  }
</script>

<div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/80 backdrop-blur-sm animate-in fade-in duration-200">
  <div class="relative w-full max-w-2xl max-h-[85vh] rounded-2xl bg-zinc-950 border border-zinc-800 shadow-2xl flex flex-col overflow-hidden">
    <!-- Header -->
    <div class="p-5 border-b border-zinc-800/80 flex items-center justify-between">
      <div class="flex items-center gap-3">
        <span class="text-2xl">📤</span>
        <div>
          <h2 class="text-base font-bold text-white">Export Scenario & Reports</h2>
          <p class="text-xs text-zinc-400">Download CLI-compatible JSON scenario files or comprehensive analytical reports</p>
        </div>
      </div>
      <button
        class="w-8 h-8 rounded-lg bg-zinc-900 border border-zinc-800 hover:bg-zinc-800 text-zinc-400 hover:text-white flex items-center justify-center transition-colors text-base"
        onclick={onClose}
      >
        ✕
      </button>
    </div>

    <div class="p-6 space-y-6 overflow-y-auto">
      {#if !purchase}
        <div class="p-8 text-center space-y-3">
          <div class="text-3xl">📦</div>
          <h3 class="text-sm font-bold text-white">Slot {slot.id} is Empty</h3>
          <p class="text-xs text-zinc-400 max-w-xs mx-auto">
            Create or load a scenario into Slot {slot.id} to export JSON data and analytical reports.
          </p>
          <button
            class="mt-2 py-2 px-4 rounded-xl bg-emerald-600 hover:bg-emerald-500 text-white text-xs font-semibold transition-colors"
            onclick={() => { appState.createScenarioInSlot(slot.id); onClose(); }}
          >
            ✨ Create Scenario in Slot {slot.id}
          </button>
        </div>
      {:else}
        <!-- Export Actions Grid -->
        <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
          <!-- 1. CLI JSON -->
          <button
            class="p-4 rounded-xl bg-zinc-900/60 border border-zinc-800/80 hover:border-emerald-500/60 transition-all text-left space-y-1.5 group"
            onclick={() => { if (purchase) { exportPurchaseToJson(purchase); onClose(); } }}
          >
            <div class="flex items-center justify-between">
              <span class="text-sm font-semibold text-white group-hover:text-emerald-400 transition-colors flex items-center gap-2">
                <span>📄</span>
                <span>CLI Scenario JSON</span>
              </span>
              <span class="text-[10px] font-mono px-1.5 py-0.5 rounded bg-emerald-950 text-emerald-300 border border-emerald-800/50">
                .json
              </span>
            </div>
            <p class="text-xs text-zinc-400">100% compatible with Rust `homecalc` CLI toolchain.</p>
          </button>

          <!-- 2. Full Analytical Report -->
          <button
            class="p-4 rounded-xl bg-zinc-900/60 border border-zinc-800/80 hover:border-indigo-500/60 transition-all text-left space-y-1.5 group"
            onclick={() => { exportFullReportJson(slot); onClose(); }}
          >
            <div class="flex items-center justify-between">
              <span class="text-sm font-semibold text-white group-hover:text-indigo-300 transition-colors flex items-center gap-2">
                <span>📊</span>
                <span>Full Analytics Report</span>
              </span>
              <span class="text-[10px] font-mono px-1.5 py-0.5 rounded bg-indigo-950 text-indigo-300 border border-indigo-800/50">
                .json
              </span>
            </div>
            <p class="text-xs text-zinc-400">Contains parameters, KPIs, metrics, and statement ledgers.</p>
          </button>
        </div>

        <!-- Save Local & Quick Copy -->
        <div class="flex items-center justify-between pt-2">
          <button
            class="px-3.5 py-1.5 rounded-lg bg-zinc-900 border border-zinc-800 hover:bg-zinc-800 text-zinc-200 text-xs font-medium transition-colors flex items-center gap-2"
            onclick={handleSaveLocal}
          >
            <span>💾</span>
            <span>{savedToStorage ? 'Saved to Browser!' : 'Save to Browser Library'}</span>
          </button>

          <button
            class="px-3.5 py-1.5 rounded-lg bg-zinc-900 border border-zinc-800 hover:bg-zinc-800 text-zinc-200 text-xs font-medium transition-colors flex items-center gap-2"
            onclick={handleCopy}
          >
            <span>📋</span>
            <span>{copied ? 'Copied to Clipboard!' : 'Copy JSON'}</span>
          </button>
        </div>

        <!-- JSON Preview Box -->
        <div class="space-y-2">
          <span class="text-xs font-semibold text-zinc-400">JSON Schema Preview</span>
          <pre class="p-3.5 rounded-xl bg-zinc-950 border border-zinc-800 text-[11px] font-mono text-zinc-300 max-h-48 overflow-y-auto selection:bg-emerald-500/30"><code>{jsonSnippet}</code></pre>
        </div>
      {/if}
    </div>
  </div>
</div>
