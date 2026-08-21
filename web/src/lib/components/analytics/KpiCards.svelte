<script lang="ts">
  import { appState } from '../../state/appState.svelte';

  const slot = $derived(appState.activeSlot);
  const scenario = $derived(slot.scenario);
  const analysis = $derived(slot.analysis);
  const total = $derived(scenario?.total_statement);

  const payoffMonth = $derived(analysis?.payoff_month || 0);
  const payoffYears = $derived((payoffMonth / 12).toFixed(1));
  const effectiveMonthly = $derived(analysis?.effective_monthly_cost || 0);
  const wasteRatio = $derived((analysis?.waste_ratio || 0) * 100);
  const taxSavingsRatio = $derived((analysis?.tax_savings_ratio || 0) * 100);

  const totalPaid = $derived(total?.total_paid || 0);
  const totalInterest = $derived(total?.total_interest_paid || 0);
  const totalHolding = $derived(total?.total_holding_cost || 0);
  const totalTaxSavings = $derived(total?.total_tax_savings || 0);
  const totalCashInterest = $derived(total?.total_cash_interest || 0);
</script>

<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
  <!-- Card 1: Payoff Timeline -->
  <div class="p-4 rounded-2xl bg-zinc-900/60 border border-zinc-800/80 shadow-sm relative overflow-hidden group hover:border-zinc-700/80 transition-all">
    <div class="flex items-center justify-between">
      <span class="text-xs font-semibold text-zinc-400">⏱️ Payoff Timeline</span>
      <span class="text-[11px] font-mono px-2 py-0.5 rounded-full bg-zinc-800 text-zinc-300">
        Month {payoffMonth}
      </span>
    </div>
    <div class="mt-2.5 flex items-baseline gap-2">
      <span class="text-2xl lg:text-3xl font-bold font-mono tracking-tight text-white tabular-nums">
        {payoffYears}
      </span>
      <span class="text-xs text-zinc-400 font-medium">Years to Debt-Free</span>
    </div>
    <div class="mt-2 text-[11px] font-mono text-zinc-500">
      {#if payoffMonth < 360}
        <span class="text-emerald-400 font-semibold">⚡ {360 - payoffMonth} months faster</span> than 30-year base
      {:else}
        <span>Standard 30-year amortization schedule</span>
      {/if}
    </div>
  </div>

  <!-- Card 2: Effective Monthly Cost -->
  <div class="p-4 rounded-2xl bg-zinc-900/60 border border-zinc-800/80 shadow-sm relative overflow-hidden group hover:border-zinc-700/80 transition-all">
    <div class="flex items-center justify-between">
      <span class="text-xs font-semibold text-zinc-400">💳 Effective Monthly Outlay</span>
      <span class="text-[11px] font-mono px-2 py-0.5 rounded-full bg-indigo-950 text-indigo-300 border border-indigo-800/40">
        All-In
      </span>
    </div>
    <div class="mt-2.5 flex items-baseline gap-2">
      <span class="text-2xl lg:text-3xl font-bold font-mono tracking-tight text-indigo-300 tabular-nums">
        ${Math.round(effectiveMonthly).toLocaleString()}
      </span>
      <span class="text-xs text-zinc-400 font-medium">/ month</span>
    </div>
    <div class="mt-2 text-[11px] font-mono text-zinc-500">
      Debt service + holding costs net of tax & cash yield
    </div>
  </div>

  <!-- Card 3: Total Interest & Waste Ratio -->
  <div class="p-4 rounded-2xl bg-zinc-900/60 border border-zinc-800/80 shadow-sm relative overflow-hidden group hover:border-zinc-700/80 transition-all">
    <div class="flex items-center justify-between">
      <span class="text-xs font-semibold text-zinc-400">📉 Total Interest Paid</span>
      <span class="text-[11px] font-mono px-2 py-0.5 rounded-full bg-rose-950 text-rose-300 border border-rose-800/40">
        Waste: {wasteRatio.toFixed(1)}%
      </span>
    </div>
    <div class="mt-2.5 flex items-baseline gap-2">
      <span class="text-2xl lg:text-3xl font-bold font-mono tracking-tight text-rose-400 tabular-nums">
        ${Math.round(totalInterest).toLocaleString()}
      </span>
    </div>
    <div class="mt-2 text-[11px] font-mono text-zinc-500">
      Borrowing friction cost over lifetime
    </div>
  </div>

  <!-- Card 4: Tax Savings Realized -->
  <div class="p-4 rounded-2xl bg-zinc-900/60 border border-zinc-800/80 shadow-sm relative overflow-hidden group hover:border-zinc-700/80 transition-all">
    <div class="flex items-center justify-between">
      <span class="text-xs font-semibold text-zinc-400">🧾 Total Tax Savings</span>
      <span class="text-[11px] font-mono px-2 py-0.5 rounded-full bg-emerald-950 text-emerald-300 border border-emerald-800/40">
        Offset: {taxSavingsRatio.toFixed(1)}%
      </span>
    </div>
    <div class="mt-2.5 flex items-baseline gap-2">
      <span class="text-2xl lg:text-3xl font-bold font-mono tracking-tight text-emerald-400 tabular-nums">
        ${Math.round(totalTaxSavings).toLocaleString()}
      </span>
    </div>
    <div class="mt-2 text-[11px] font-mono text-zinc-500">
      IRS mortgage interest deduction (24% bracket)
    </div>
  </div>

  <!-- Card 5: Holding Costs -->
  <div class="p-4 rounded-2xl bg-zinc-900/60 border border-zinc-800/80 shadow-sm relative overflow-hidden group hover:border-zinc-700/80 transition-all">
    <div class="flex items-center justify-between">
      <span class="text-xs font-semibold text-zinc-400">🏚️ Lifetime Holding Costs</span>
      <span class="text-[11px] font-mono px-2 py-0.5 rounded-full bg-amber-950 text-amber-300 border border-amber-800/40">
        Property + HOA
      </span>
    </div>
    <div class="mt-2.5 flex items-baseline gap-2">
      <span class="text-2xl lg:text-3xl font-bold font-mono tracking-tight text-amber-400 tabular-nums">
        ${Math.round(totalHolding).toLocaleString()}
      </span>
    </div>
    <div class="mt-2 text-[11px] font-mono text-zinc-500">
      Tax, insurance & HOA with inflation growth
    </div>
  </div>

  <!-- Card 6: Total Gross Paid -->
  <div class="p-4 rounded-2xl bg-zinc-900/60 border border-zinc-800/80 shadow-sm relative overflow-hidden group hover:border-zinc-700/80 transition-all">
    <div class="flex items-center justify-between">
      <span class="text-xs font-semibold text-zinc-400">💸 Total Lifetime Cash Outlay</span>
      <span class="text-[11px] font-mono px-2 py-0.5 rounded-full bg-zinc-800 text-zinc-300">
        Grand Total
      </span>
    </div>
    <div class="mt-2.5 flex items-baseline gap-2">
      <span class="text-2xl lg:text-3xl font-bold font-mono tracking-tight text-white tabular-nums">
        ${Math.round(totalPaid).toLocaleString()}
      </span>
    </div>
    <div class="mt-2 text-[11px] font-mono text-zinc-500">
      Cash earned: +${Math.round(totalCashInterest).toLocaleString()}
    </div>
  </div>
</div>
