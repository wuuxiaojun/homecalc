<script lang="ts">
  import { appState } from '../../state/appState.svelte';
  import MonthlyStatementTable from './MonthlyStatementTable.svelte';
  import YearlyStatementTable from './YearlyStatementTable.svelte';

  let activeTab = $state<'monthly' | 'yearly'>('monthly');
  const slot = $derived(appState.activeSlot);
</script>

<div class="space-y-6">
  {#if !slot.purchase || !slot.scenario}
    <div class="p-12 rounded-2xl bg-zinc-900/40 border border-zinc-800/80 text-center space-y-3">
      <div class="text-3xl">📅</div>
      <h3 class="text-sm font-bold text-white">No Statements Ledger in Slot {slot.id}</h3>
      <p class="text-xs text-zinc-400 max-w-sm mx-auto">
        Create or load a scenario to generate complete 360-month schedules and 30-year ledger statements.
      </p>
      <button
        class="mt-2 py-2 px-4 rounded-xl bg-emerald-600 hover:bg-emerald-500 text-white text-xs font-semibold transition-colors"
        onclick={() => appState.createScenarioInSlot(slot.id)}
      >
        ✨ Create Scenario in Slot {slot.id}
      </button>
    </div>
  {:else}
    <!-- Statements Tab Switcher -->
    <div class="flex items-center justify-between border-b border-zinc-800/70 pb-3">
      <div class="flex items-center gap-2">
        <span class="text-xl">📅</span>
        <div>
          <h2 class="text-base font-bold text-white">Amortization & Financial Statement Ledger</h2>
          <p class="text-xs text-zinc-400">Granular monthly breakdown schedule and yearly aggregated ledger</p>
        </div>
      </div>

      <div class="flex items-center bg-zinc-900 p-1 rounded-xl border border-zinc-800">
        <button
          class="px-3.5 py-1.5 text-xs font-medium rounded-lg transition-all flex items-center gap-1.5 {activeTab === 'monthly' ? 'bg-zinc-800 text-white shadow-sm font-semibold' : 'text-zinc-400 hover:text-zinc-200'}"
          onclick={() => activeTab = 'monthly'}
        >
          <span>📅</span>
          <span>Monthly Schedule</span>
        </button>
        <button
          class="px-3.5 py-1.5 text-xs font-medium rounded-lg transition-all flex items-center gap-1.5 {activeTab === 'yearly' ? 'bg-zinc-800 text-white shadow-sm font-semibold' : 'text-zinc-400 hover:text-zinc-200'}"
          onclick={() => activeTab = 'yearly'}
        >
          <span>📈</span>
          <span>Yearly Statement</span>
        </button>
      </div>
    </div>

    {#if activeTab === 'monthly'}
      <MonthlyStatementTable />
    {:else}
      <YearlyStatementTable />
    {/if}
  {/if}
</div>
