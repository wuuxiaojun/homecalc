<script lang="ts">
  import { appState } from '../../state/appState.svelte';
  import type { SlotId } from '../../state/types';
  import { formatCurrency, formatPercent, isValidNumber } from '../../utils/format';

  const baselineSlot = $derived(appState.getSlot(appState.comparisonBaselineId));
  const alternativeSlot = $derived(appState.getSlot(appState.comparisonAlternativeId));
  const comparison = $derived(appState.comparison);

  function setBaseline(id: SlotId) {
    appState.comparisonBaselineId = id;
  }

  function setAlternative(id: SlotId) {
    appState.comparisonAlternativeId = id;
  }

  function formatDeltaCurrency(val: number | null | undefined): { text: string; isPositive: boolean; isNeutral: boolean } {
    if (!isValidNumber(val)) return { text: 'N/A', isPositive: false, isNeutral: true };
    const rounded = Math.round(val);
    if (Math.abs(rounded) < 1) return { text: '$0', isPositive: false, isNeutral: true };
    if (rounded < 0) {
      return { text: `-$${Math.abs(rounded).toLocaleString()}`, isPositive: true, isNeutral: false }; // Saved money = positive outcome
    }
    return { text: `+$${rounded.toLocaleString()}`, isPositive: false, isNeutral: false }; // Paid more = cost
  }
</script>

<div class="space-y-6">
  <!-- Slot Pair Selector Header -->
  <div class="p-4 rounded-2xl bg-zinc-900/60 border border-zinc-800/80 flex flex-col md:flex-row items-center justify-between gap-4">
    <div class="flex items-center gap-2">
      <span class="text-xl">⚖️</span>
      <div>
        <h2 class="text-base font-bold text-white">Scenario Differential Analysis</h2>
        <p class="text-xs text-zinc-400">Comparing financial trajectories, cost deltas, and strategy return rate</p>
      </div>
    </div>

    <!-- Selectors -->
    <div class="flex items-center gap-3">
      <!-- Slot A Selector -->
      <div class="flex items-center gap-1.5 bg-zinc-950 px-2.5 py-1.5 rounded-xl border border-zinc-800 text-xs">
        <span class="text-zinc-500 font-mono">A:</span>
        <select
          class="bg-transparent text-zinc-200 font-semibold focus:outline-none cursor-pointer"
          value={appState.comparisonBaselineId}
          onchange={(e) => setBaseline(parseInt(e.currentTarget.value, 10) as SlotId)}
        >
          <option value={1}>Slot 1: {appState.slot1.purchase.name}</option>
          <option value={2}>Slot 2: {appState.slot2.purchase.name}</option>
          <option value={3}>Slot 3: {appState.slot3.purchase.name}</option>
        </select>
      </div>

      <span class="text-zinc-500 font-mono text-xs">vs</span>

      <!-- Slot B Selector -->
      <div class="flex items-center gap-1.5 bg-zinc-950 px-2.5 py-1.5 rounded-xl border border-zinc-800 text-xs">
        <span class="text-zinc-500 font-mono">B:</span>
        <select
          class="bg-transparent text-indigo-300 font-semibold focus:outline-none cursor-pointer"
          value={appState.comparisonAlternativeId}
          onchange={(e) => setAlternative(parseInt(e.currentTarget.value, 10) as SlotId)}
        >
          <option value={1}>Slot 1: {appState.slot1.purchase.name}</option>
          <option value={2}>Slot 2: {appState.slot2.purchase.name}</option>
          <option value={3}>Slot 3: {appState.slot3.purchase.name}</option>
        </select>
      </div>
    </div>
  </div>

  {#if comparison}
    <!-- Top Comparison Highlight KPI Cards -->
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
      <!-- 1. Gross Outlay (Gross Lifetime Total) -->
      <div class="p-4 rounded-2xl bg-zinc-900/60 border border-zinc-800/80">
        <div class="text-xs font-semibold text-zinc-400">💳 Gross Outlay</div>
        <div class="mt-2 text-2xl font-bold font-mono tracking-tight tabular-nums {formatDeltaCurrency(comparison.delta_gross_paid).isPositive ? 'text-emerald-400' : formatDeltaCurrency(comparison.delta_gross_paid).isNeutral ? 'text-zinc-300' : 'text-rose-400'}">
          {formatDeltaCurrency(comparison.delta_gross_paid).text}
        </div>
        <div class="mt-1 text-[11px] font-mono text-zinc-500">
          {formatDeltaCurrency(comparison.delta_gross_paid).isPositive ? 'Lifetime gross outlay saved' : 'Additional gross outlay'}
        </div>
      </div>

      <!-- 2. Interest Paid -->
      <div class="p-4 rounded-2xl bg-zinc-900/60 border border-zinc-800/80">
        <div class="text-xs font-semibold text-zinc-400">📉 Interest Paid</div>
        <div class="mt-2 text-2xl font-bold font-mono tracking-tight tabular-nums {formatDeltaCurrency(comparison.delta_interest_paid).isPositive ? 'text-emerald-400' : formatDeltaCurrency(comparison.delta_interest_paid).isNeutral ? 'text-zinc-300' : 'text-rose-400'}">
          {formatDeltaCurrency(comparison.delta_interest_paid).text}
        </div>
        <div class="mt-1 text-[11px] font-mono text-zinc-500">
          {formatDeltaCurrency(comparison.delta_interest_paid).isPositive ? 'Lifetime interest saved' : 'Additional interest cost'}
        </div>
      </div>

      <!-- 3. Present Value -->
      <div class="p-4 rounded-2xl bg-zinc-900/60 border border-zinc-800/80">
        <div class="text-xs font-semibold text-zinc-400">📊 Present Value</div>
        <div class="mt-2 text-2xl font-bold font-mono tracking-tight tabular-nums {formatDeltaCurrency(comparison.delta_pv).isPositive ? 'text-emerald-400' : formatDeltaCurrency(comparison.delta_pv).isNeutral ? 'text-zinc-300' : 'text-rose-400'}">
          {formatDeltaCurrency(comparison.delta_pv).text}
        </div>
        <div class="mt-1 text-[11px] font-mono text-zinc-500">
          {formatDeltaCurrency(comparison.delta_pv).isPositive ? 'Lifetime PV saved (@ 6.5%)' : 'Additional PV cost (@ 6.5%)'}
        </div>
      </div>

      <!-- 4. Internal Rate of Return -->
      <div class="p-4 rounded-2xl bg-zinc-900/60 border border-zinc-800/80">
        <div class="text-xs font-semibold text-zinc-400">📈 Internal Rate of Return</div>
        <div class="mt-2 text-2xl font-bold font-mono tracking-tight text-indigo-400 tabular-nums">
          {#if isValidNumber(comparison.irr)}
            {formatPercent(comparison.irr * 100, 2)}
          {:else}
            <span class="text-zinc-500 text-lg">N/A</span>
          {/if}
        </div>
        <div class="mt-1 text-[11px] font-mono text-zinc-500">
          Internal Rate of Return on accelerated equity
        </div>
      </div>
    </div>

    <!-- 4-Column Side-by-Side Comparison Table -->
    <div class="rounded-2xl bg-zinc-900/60 border border-zinc-800/80 overflow-hidden shadow-sm">
      <div class="p-4 border-b border-zinc-800/60 bg-zinc-950/40 flex items-center justify-between">
        <h3 class="text-sm font-semibold text-zinc-200">📊 Metric Differential Table</h3>
        <span class="text-xs font-mono text-zinc-500">Delta = B - A</span>
      </div>

      <div class="overflow-x-auto">
        <table class="w-full text-xs font-mono">
          <thead>
            <tr class="border-b border-zinc-800/80 bg-zinc-950/70 text-zinc-400 text-left">
              <th class="py-3 px-4 font-semibold">Metric Category</th>
              <th class="py-3 px-4 font-semibold text-right">A: {baselineSlot.name}</th>
              <th class="py-3 px-4 font-semibold text-right">B: {alternativeSlot.name}</th>
              <th class="py-3 px-4 font-semibold text-right">Delta (B - A)</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-zinc-800/40">
            <!-- Timeline -->
            <tr class="hover:bg-zinc-800/20 transition-colors">
              <td class="py-2.5 px-4 font-sans text-zinc-300 font-medium">⏱️ Payoff Timeline</td>
              <td class="py-2.5 px-4 text-right text-zinc-200 tabular-nums">
                {isValidNumber(comparison.baseline_payoff_month) ? `Month ${comparison.baseline_payoff_month} (${(comparison.baseline_payoff_month / 12).toFixed(1)}y)` : 'N/A'}
              </td>
              <td class="py-2.5 px-4 text-right text-zinc-200 tabular-nums">
                {isValidNumber(comparison.alternative_payoff_month) ? `Month ${comparison.alternative_payoff_month} (${(comparison.alternative_payoff_month / 12).toFixed(1)}y)` : 'N/A'}
              </td>
              <td class="py-2.5 px-4 text-right font-bold tabular-nums {isValidNumber(comparison.months_saved) && comparison.months_saved > 0 ? 'text-emerald-400' : isValidNumber(comparison.months_saved) && comparison.months_saved < 0 ? 'text-rose-400' : 'text-zinc-400'}">
                {#if isValidNumber(comparison.months_saved) && comparison.months_saved > 0}
                  -{comparison.months_saved} Months Saved
                {:else if isValidNumber(comparison.months_saved)}
                  {comparison.months_saved} Months
                {:else}
                  N/A
                {/if}
              </td>
            </tr>

            <!-- Extra Principal -->
            <tr class="hover:bg-zinc-800/20 transition-colors">
              <td class="py-2.5 px-4 font-sans text-zinc-300 font-medium">⚡ Extra Principal Prepayments</td>
              <td class="py-2.5 px-4 text-right text-zinc-200 tabular-nums">{formatCurrency(comparison.baseline_extra_payment)}</td>
              <td class="py-2.5 px-4 text-right text-zinc-200 tabular-nums">{formatCurrency(comparison.alternative_extra_payment)}</td>
              <td class="py-2.5 px-4 text-right font-bold text-zinc-300 tabular-nums">{formatDeltaCurrency(comparison.delta_extra_payment).text}</td>
            </tr>

            <!-- Total Interest Paid -->
            <tr class="hover:bg-zinc-800/20 transition-colors">
              <td class="py-2.5 px-4 font-sans text-zinc-300 font-medium">📉 Total Interest Paid</td>
              <td class="py-2.5 px-4 text-right text-rose-300 tabular-nums">{formatCurrency(comparison.baseline_interest_paid)}</td>
              <td class="py-2.5 px-4 text-right text-rose-300 tabular-nums">{formatCurrency(comparison.alternative_interest_paid)}</td>
              <td class="py-2.5 px-4 text-right font-bold tabular-nums {formatDeltaCurrency(comparison.delta_interest_paid).isPositive ? 'text-emerald-400' : formatDeltaCurrency(comparison.delta_interest_paid).isNeutral ? 'text-zinc-400' : 'text-rose-400'}">
                {formatDeltaCurrency(comparison.delta_interest_paid).text}
              </td>
            </tr>

            <!-- Cash Yield -->
            <tr class="hover:bg-zinc-800/20 transition-colors">
              <td class="py-2.5 px-4 font-sans text-zinc-300 font-medium">💵 Total Cash Yield Earned</td>
              <td class="py-2.5 px-4 text-right text-zinc-200 tabular-nums">{formatCurrency(comparison.baseline_cash_interest)}</td>
              <td class="py-2.5 px-4 text-right text-zinc-200 tabular-nums">{formatCurrency(comparison.alternative_cash_interest)}</td>
              <td class="py-2.5 px-4 text-right font-bold text-zinc-300 tabular-nums">{formatDeltaCurrency(comparison.delta_cash_interest).text}</td>
            </tr>

            <!-- Tax Savings -->
            <tr class="hover:bg-zinc-800/20 transition-colors">
              <td class="py-2.5 px-4 font-sans text-zinc-300 font-medium">🧾 Total Tax Savings Realized</td>
              <td class="py-2.5 px-4 text-right text-emerald-400 tabular-nums">{formatCurrency(comparison.baseline_tax_savings)}</td>
              <td class="py-2.5 px-4 text-right text-emerald-400 tabular-nums">{formatCurrency(comparison.alternative_tax_savings)}</td>
              <td class="py-2.5 px-4 text-right font-bold text-zinc-300 tabular-nums">{formatDeltaCurrency(comparison.delta_tax_savings).text}</td>
            </tr>

            <!-- Lifetime Outlay -->
            <tr class="hover:bg-zinc-800/20 transition-colors">
              <td class="py-2.5 px-4 font-sans text-zinc-300 font-medium">💳 Total Lifetime Gross Outlay</td>
              <td class="py-2.5 px-4 text-right text-zinc-200 tabular-nums">{formatCurrency(comparison.baseline_gross_paid)}</td>
              <td class="py-2.5 px-4 text-right text-zinc-200 tabular-nums">{formatCurrency(comparison.alternative_gross_paid)}</td>
              <td class="py-2.5 px-4 text-right font-bold tabular-nums {formatDeltaCurrency(comparison.delta_gross_paid).isPositive ? 'text-emerald-400' : formatDeltaCurrency(comparison.delta_gross_paid).isNeutral ? 'text-zinc-400' : 'text-rose-400'}">
                {formatDeltaCurrency(comparison.delta_gross_paid).text}
              </td>
            </tr>

            <!-- Present Value -->
            <tr class="hover:bg-zinc-800/20 transition-colors">
              <td class="py-2.5 px-4 font-sans text-zinc-300 font-medium">📊 Present Value Outflow (PV @ 6.5%)</td>
              <td class="py-2.5 px-4 text-right text-indigo-300 tabular-nums">{formatCurrency(comparison.baseline_pv)}</td>
              <td class="py-2.5 px-4 text-right text-indigo-300 tabular-nums">{formatCurrency(comparison.alternative_pv)}</td>
              <td class="py-2.5 px-4 text-right font-bold tabular-nums {formatDeltaCurrency(comparison.delta_pv).isPositive ? 'text-emerald-400' : formatDeltaCurrency(comparison.delta_pv).isNeutral ? 'text-zinc-400' : 'text-rose-400'}">
                {formatDeltaCurrency(comparison.delta_pv).text}
              </td>
            </tr>


            <!-- Strategy IRR -->
            <tr class="hover:bg-zinc-800/20 transition-colors">
              <td class="py-2.5 px-4 font-sans text-zinc-300 font-medium">📈 Strategy Return (IRR)</td>
              <td class="py-2.5 px-4 text-right text-zinc-500">N/A</td>
              <td class="py-2.5 px-4 text-right text-zinc-500">N/A</td>
              <td class="py-2.5 px-4 text-right font-bold text-indigo-400 tabular-nums">
                {isValidNumber(comparison.irr) ? formatPercent(comparison.irr * 100, 2) : 'N/A'}
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  {:else}
    <div class="p-12 text-center text-zinc-500 font-mono text-sm border border-zinc-800 rounded-2xl">
      Calculating comparison differential...
    </div>
  {/if}
</div>
