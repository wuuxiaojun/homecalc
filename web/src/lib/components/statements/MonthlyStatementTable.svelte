<script lang="ts">
  import { appState } from '../../state/appState.svelte';
  import type { MonthlyStatementRow } from '../../state/types';

  let filterMode = $state<'all' | 'extras' | 'milestones'>('all');
  let searchQuery = $state<string>('');
  let page = $state<number>(1);
  let pageSize = $state<number>(36); // 3 years per page

  const slot = $derived(appState.activeSlot);
  const monthlyRows = $derived(slot.scenario?.monthly_statement || []);

  const filteredRows = $derived.by(() => {
    let list = monthlyRows;

    if (filterMode === 'extras') {
      list = list.filter(r => r.total_extra_payment > 0 || r.month === 0);
    } else if (filterMode === 'milestones') {
      list = list.filter(r => r.month === 0 || r.month % 12 === 0 || r.month === monthlyRows[monthlyRows.length - 1]?.month);
    }

    if (searchQuery.trim()) {
      const q = parseInt(searchQuery.trim(), 10);
      if (!isNaN(q)) {
        list = list.filter(r => r.month === q);
      }
    }

    return list;
  });

  const totalPages = $derived(Math.max(1, Math.ceil(filteredRows.length / pageSize)));
  const paginatedRows = $derived(
    pageSize === -1
      ? filteredRows
      : filteredRows.slice((page - 1) * pageSize, page * pageSize)
  );

  function formatCurrency(val: number): string {
    return '$' + Math.round(val).toLocaleString();
  }

  function exportCsv() {
    if (monthlyRows.length === 0) return;
    const headers = [
      'Month',
      'Cash Yield',
      'Mortgage PMT',
      'Mortgage Extra',
      'LOC PMT',
      'LOC Extra',
      'Holding Cost',
      'Total Paid',
      'Remaining Balance'
    ];

    const rows = monthlyRows.map(r => [
      r.month,
      r.cash?.cash_interest || 0,
      r.mortgage ? (r.mortgage.principal_paid + r.mortgage.interest_paid) : 0,
      r.mortgage?.extra_payment || 0,
      r.loc?.monthly_payment || 0,
      r.loc?.extra_payment || 0,
      r.total_holding_cost,
      r.total_paid,
      r.total_remaining_balance
    ]);

    const csvContent = [headers.join(','), ...rows.map(row => row.join(','))].join('\n');
    const blob = new Blob([csvContent], { type: 'text/csv;charset=utf-8;' });
    const url = URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.setAttribute('href', url);
    link.setAttribute('download', `homecalc_monthly_schedule_${slot.name.replace(/\s+/g, '_')}.csv`);
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
  }
</script>

<div class="space-y-4">
  <!-- Controls Bar -->
  <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3 p-4 rounded-xl bg-zinc-900/60 border border-zinc-800/80">
    <div class="flex items-center gap-2">
      <!-- Filter Mode -->
      <div class="flex items-center bg-zinc-950 p-1 rounded-lg border border-zinc-800 text-xs">
        <button
          class="px-2.5 py-1 rounded transition-colors {filterMode === 'all' ? 'bg-zinc-800 text-white font-medium' : 'text-zinc-400 hover:text-zinc-200'}"
          onclick={() => { filterMode = 'all'; page = 1; }}
        >
          All ({monthlyRows.length})
        </button>
        <button
          class="px-2.5 py-1 rounded transition-colors {filterMode === 'extras' ? 'bg-zinc-800 text-white font-medium' : 'text-zinc-400 hover:text-zinc-200'}"
          onclick={() => { filterMode = 'extras'; page = 1; }}
        >
          ⚡ Extra Only
        </button>
        <button
          class="px-2.5 py-1 rounded transition-colors {filterMode === 'milestones' ? 'bg-zinc-800 text-white font-medium' : 'text-zinc-400 hover:text-zinc-200'}"
          onclick={() => { filterMode = 'milestones'; page = 1; }}
        >
          Yearly Milestones
        </button>
      </div>

      <!-- Month Search -->
      <input
        type="number"
        placeholder="Month #"
        min="0"
        max="360"
        class="w-24 px-2.5 py-1 rounded-lg bg-zinc-950 border border-zinc-800 text-xs font-mono text-zinc-200 focus:border-emerald-500 focus:outline-none placeholder:text-zinc-600"
        bind:value={searchQuery}
      />
    </div>

    <!-- Right Controls: Page Size & CSV Export -->
    <div class="flex items-center gap-2 text-xs">
      <select
        class="px-2.5 py-1 rounded-lg bg-zinc-950 border border-zinc-800 text-xs text-zinc-300 focus:outline-none font-mono"
        bind:value={pageSize}
        onchange={() => page = 1}
      >
        <option value={36}>36 rows/page</option>
        <option value={60}>60 rows/page</option>
        <option value={120}>120 rows/page</option>
        <option value={-1}>Show All</option>
      </select>

      <button
        class="px-3 py-1 rounded-lg bg-zinc-800 hover:bg-zinc-700 text-zinc-200 font-medium transition-colors flex items-center gap-1.5"
        onclick={exportCsv}
      >
        <span>📥</span>
        <span>CSV</span>
      </button>
    </div>
  </div>

  <!-- Tabular Schedule -->
  <div class="rounded-xl border border-zinc-800/80 bg-zinc-900/40 overflow-hidden shadow-sm">
    <div class="overflow-x-auto">
      <table class="w-full text-xs font-mono">
        <thead>
          <tr class="border-b border-zinc-800/80 bg-zinc-950/80 text-zinc-400 text-left">
            <th class="py-2.5 px-3 font-semibold text-center w-16">Month</th>
            <th class="py-2.5 px-3 font-semibold text-right">Cash Yield</th>
            <th class="py-2.5 px-3 font-semibold text-right">Mortgage PMT</th>
            <th class="py-2.5 px-3 font-semibold text-right text-emerald-400">Mort Extra</th>
            <th class="py-2.5 px-3 font-semibold text-right">LOC PMT</th>
            <th class="py-2.5 px-3 font-semibold text-right text-amber-400">LOC Extra</th>
            <th class="py-2.5 px-3 font-semibold text-right">Holding Cost</th>
            <th class="py-2.5 px-3 font-semibold text-right text-white">Total Paid</th>
            <th class="py-2.5 px-3 font-semibold text-right text-emerald-300">Remaining Balance</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-zinc-800/40 tabular-nums">
          {#each paginatedRows as row}
            {@const hasExtra = row.total_extra_payment > 0}
            <tr class="hover:bg-zinc-800/30 transition-colors {hasExtra ? 'bg-emerald-950/20' : row.month % 12 === 0 ? 'bg-zinc-950/40 font-semibold' : ''}">
              <td class="py-2 px-3 text-center text-zinc-400 font-bold">
                {row.month === 0 ? 'M0' : `M${row.month}`}
              </td>
              <td class="py-2 px-3 text-right text-cyan-400/90">
                {row.cash ? formatCurrency(row.cash.cash_interest) : '$0'}
              </td>
              <td class="py-2 px-3 text-right text-indigo-300">
                {row.mortgage ? formatCurrency(row.mortgage.principal_paid + row.mortgage.interest_paid) : '$0'}
              </td>
              <td class="py-2 px-3 text-right font-bold text-emerald-400">
                {row.mortgage && row.mortgage.extra_payment > 0 ? `+${formatCurrency(row.mortgage.extra_payment)}` : '-'}
              </td>
              <td class="py-2 px-3 text-right text-amber-300">
                {row.loc ? formatCurrency(row.loc.monthly_payment) : '$0'}
              </td>
              <td class="py-2 px-3 text-right font-bold text-amber-400">
                {row.loc && row.loc.extra_payment > 0 ? `+${formatCurrency(row.loc.extra_payment)}` : '-'}
              </td>
              <td class="py-2 px-3 text-right text-zinc-400">
                {formatCurrency(row.total_holding_cost)}
              </td>
              <td class="py-2 px-3 text-right font-bold text-white">
                {formatCurrency(row.total_paid)}
              </td>
              <td class="py-2 px-3 text-right font-bold text-emerald-400">
                {formatCurrency(row.total_remaining_balance)}
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>

    <!-- Pagination Footer -->
    {#if pageSize !== -1 && totalPages > 1}
      <div class="p-3 bg-zinc-950/60 border-t border-zinc-800/80 flex items-center justify-between text-xs font-mono">
        <span class="text-zinc-500">
          Showing {(page - 1) * pageSize + 1} - {Math.min(page * pageSize, filteredRows.length)} of {filteredRows.length} months
        </span>
        <div class="flex items-center gap-1.5">
          <button
            class="px-2.5 py-1 rounded bg-zinc-900 border border-zinc-800 hover:bg-zinc-800 disabled:opacity-30 transition-colors"
            disabled={page <= 1}
            onclick={() => page -= 1}
          >
            ◀ Prev
          </button>
          <span class="px-2 text-zinc-300">Page {page} of {totalPages}</span>
          <button
            class="px-2.5 py-1 rounded bg-zinc-900 border border-zinc-800 hover:bg-zinc-800 disabled:opacity-30 transition-colors"
            disabled={page >= totalPages}
            onclick={() => page += 1}
          >
            Next ▶
          </button>
        </div>
      </div>
    {/if}
  </div>
</div>
