<script lang="ts">
  import { appState, createDefaultScenario } from '../../state/appState.svelte';
  import { getCustomScenarios, deleteCustomScenario, type SavedUserScenario } from '../../services/persistence';
  import { exportPurchaseToJson } from '../../services/importExport';
  import type { Purchase, SlotId } from '../../state/types';

  let { onClose, onOpenImport }: { onClose: () => void; onOpenImport?: () => void } = $props();

  let searchQuery = $state('');
  let customList = $state<SavedUserScenario[]>(getCustomScenarios());

  const defaultScenario = createDefaultScenario('Standard 30Y Mortgage');

  const filteredCustom = $derived.by(() => {
    const q = searchQuery.toLowerCase().trim();
    if (!q) return customList;
    return customList.filter(s =>
      s.name.toLowerCase().includes(q)
    );
  });

  function handleLoad(purchase: Purchase, slotId: SlotId) {
    appState.loadPurchaseIntoSlot(slotId, purchase);
    appState.setActiveSlot(slotId);
    onClose();
  }

  function handleDeleteCustom(id: string, name: string) {
    if (confirm(`Delete saved scenario "${name}"?`)) {
      deleteCustomScenario(id);
      customList = getCustomScenarios();
    }
  }
</script>

<div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/80 backdrop-blur-sm animate-in fade-in duration-200">
  <div class="relative w-full max-w-3xl max-h-[85vh] rounded-2xl bg-zinc-950 border border-zinc-800 shadow-2xl flex flex-col overflow-hidden">
    <!-- Header -->
    <div class="p-5 border-b border-zinc-800/80 flex items-center justify-between">
      <div class="flex items-center gap-3">
        <span class="text-2xl">📚</span>
        <div>
          <h2 class="text-base font-bold text-white">Scenario Library & Presets</h2>
          <p class="text-xs text-zinc-400">Load the standard baseline scenario or your custom saved scenarios into any slot</p>
        </div>
      </div>
      <button
        class="w-8 h-8 rounded-lg bg-zinc-900 border border-zinc-800 hover:bg-zinc-800 text-zinc-400 hover:text-white flex items-center justify-center transition-colors text-base"
        onclick={onClose}
      >
        ✕
      </button>
    </div>

    <!-- Modal Content -->
    <div class="flex-1 overflow-y-auto p-5 space-y-6">
      <!-- Section 1: Standard Baseline Scenario -->
      <div class="space-y-3">
        <div class="flex items-center justify-between">
          <h3 class="text-xs font-semibold text-zinc-300 uppercase tracking-wider font-mono flex items-center gap-2">
            <span>⭐ Baseline Preset</span>
          </h3>
        </div>

        <div class="p-4 rounded-xl bg-zinc-900/70 border border-emerald-500/30 shadow-sm shadow-emerald-950/20 space-y-3">
          <div class="flex flex-col sm:flex-row sm:items-start justify-between gap-2">
            <div>
              <div class="flex items-center gap-2">
                <h4 class="text-sm font-bold text-white">Standard 30Y Mortgage</h4>
                <span class="text-[10px] font-mono px-2 py-0.5 rounded bg-emerald-950 text-emerald-300 border border-emerald-800/60 font-semibold">
                  Standard Baseline
                </span>
              </div>
              <p class="text-xs text-zinc-400 mt-1 leading-relaxed">
                Standard 30-Year Fixed Mortgage ($800k @ 6.50%), 20% Cash Down Payment ($200k @ 4.0% yield), baseline Property Tax (1.20%), Insurance ($2,400/yr), and HOA ($120/mo).
              </p>
            </div>
            <div class="text-right font-mono shrink-0">
              <span class="text-base font-bold text-emerald-400 tabular-nums">$1,000,000</span>
              <div class="text-[10px] text-zinc-500">Purchase Price</div>
            </div>
          </div>

          <div class="flex flex-wrap items-center gap-1.5 pt-1">
            <span class="text-[10px] font-mono px-2 py-0.5 rounded bg-emerald-950/80 text-emerald-300 border border-emerald-800/50">
              💵 $200k Cash (20%)
            </span>
            <span class="text-[10px] font-mono px-2 py-0.5 rounded bg-indigo-950/80 text-indigo-300 border border-indigo-800/50">
              🏦 $800k Mort @ 6.5% (30y)
            </span>
            <span class="text-[10px] font-mono px-2 py-0.5 rounded bg-zinc-800 text-zinc-300 border border-zinc-700/50">
              🏛️ 1.2% Tax
            </span>
            <span class="text-[10px] font-mono px-2 py-0.5 rounded bg-zinc-800 text-zinc-300 border border-zinc-700/50">
              🛡️ $2,400/yr Ins
            </span>
            <span class="text-[10px] font-mono px-2 py-0.5 rounded bg-zinc-800 text-zinc-300 border border-zinc-700/50">
              🏢 $120/mo HOA
            </span>
          </div>

          <div class="pt-3 border-t border-zinc-800/80 flex items-center justify-between">
            <span class="text-xs text-zinc-400 font-medium">Load baseline preset into:</span>
            <div class="flex items-center gap-1.5 font-mono">
              <button
                class="px-3 py-1 text-xs font-semibold rounded-lg bg-emerald-600 hover:bg-emerald-500 text-white transition-colors shadow-sm"
                onclick={() => handleLoad(defaultScenario, 1)}
              >
                Slot 1
              </button>
              <button
                class="px-3 py-1 text-xs font-semibold rounded-lg bg-emerald-600 hover:bg-emerald-500 text-white transition-colors shadow-sm"
                onclick={() => handleLoad(defaultScenario, 2)}
              >
                Slot 2
              </button>
              <button
                class="px-3 py-1 text-xs font-semibold rounded-lg bg-emerald-600 hover:bg-emerald-500 text-white transition-colors shadow-sm"
                onclick={() => handleLoad(defaultScenario, 3)}
              >
                Slot 3
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Section 2: User Saved Custom Scenarios -->
      <div class="space-y-3 pt-2">
        <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-2">
          <h3 class="text-xs font-semibold text-zinc-300 uppercase tracking-wider font-mono flex items-center gap-2">
            <span>📁 My Saved Scenarios ({customList.length})</span>
          </h3>

          {#if customList.length > 0}
            <input
              type="text"
              placeholder="Filter saved scenarios..."
              class="w-full sm:w-60 px-3 py-1 rounded-lg bg-zinc-900 border border-zinc-800 text-xs text-zinc-200 focus:border-emerald-500 focus:outline-none placeholder:text-zinc-600"
              bind:value={searchQuery}
            />
          {/if}
        </div>

        {#if customList.length === 0}
          <div class="p-8 rounded-xl bg-zinc-900/40 border border-zinc-800/80 text-center space-y-3">
            <div class="text-2xl">💾</div>
            <div>
              <h4 class="text-sm font-semibold text-zinc-200">No Saved Custom Scenarios</h4>
              <p class="text-xs text-zinc-400 mt-1 max-w-sm mx-auto leading-relaxed">
                Save any scenario from the parameter suite using the <span class="text-zinc-200 font-mono">"Save"</span> button, or import a JSON file.
              </p>
            </div>
            {#if onOpenImport}
              <button
                class="px-3.5 py-1.5 rounded-lg bg-zinc-800 hover:bg-zinc-700 text-zinc-200 text-xs font-medium transition-colors inline-flex items-center gap-1.5"
                onclick={() => { onClose(); onOpenImport?.(); }}
              >
                <span>📥</span>
                <span>Import JSON Scenario</span>
              </button>
            {/if}
          </div>
        {:else if filteredCustom.length === 0}
          <div class="p-8 text-center text-zinc-500 text-xs font-mono">
            No scenarios match your search query "{searchQuery}".
          </div>
        {:else}
          <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
            {#each filteredCustom as item}
              <div class="p-4 rounded-xl bg-zinc-900/50 border border-zinc-800/80 hover:border-zinc-700 transition-all flex flex-col justify-between space-y-3">
                <div>
                  <div class="flex items-start justify-between gap-2">
                    <h4 class="text-sm font-semibold text-white">{item.name}</h4>
                    <button
                      class="text-zinc-500 hover:text-rose-400 text-xs transition-colors p-1"
                      onclick={() => handleDeleteCustom(item.id, item.name)}
                      title="Delete scenario"
                    >
                      🗑️
                    </button>
                  </div>
                  <div class="flex items-center justify-between text-xs mt-1">
                    <span class="font-mono font-bold text-emerald-400 tabular-nums">
                      ${item.purchase.house.purchase_price.toLocaleString()}
                    </span>
                    <span class="text-[10px] text-zinc-500 font-mono">
                      {new Date(item.savedAt).toLocaleDateString()}
                    </span>
                  </div>

                  <!-- Tool Tags -->
                  <div class="flex flex-wrap items-center gap-1.5 mt-2.5">
                    {#each item.purchase.tools as t}
                      {#if 'Cash' in t && t.Cash}
                        <span class="text-[10px] font-mono px-2 py-0.5 rounded bg-emerald-950/80 text-emerald-300 border border-emerald-800/50">
                          💵 ${(t.Cash.amount / 1000).toFixed(0)}k Cash
                        </span>
                      {:else if 'Mortgage' in t && t.Mortgage}
                        <span class="text-[10px] font-mono px-2 py-0.5 rounded bg-indigo-950/80 text-indigo-300 border border-indigo-800/50">
                          🏦 ${(t.Mortgage.amount / 1000).toFixed(0)}k @ {t.Mortgage.rate}%
                        </span>
                      {:else if 'Loc' in t && t.Loc}
                        <span class="text-[10px] font-mono px-2 py-0.5 rounded bg-amber-950/80 text-amber-300 border border-amber-800/50">
                          💳 ${(t.Loc.amount / 1000).toFixed(0)}k LOC
                        </span>
                      {/if}
                    {/each}
                  </div>
                </div>

                <!-- Action Bar -->
                <div class="pt-2 border-t border-zinc-800/60 flex items-center justify-between">
                  <button
                    class="text-[11px] font-mono text-zinc-400 hover:text-zinc-200 transition-colors flex items-center gap-1"
                    onclick={() => exportPurchaseToJson(item.purchase)}
                    title="Export to JSON file"
                  >
                    <span>📄 Export</span>
                  </button>
                  <div class="flex items-center gap-1">
                    <span class="text-[10px] font-mono text-zinc-500 mr-1">Load to:</span>
                    <button
                      class="px-2 py-1 text-xs font-mono font-medium rounded bg-zinc-800 hover:bg-emerald-600 text-zinc-300 hover:text-white transition-colors"
                      onclick={() => handleLoad(item.purchase, 1)}
                    >
                      S1
                    </button>
                    <button
                      class="px-2 py-1 text-xs font-mono font-medium rounded bg-zinc-800 hover:bg-emerald-600 text-zinc-300 hover:text-white transition-colors"
                      onclick={() => handleLoad(item.purchase, 2)}
                    >
                      S2
                    </button>
                    <button
                      class="px-2 py-1 text-xs font-mono font-medium rounded bg-zinc-800 hover:bg-emerald-600 text-zinc-300 hover:text-white transition-colors"
                      onclick={() => handleLoad(item.purchase, 3)}
                    >
                      S3
                    </button>
                  </div>
                </div>
              </div>
            {/each}
          </div>
        {/if}
      </div>
    </div>
  </div>
</div>
