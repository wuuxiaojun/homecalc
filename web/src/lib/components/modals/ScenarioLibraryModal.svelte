<script lang="ts">
  import { appState } from '../../state/appState.svelte';
  import { getCustomScenarios, deleteCustomScenario, type SavedUserScenario } from '../../services/persistence';
  import { exportPurchaseToJson } from '../../services/importExport';
  import type { Purchase, SlotId } from '../../state/types';

  let { onClose, onOpenImport }: { onClose: () => void; onOpenImport?: () => void } = $props();

  let searchQuery = $state('');
  let customList = $state<SavedUserScenario[]>(getCustomScenarios());

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
        <span class="text-2xl">📁</span>
        <div>
          <h2 class="text-base font-bold text-white">My Saved Scenarios ({customList.length})</h2>
          <p class="text-xs text-zinc-400">Manage and load your saved mortgage & property scenarios</p>
        </div>
      </div>
      <button
        class="w-8 h-8 rounded-lg bg-zinc-900 border border-zinc-800 hover:bg-zinc-800 text-zinc-400 hover:text-white flex items-center justify-center transition-colors text-base"
        onclick={onClose}
      >
        ✕
      </button>
    </div>

    <!-- Subheader Search -->
    <div class="p-4 border-b border-zinc-800/60 bg-zinc-900/30 flex items-center justify-between gap-3">
      <input
        type="text"
        placeholder="Filter your saved scenarios by name..."
        class="w-full px-3 py-1.5 rounded-xl bg-zinc-900 border border-zinc-800 text-xs text-zinc-200 focus:border-emerald-500 focus:outline-none placeholder:text-zinc-600"
        bind:value={searchQuery}
      />
    </div>

    <!-- Scenarios Grid -->
    <div class="flex-1 overflow-y-auto p-5 space-y-3">
      {#if customList.length === 0}
        <div class="p-12 text-center space-y-4">
          <div class="w-14 h-14 rounded-2xl bg-zinc-900 border border-zinc-800 flex items-center justify-center text-2xl mx-auto">
            📂
          </div>
          <div>
            <h3 class="text-sm font-semibold text-white">No Saved Scenarios Found</h3>
            <p class="text-xs text-zinc-400 max-w-sm mx-auto mt-1 leading-relaxed">
              You can save any active scenario using the <span class="text-zinc-200 font-mono font-semibold">"Save"</span> button in the parameter suite, or import a JSON file.
            </p>
          </div>
          <div class="flex items-center justify-center gap-2 pt-2">
            {#if onOpenImport}
              <button
                class="px-4 py-2 rounded-xl bg-emerald-600 hover:bg-emerald-500 text-white text-xs font-semibold shadow-sm transition-colors flex items-center gap-2"
                onclick={() => { onClose(); onOpenImport?.(); }}
              >
                <span>📥</span>
                <span>Import JSON Scenario</span>
              </button>
            {/if}
          </div>
        </div>
      {:else if filteredCustom.length === 0}
        <div class="p-10 text-center text-zinc-500 text-xs font-mono">
          No scenarios match your search query "{searchQuery}".
        </div>
      {:else}
        <div class="grid grid-cols-1 md:grid-cols-2 gap-3.5">
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
