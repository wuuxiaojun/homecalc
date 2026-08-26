<script lang="ts">
  import MonthlyStatementTable from './MonthlyStatementTable.svelte';
  import YearlyStatementTable from './YearlyStatementTable.svelte';

  let activeTab = $state<'monthly' | 'yearly'>('monthly');
</script>

<div class="space-y-4 sm:space-y-6">
  <!-- Statements Tab Switcher -->
  <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3 border-b border-zinc-800/70 pb-3">
    <div class="flex items-center gap-2">
      <span class="text-xl">📅</span>
      <div>
        <h2 class="text-sm sm:text-base font-bold text-white">Statement Ledger</h2>
        <p class="text-[11px] sm:text-xs text-zinc-400">Granular monthly breakdown schedule and yearly aggregated ledger</p>
      </div>
    </div>

    <div class="flex items-center bg-zinc-900 p-1 rounded-xl border border-zinc-800 self-start sm:self-auto">
      <button
        class="px-3 sm:px-3.5 py-1.5 text-xs font-medium rounded-lg transition-all flex items-center gap-1.5 {activeTab === 'monthly' ? 'bg-zinc-800 text-white shadow-sm font-semibold' : 'text-zinc-400 hover:text-zinc-200'}"
        onclick={() => activeTab = 'monthly'}
      >
        <span>📅</span>
        <span>Monthly</span>
      </button>
      <button
        class="px-3 sm:px-3.5 py-1.5 text-xs font-medium rounded-lg transition-all flex items-center gap-1.5 {activeTab === 'yearly' ? 'bg-zinc-800 text-white shadow-sm font-semibold' : 'text-zinc-400 hover:text-zinc-200'}"
        onclick={() => activeTab = 'yearly'}
      >
        <span>📈</span>
        <span>Yearly</span>
      </button>
    </div>
  </div>

  {#if activeTab === 'monthly'}
    <MonthlyStatementTable />
  {:else}
    <YearlyStatementTable />
  {/if}
</div>
