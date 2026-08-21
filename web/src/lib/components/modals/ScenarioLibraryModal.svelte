<script lang="ts">
  import { appState } from '../../state/appState.svelte';
  import { BUILTIN_SCENARIOS, type PresetScenario } from '../../data/builtinScenarios';
  import { getCustomScenarios, deleteCustomScenario, type SavedUserScenario } from '../../services/persistence';
  import type { Purchase, SlotId } from '../../state/types';

  let { onClose }: { onClose: () => void } = $props();

  let searchQuery = $state('');
  let activeTab = $state<'builtin' | 'custom'>('builtin');
  let customList = $state<SavedUserScenario[]>(getCustomScenarios());

  const filteredBuiltin = $derived.by(() => {
    const q = searchQuery.toLowerCase().trim();
    if (!q) return BUILTIN_SCENARIOS;
    return BUILTIN_SCENARIOS.filter(s =>
      s.name.toLowerCase().includes(q) ||
      s.description.toLowerCase().includes(q) ||
      s.filename.toLowerCase().includes(q)
    );
  });

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

  function handleDeleteCustom(id: string) {
    deleteCustomScenario(id);
    customList = getCustomScenarios();
  }
</script>

<div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/80 backdrop-blur-sm animate-in fade-in duration-200">
  <div class="relative w-full max-w-4xl max-h-[85vh] rounded-2xl bg-zinc-950 border border-zinc-800 shadow-2xl flex flex-col overflow-hidden">
    <!-- Header -->
    <div class="p-5 border-b border-zinc-800/80 flex items-center justify-between">
      <div class="flex items-center gap-3">
        <span class="text-2xl">📚</span>
        <div>
          <h2 class="text-base font-bold text-white">Scenario Library & CLI Presets</h2>
          <p class="text-xs text-zinc-400">Load pre-configured real estate & financing strategies directly into your workspace slots</p>
        </div>
      </div>
      <button
        class="w-8 h-8 rounded-lg bg-zinc-900 border border-zinc-800 hover:bg-zinc-800 text-zinc-400 hover:text-white flex items-center justify-center transition-colors text-base"
        onclick={onClose}
      >
        ✕
      </button>
    </div>

    <!-- Subheader Search & Tabs -->
    <div class="p-4 border-b border-zinc-800/60 bg-zinc-900/30 flex flex-col sm:flex-row items-center justify-between gap-3">
      <div class="flex items-center bg-zinc-900 p-1 rounded-xl border border-zinc-800 text-xs">
        <button
          class="px-3 py-1 rounded-lg transition-all {activeTab === 'builtin' ? 'bg-zinc-800 text-white font-medium shadow-sm' : 'text-zinc-400 hover:text-zinc-200'}"
          onclick={() => activeTab = 'builtin'}
        >
          CLI Standard Presets ({BUILTIN_SCENARIOS.length})
        </button>
        <button
          class="px-3 py-1 rounded-lg transition-all {activeTab === 'custom' ? 'bg-zinc-800 text-white font-medium shadow-sm' : 'text-zinc-400 hover:text-zinc-200'}"
          onclick={() => activeTab = 'custom'}
        >
          My Saved Scenarios ({customList.length})
        </button>
      </div>

      <input
        type="text"
        placeholder="Search scenarios by title, strategy, or tags..."
        class="w-full sm:w-72 px-3 py-1.5 rounded-xl bg-zinc-900 border border-zinc-800 text-xs text-zinc-200 focus:border-emerald-500 focus:outline-none placeholder:text-zinc-600"
        bind:value={searchQuery}
      />
    </div>

    <!-- Scenarios Grid -->
    <div class="flex-1 overflow-y-auto p-5 space-y-3">
      {#if activeTab === 'builtin'}
        <div class="grid grid-cols-1 md:grid-cols-2 gap-3.5">
          {#each filteredBuiltin as item}
            <div class="p-4 rounded-xl bg-zinc-900/50 border border-zinc-800/80 hover:border-zinc-700 transition-all flex flex-col justify-between space-y-3">
              <div>
                <div class="flex items-start justify-between gap-2">
                  <h4 class="text-sm font-semibold text-white">{item.name}</h4>
                  <span class="text-xs font-mono font-bold text-emerald-400 tabular-nums">
                    ${item.purchase.house.purchase_price.toLocaleString()}
                  </span>
                </div>
                <p class="text-xs text-zinc-400 mt-1 leading-relaxed">{item.description}</p>

                <!-- Tool Tags -->
                <div class="flex flex-wrap items-center gap-1.5 mt-2.5">
                  {#each item.purchase.tools as t}
                    {#if 'Cash' in t && t.Cash}
                      <span class="text-[10px] font-mono px-2 py-0.5 rounded bg-emerald-950/80 text-emerald-300 border border-emerald-800/50">
                        💵 ${(t.Cash.amount / 1000).toFixed(0)}k Cash
                      </span>
                    {:else if 'Mortgage' in t && t.Mortgage}
                      <span class="text-[10px] font-mono px-2 py-0.5 rounded bg-indigo-950/80 text-indigo-300 border border-indigo-800/50">
                        🏦 ${(t.Mortgage.amount / 1000).toFixed(0)}k @ {t.Mortgage.rate}% ({t.Mortgage.term}y)
                      </span>
                    {:else if 'Loc' in t && t.Loc}
                      <span class="text-[10px] font-mono px-2 py-0.5 rounded bg-amber-950/80 text-amber-300 border border-amber-800/50">
                        💳 ${(t.Loc.amount / 1000).toFixed(0)}k LOC @ {t.Loc.rate}%
                      </span>
                    {/if}
                  {/each}
                  {#if Object.keys(item.purchase.mortgage_repay || {}).length > 0 || Object.keys(item.purchase.loc_repay || {}).length > 0}
                    <span class="text-[10px] font-mono px-2 py-0.5 rounded bg-rose-950/80 text-rose-300 border border-rose-800/50">
                      ⚡ Prepayments
                    </span>
                  {/if}
                </div>
              </div>

              <!-- Load Actions -->
              <div class="pt-2 border-t border-zinc-800/60 flex items-center justify-between">
                <span class="text-[11px] font-mono text-zinc-500">{item.filename}</span>
                <div class="flex items-center gap-1">
                  <span class="text-[10px] font-mono text-zinc-500 mr-1">Load to:</span>
                  <button
                    class="px-2 py-1 text-xs font-mono font-medium rounded bg-zinc-800 hover:bg-emerald-600 text-zinc-300 hover:text-white transition-colors"
                    onclick={() => handleLoad(item.purchase, 1)}
                  >
                    Slot 1
                  </button>
                  <button
                    class="px-2 py-1 text-xs font-mono font-medium rounded bg-zinc-800 hover:bg-emerald-600 text-zinc-300 hover:text-white transition-colors"
                    onclick={() => handleLoad(item.purchase, 2)}
                  >
                    Slot 2
                  </button>
                  <button
                    class="px-2 py-1 text-xs font-mono font-medium rounded bg-zinc-800 hover:bg-emerald-600 text-zinc-300 hover:text-white transition-colors"
                    onclick={() => handleLoad(item.purchase, 3)}
                  >
                    Slot 3
                  </button>
                </div>
              </div>
            </div>
          {/each}
        </div>
      {:else}
        {#if filteredCustom.length === 0}
          <div class="p-12 text-center text-zinc-500 font-mono text-xs">
            No saved custom scenarios found. Use the Export button to save scenarios to your local browser storage.
          </div>
        {:else}
          <div class="grid grid-cols-1 md:grid-cols-2 gap-3.5">
            {#each filteredCustom as item}
              <div class="p-4 rounded-xl bg-zinc-900/50 border border-zinc-800/80 flex flex-col justify-between space-y-3">
                <div>
                  <div class="flex items-start justify-between gap-2">
                    <h4 class="text-sm font-semibold text-white">{item.name}</h4>
                    <button
                      class="text-zinc-500 hover:text-rose-400 text-xs transition-colors"
                      onclick={() => handleDeleteCustom(item.id)}
                      title="Delete saved scenario"
                    >
                      🗑️
                    </button>
                  </div>
                  <p class="text-[11px] text-zinc-500 font-mono mt-1">Saved on {new Date(item.savedAt).toLocaleDateString()}</p>
                </div>

                <div class="pt-2 border-t border-zinc-800/60 flex items-center justify-between">
                  <span class="text-xs font-mono text-emerald-400 font-bold">${item.purchase.house.purchase_price.toLocaleString()}</span>
                  <div class="flex items-center gap-1">
                    <button class="px-2 py-1 text-xs font-mono rounded bg-zinc-800 hover:bg-emerald-600 text-zinc-300 transition-colors" onclick={() => handleLoad(item.purchase, 1)}>S1</button>
                    <button class="px-2 py-1 text-xs font-mono rounded bg-zinc-800 hover:bg-emerald-600 text-zinc-300 transition-colors" onclick={() => handleLoad(item.purchase, 2)}>S2</button>
                    <button class="px-2 py-1 text-xs font-mono rounded bg-zinc-800 hover:bg-emerald-600 text-zinc-300 transition-colors" onclick={() => handleLoad(item.purchase, 3)}>S3</button>
                  </div>
                </div>
              </div>
            {/each}
          </div>
        {/if}
      {/if}
    </div>
  </div>
</div>
