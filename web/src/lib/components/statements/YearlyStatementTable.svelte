<script lang="ts">
  import { appState } from '../../state/appState.svelte';
  import { exportCsvFile } from '../../services/importExport';

  const slot = $derived(appState.activeSlot);
  const yearlyRows = $derived(slot.scenario?.yearly_statement || []);
  const totalStatement = $derived(slot.scenario?.total_statement);

  function formatCurrency(val: number): string {
    return '$' + Math.round(val).toLocaleString();
  }

  function exportCsv() {
    if (yearlyRows.length === 0) return;
    const headers = [
      'Year',
      'Cash Yield',
      'Interest Paid',
      'Debt Paid',
      'Extra Principal Paid',
      'Holding Cost',
      'Tax Savings',
      'Total Paid',
      'Ending Remaining Balance'
    ];

    const rows = yearlyRows.map(y => [
      y.year,
      y.annual_cash_interest,
      y.annual_interest_paid,
      y.annual_debt_paid,
      y.annual_extra_payment,
      y.annual_holding_cost,
      y.annual_tax_savings,
      y.annual_paid,
      y.ending_remaining_balance
    ]);

    const csvContent = [headers.join(','), ...rows.map(r => r.join(','))].join('\n');
    const filename = `homecalc_yearly_statement_${slot.name.replace(/\s+/g, '_')}.csv`;
    exportCsvFile(csvContent, filename);
  }
</script>

<div class="space-y-4">
  <!-- Controls Bar -->
  <div class="flex items-center justify-between p-3.5 sm:p-4 rounded-xl bg-zinc-900/60 border border-zinc-800/80">
    <div class="text-xs font-semibold text-zinc-300 flex items-center gap-2">
      <span>📈 Yearly Statement Summary</span>
      <span class="text-[11px] font-mono text-zinc-500 hidden sm:inline">({yearlyRows.length} Annual Periods)</span>
    </div>
    <button
      class="px-3 py-1 text-xs rounded-lg bg-zinc-800 hover:bg-zinc-700 text-zinc-200 font-medium transition-colors flex items-center gap-1.5"
      onclick={exportCsv}
    >
      <span>📥</span>
      <span>CSV Export</span>
    </button>
  </div>

  <!-- Table -->
  <div class="rounded-xl border border-zinc-800/80 bg-zinc-900/40 overflow-hidden shadow-sm">
    <div class="overflow-x-auto custom-scrollbar" style="-webkit-overflow-scrolling: touch;">
      <table class="w-full text-xs font-mono min-w-[760px]">
        <thead>
          <tr class="border-b border-zinc-800/80 bg-zinc-950/80 text-zinc-400 text-left whitespace-nowrap">
            <th class="py-3 px-3 font-semibold text-center w-14">Year</th>
            <th class="py-3 px-3 font-semibold text-right">Cash Yield</th>
            <th class="py-3 px-3 font-semibold text-right text-rose-300">Interest Paid</th>
            <th class="py-3 px-3 font-semibold text-right">Debt Service</th>
            <th class="py-3 px-3 font-semibold text-right text-emerald-400">Extra Prepay</th>
            <th class="py-3 px-3 font-semibold text-right text-amber-300">Holding Cost</th>
            <th class="py-3 px-3 font-semibold text-right text-emerald-400">Tax Savings</th>
            <th class="py-3 px-3 font-semibold text-right text-white">Annual Paid</th>
            <th class="py-3 px-3 font-semibold text-right text-emerald-300">Ending Balance</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-zinc-800/40 tabular-nums whitespace-nowrap">
          {#each yearlyRows as row}
            <tr class="hover:bg-zinc-800/30 transition-colors {row.annual_extra_payment > 0 ? 'bg-emerald-950/10' : ''}">
              <td class="py-2.5 px-3 text-center text-zinc-300 font-bold">
                Y{row.year}
              </td>
              <td class="py-2.5 px-3 text-right text-cyan-400">
                {formatCurrency(row.annual_cash_interest)}
              </td>
              <td class="py-2.5 px-3 text-right text-rose-400">
                {formatCurrency(row.annual_interest_paid)}
              </td>
              <td class="py-2.5 px-3 text-right text-indigo-300">
                {formatCurrency(row.annual_debt_paid)}
              </td>
              <td class="py-2.5 px-3 text-right font-bold text-emerald-400">
                {row.annual_extra_payment > 0 ? `+${formatCurrency(row.annual_extra_payment)}` : '-'}
              </td>
              <td class="py-2.5 px-3 text-right text-amber-400">
                {formatCurrency(row.annual_holding_cost)}
              </td>
              <td class="py-2.5 px-3 text-right text-emerald-400">
                {row.annual_tax_savings > 0 ? `-${formatCurrency(row.annual_tax_savings)}` : '$0'}
              </td>
              <td class="py-2.5 px-3 text-right font-bold text-white">
                {formatCurrency(row.annual_paid)}
              </td>
              <td class="py-2.5 px-3 text-right font-bold text-emerald-400">
                {formatCurrency(row.ending_remaining_balance)}
              </td>
            </tr>
          {/each}
        </tbody>

        <!-- Lifetime Totals Footer -->
        {#if totalStatement}
          <tfoot>
            <tr class="border-t-2 border-zinc-700 bg-zinc-950 font-bold text-white tabular-nums whitespace-nowrap">
              <td class="py-3 px-3 text-center text-emerald-400 font-mono">TOTAL</td>
              <td class="py-3 px-3 text-right text-cyan-400">+{formatCurrency(totalStatement.total_cash_interest)}</td>
              <td class="py-3 px-3 text-right text-rose-400">{formatCurrency(totalStatement.total_interest_paid)}</td>
              <td class="py-3 px-3 text-right text-indigo-300">-</td>
              <td class="py-3 px-3 text-right text-emerald-400">-</td>
              <td class="py-3 px-3 text-right text-amber-400">{formatCurrency(totalStatement.total_holding_cost)}</td>
              <td class="py-3 px-3 text-right text-emerald-400">-{formatCurrency(totalStatement.total_tax_savings)}</td>
              <td class="py-3 px-3 text-right text-white text-sm">{formatCurrency(totalStatement.total_paid)}</td>
              <td class="py-3 px-3 text-right text-emerald-400">$0</td>
            </tr>
          </tfoot>
        {/if}
      </table>
    </div>
  </div>
</div>
