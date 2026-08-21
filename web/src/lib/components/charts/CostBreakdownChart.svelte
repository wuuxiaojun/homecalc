<script lang="ts">
  import { appState } from '../../state/appState.svelte';

  const slot = $derived(appState.activeSlot);
  const scenario = $derived(slot.scenario);
  const total = $derived(scenario?.total_statement);

  const purchasePrice = $derived(slot.purchase?.house.purchase_price || 0);
  const cashDown = $derived(slot.purchase?.tools.find(t => 'Cash' in t)?.Cash?.amount || 0);
  const principalBorrowed = $derived(Math.max(0, purchasePrice - cashDown));
  const totalInterest = $derived(total?.total_interest_paid || 0);
  const totalHolding = $derived(total?.total_holding_cost || 0);
  const totalTaxSavings = $derived(total?.total_tax_savings || 0);
  const totalCashInterest = $derived(total?.total_cash_interest || 0);

  const grossCost = $derived(purchasePrice + totalInterest + totalHolding);
  const netCost = $derived(total?.total_paid || grossCost);

  // Percentages
  const pctPrincipal = $derived(grossCost > 0 ? (principalBorrowed / grossCost) * 100 : 0);
  const pctCash = $derived(grossCost > 0 ? (cashDown / grossCost) * 100 : 0);
  const pctInterest = $derived(grossCost > 0 ? (totalInterest / grossCost) * 100 : 0);
  const pctHolding = $derived(grossCost > 0 ? (totalHolding / grossCost) * 100 : 0);
</script>

<div class="p-5 rounded-2xl bg-zinc-900/60 border border-zinc-800/80 space-y-4">
  <div class="flex items-center justify-between border-b border-zinc-800/60 pb-3">
    <div>
      <h3 class="text-sm font-semibold text-zinc-200 flex items-center gap-2">
        <span>🍩</span>
        <span>Lifetime Cost & Waste Breakdown</span>
      </h3>
      <p class="text-xs text-zinc-500">Distribution of all gross capital outlays and offsets</p>
    </div>
    <div class="text-right font-mono">
      <div class="text-xs text-zinc-400">Total Net Outlay</div>
      <div class="text-base font-bold text-white tabular-nums">${Math.round(netCost).toLocaleString()}</div>
    </div>
  </div>

  <!-- Horizontal Multi-Tier Stacked Bar -->
  <div class="space-y-2">
    <div class="w-full h-5 rounded-xl bg-zinc-800 overflow-hidden flex shadow-inner">
      {#if pctCash > 0}
        <div
          class="h-full bg-emerald-500 transition-all duration-300 relative group"
          style="width: {pctCash}%"
          title="Cash Down: {pctCash.toFixed(1)}%"
        ></div>
      {/if}
      {#if pctPrincipal > 0}
        <div
          class="h-full bg-indigo-500 transition-all duration-300 relative group"
          style="width: {pctPrincipal}%"
          title="Loan Principal: {pctPrincipal.toFixed(1)}%"
        ></div>
      {/if}
      {#if pctInterest > 0}
        <div
          class="h-full bg-rose-500 transition-all duration-300 relative group"
          style="width: {pctInterest}%"
          title="Total Interest: {pctInterest.toFixed(1)}%"
        ></div>
      {/if}
      {#if pctHolding > 0}
        <div
          class="h-full bg-amber-500 transition-all duration-300 relative group"
          style="width: {pctHolding}%"
          title="Holding Costs: {pctHolding.toFixed(1)}%"
        ></div>
      {/if}
    </div>

    <div class="grid grid-cols-2 sm:grid-cols-4 gap-2 text-xs font-mono pt-2">
      <!-- Cash Down -->
      <div class="p-2.5 rounded-xl bg-zinc-950/60 border border-zinc-800/60">
        <div class="flex items-center gap-1.5 text-zinc-400">
          <span class="w-2 h-2 rounded-full bg-emerald-400"></span>
          <span>Cash Down</span>
        </div>
        <div class="text-sm font-bold text-emerald-400 mt-1 tabular-nums">${Math.round(cashDown).toLocaleString()}</div>
        <div class="text-[10px] text-zinc-500">{pctCash.toFixed(1)}% of gross</div>
      </div>

      <!-- Loan Principal -->
      <div class="p-2.5 rounded-xl bg-zinc-950/60 border border-zinc-800/60">
        <div class="flex items-center gap-1.5 text-zinc-400">
          <span class="w-2 h-2 rounded-full bg-indigo-400"></span>
          <span>Loan Principal</span>
        </div>
        <div class="text-sm font-bold text-indigo-300 mt-1 tabular-nums">${Math.round(principalBorrowed).toLocaleString()}</div>
        <div class="text-[10px] text-zinc-500">{pctPrincipal.toFixed(1)}% of gross</div>
      </div>

      <!-- Total Interest -->
      <div class="p-2.5 rounded-xl bg-zinc-950/60 border border-zinc-800/60">
        <div class="flex items-center gap-1.5 text-zinc-400">
          <span class="w-2 h-2 rounded-full bg-rose-400"></span>
          <span>Total Interest</span>
        </div>
        <div class="text-sm font-bold text-rose-400 mt-1 tabular-nums">${Math.round(totalInterest).toLocaleString()}</div>
        <div class="text-[10px] text-zinc-500">{pctInterest.toFixed(1)}% of gross</div>
      </div>

      <!-- Holding Costs -->
      <div class="p-2.5 rounded-xl bg-zinc-950/60 border border-zinc-800/60">
        <div class="flex items-center gap-1.5 text-zinc-400">
          <span class="w-2 h-2 rounded-full bg-amber-400"></span>
          <span>Holding Costs</span>
        </div>
        <div class="text-sm font-bold text-amber-400 mt-1 tabular-nums">${Math.round(totalHolding).toLocaleString()}</div>
        <div class="text-[10px] text-zinc-500">{pctHolding.toFixed(1)}% of gross</div>
      </div>
    </div>
  </div>

  <!-- Offsets (Tax Deductions & Cash Yield) -->
  <div class="p-3 rounded-xl bg-zinc-950/80 border border-zinc-800/80 flex flex-wrap items-center justify-between gap-4 text-xs font-mono">
    <div class="flex items-center gap-4">
      <div class="flex items-center gap-2">
        <span class="text-emerald-400 font-bold">🧾 Tax Deduction Offset:</span>
        <span class="text-emerald-400 font-semibold tabular-nums">-${Math.round(totalTaxSavings).toLocaleString()}</span>
      </div>
      <div class="flex items-center gap-2">
        <span class="text-cyan-400 font-bold">💵 Cash Yield Earned:</span>
        <span class="text-cyan-400 font-semibold tabular-nums">-${Math.round(totalCashInterest).toLocaleString()}</span>
      </div>
    </div>
    <div class="text-zinc-400">
      Combined Benefit: <span class="font-bold text-emerald-300">-${Math.round(totalTaxSavings + totalCashInterest).toLocaleString()}</span>
    </div>
  </div>
</div>
