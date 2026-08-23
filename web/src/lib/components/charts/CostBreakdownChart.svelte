<script lang="ts">
  import { appState } from '../../state/appState.svelte';
  import Card from '../common/Card.svelte';
  import { formatCurrency, formatPercent, isValidNumber } from '../../utils/format';

  const slot = $derived(appState.activeSlot);
  const scenario = $derived(slot.scenario);
  const total = $derived(scenario?.total_statement);

  const purchasePrice = $derived(slot.purchase.house.purchase_price);
  const cashDown = $derived(slot.purchase.tools.find(t => 'Cash' in t)?.Cash?.amount || 0);
  const principalBorrowed = $derived(purchasePrice - cashDown);
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

<Card icon="🍩" title="Lifetime Cost Breakdown">
  {#snippet headerRight()}
    <div class="text-right font-mono">
      <span class="text-[10px] text-zinc-400 mr-1.5">Total Net Outlay:</span>
      <span class="text-sm font-bold text-white tabular-nums">{formatCurrency(netCost)}</span>
    </div>
  {/snippet}

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
        <div class="text-sm font-bold text-emerald-400 mt-1 tabular-nums">{formatCurrency(cashDown)}</div>
        <div class="text-[10px] text-zinc-500">{pctCash.toFixed(1)}% of gross</div>
      </div>

      <!-- Loan Principal -->
      <div class="p-2.5 rounded-xl bg-zinc-950/60 border border-zinc-800/60">
        <div class="flex items-center gap-1.5 text-zinc-400">
          <span class="w-2 h-2 rounded-full bg-indigo-400"></span>
          <span>Loan Principal</span>
        </div>
        <div class="text-sm font-bold text-indigo-300 mt-1 tabular-nums">{formatCurrency(principalBorrowed)}</div>
        <div class="text-[10px] text-zinc-500">{pctPrincipal.toFixed(1)}% of gross</div>
      </div>

      <!-- Total Interest -->
      <div class="p-2.5 rounded-xl bg-zinc-950/60 border border-zinc-800/60">
        <div class="flex items-center gap-1.5 text-zinc-400">
          <span class="w-2 h-2 rounded-full bg-rose-400"></span>
          <span>Total Interest</span>
        </div>
        <div class="text-sm font-bold text-rose-400 mt-1 tabular-nums">{formatCurrency(totalInterest)}</div>
        <div class="text-[10px] text-zinc-500">{pctInterest.toFixed(1)}% of gross</div>
      </div>

      <!-- Holding Costs -->
      <div class="p-2.5 rounded-xl bg-zinc-950/60 border border-zinc-800/60">
        <div class="flex items-center gap-1.5 text-zinc-400">
          <span class="w-2 h-2 rounded-full bg-amber-400"></span>
          <span>Holding Costs</span>
        </div>
        <div class="text-sm font-bold text-amber-400 mt-1 tabular-nums">{formatCurrency(totalHolding)}</div>
        <div class="text-[10px] text-zinc-500">{pctHolding.toFixed(1)}% of gross</div>
      </div>
    </div>
  </div>

  <!-- Offsets (Tax Deductions & Cash Yield) -->
  <div class="p-3 rounded-xl bg-zinc-950/80 border border-zinc-800/80 flex flex-wrap items-center justify-between gap-4 text-xs font-mono">
    <div class="flex items-center gap-4">
      <div class="flex items-center gap-2">
        <span class="text-emerald-400 font-bold">🧾 Tax Deduction Offset:</span>
        <span class="text-emerald-400 font-semibold tabular-nums">-{formatCurrency(totalTaxSavings)}</span>
      </div>
      <div class="flex items-center gap-2">
        <span class="text-cyan-400 font-bold">💵 Cash Yield Earned:</span>
        <span class="text-cyan-400 font-semibold tabular-nums">-{formatCurrency(totalCashInterest)}</span>
      </div>
    </div>
    <div class="text-zinc-400">
      Combined Benefit: <span class="font-bold text-emerald-300">-{formatCurrency(totalTaxSavings + totalCashInterest)}</span>
    </div>
  </div>
</Card>
