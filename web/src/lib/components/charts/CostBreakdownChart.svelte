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
  const combinedBenefit = $derived(totalTaxSavings + totalCashInterest);

  // Percentages
  const pctPrincipal = $derived(grossCost > 0 ? (principalBorrowed / grossCost) * 100 : 0);
  const pctCash = $derived(grossCost > 0 ? (cashDown / grossCost) * 100 : 0);
  const pctInterest = $derived(grossCost > 0 ? (totalInterest / grossCost) * 100 : 0);
  const pctHolding = $derived(grossCost > 0 ? (totalHolding / grossCost) * 100 : 0);
</script>

<Card icon="🍩" title="Lifetime Breakdown">
  {#snippet headerRight()}
    <div class="text-right font-mono">
      <span class="text-[10px] text-zinc-400 mr-1.5 hidden sm:inline">Gross Outflow:</span>
      <span class="text-xs sm:text-sm font-bold text-zinc-200 tabular-nums">{formatCurrency(grossCost)}</span>
    </div>
  {/snippet}

  <div class="space-y-4">
    <!-- Horizontal Multi-Tier Stacked Bar -->
    <div class="w-full h-4 rounded-xl bg-zinc-800 overflow-hidden flex shadow-inner">
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

    <!-- 2 Rows x 4 Cards Grid -->
    <div class="grid grid-cols-2 sm:grid-cols-4 gap-2 text-xs font-mono">
      <!-- ROW 1: GROSS COST COMPONENTS -->
      <!-- 1. Cash Down -->
      <div class="p-2 rounded-xl bg-zinc-950/70 border border-zinc-800/60 flex flex-col justify-between">
        <div class="flex items-center gap-1.5 text-zinc-400 text-[11px]">
          <span class="w-2 h-2 rounded-full bg-emerald-400"></span>
          <span>Cash Down</span>
        </div>
        <div class="mt-1">
          <div class="text-xs sm:text-sm font-bold text-emerald-400 tabular-nums">{formatCurrency(cashDown)}</div>
          <div class="text-[10px] text-zinc-500">{pctCash.toFixed(1)}% gross</div>
        </div>
      </div>

      <!-- 2. Loan Principal -->
      <div class="p-2 rounded-xl bg-zinc-950/70 border border-zinc-800/60 flex flex-col justify-between">
        <div class="flex items-center gap-1.5 text-zinc-400 text-[11px]">
          <span class="w-2 h-2 rounded-full bg-indigo-400"></span>
          <span>Loan Principal</span>
        </div>
        <div class="mt-1">
          <div class="text-xs sm:text-sm font-bold text-indigo-300 tabular-nums">{formatCurrency(principalBorrowed)}</div>
          <div class="text-[10px] text-zinc-500">{pctPrincipal.toFixed(1)}% gross</div>
        </div>
      </div>

      <!-- 3. Total Interest -->
      <div class="p-2 rounded-xl bg-zinc-950/70 border border-zinc-800/60 flex flex-col justify-between">
        <div class="flex items-center gap-1.5 text-zinc-400 text-[11px]">
          <span class="w-2 h-2 rounded-full bg-rose-400"></span>
          <span>Total Interest</span>
        </div>
        <div class="mt-1">
          <div class="text-xs sm:text-sm font-bold text-rose-400 tabular-nums">{formatCurrency(totalInterest)}</div>
          <div class="text-[10px] text-zinc-500">{pctInterest.toFixed(1)}% gross</div>
        </div>
      </div>

      <!-- 4. Holding Costs -->
      <div class="p-2 rounded-xl bg-zinc-950/70 border border-zinc-800/60 flex flex-col justify-between">
        <div class="flex items-center gap-1.5 text-zinc-400 text-[11px]">
          <span class="w-2 h-2 rounded-full bg-amber-400"></span>
          <span>Holding Costs</span>
        </div>
        <div class="mt-1">
          <div class="text-xs sm:text-sm font-bold text-amber-400 tabular-nums">{formatCurrency(totalHolding)}</div>
          <div class="text-[10px] text-zinc-500">{pctHolding.toFixed(1)}% gross</div>
        </div>
      </div>

      <!-- ROW 2: OFFSETS, BENEFITS & NET OUTLAY -->
      <!-- 5. Tax Deductions -->
      <div class="p-2 rounded-xl bg-zinc-950/70 border border-zinc-800/60 flex flex-col justify-between">
        <div class="flex items-center gap-1 text-zinc-400 text-[11px]">
          <span>🧾</span>
          <span>Tax Savings</span>
        </div>
        <div class="mt-1">
          <div class="text-xs sm:text-sm font-bold text-emerald-400 tabular-nums">{formatCurrency(totalTaxSavings)}</div>
          <div class="text-[10px] text-zinc-500">IRS offset</div>
        </div>
      </div>

      <!-- 6. Cash Yield Earned -->
      <div class="p-2 rounded-xl bg-zinc-950/70 border border-zinc-800/60 flex flex-col justify-between">
        <div class="flex items-center gap-1 text-zinc-400 text-[11px]">
          <span>📈</span>
          <span>Cash Yield</span>
        </div>
        <div class="mt-1">
          <div class="text-xs sm:text-sm font-bold text-cyan-400 tabular-nums">{formatCurrency(totalCashInterest)}</div>
          <div class="text-[10px] text-zinc-500">Cash growth</div>
        </div>
      </div>

      <!-- 7. Combined Benefit -->
      <div class="p-2 rounded-xl bg-zinc-950/70 border border-zinc-800/60 flex flex-col justify-between">
        <div class="flex items-center gap-1 text-teal-300 text-[11px]">
          <span>🛡️</span>
          <span>Net Benefit</span>
        </div>
        <div class="mt-1">
          <div class="text-xs sm:text-sm font-bold text-teal-300 tabular-nums">{formatCurrency(combinedBenefit)}</div>
          <div class="text-[10px] text-zinc-500">Tax + yield</div>
        </div>
      </div>

      <!-- 8. Total Net Outlay -->
      <div class="p-2 rounded-xl bg-indigo-950/50 border border-indigo-700/60 flex flex-col justify-between">
        <div class="flex items-center gap-1 text-indigo-300 text-[11px] font-semibold">
          <span>💸</span>
          <span>Net Outlay</span>
        </div>
        <div class="mt-1">
          <div class="text-xs sm:text-sm font-bold text-white tabular-nums">{formatCurrency(netCost)}</div>
          <div class="text-[10px] text-indigo-300">Grand Total</div>
        </div>
      </div>
    </div>
  </div>
</Card>
