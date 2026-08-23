<script lang="ts">
  import { appState } from '../../state/appState.svelte';
  import KpiCards from './KpiCards.svelte';
  import Card from '../common/Card.svelte';
  import { formatCurrency, formatPercent, formatYears, isValidNumber } from '../../utils/format';

  const purchase = $derived(appState.activeSlot.purchase);
  const house = $derived(purchase.house);
  const scenario = $derived(appState.activeSlot.scenario);
  const analysis = $derived(appState.activeSlot.analysis);

  // Derive Milestone metrics from yearly statements
  const yStatements = $derived(scenario?.yearly_statement || []);
  const y5 = $derived(yStatements.find(y => y.year === 5));
  const y10 = $derived(yStatements.find(y => y.year === 10));

  const monthlyTax = $derived((house.purchase_price * house.annual_property_tax_rate * 0.01) / 12);
  const monthlyIns = $derived(house.annual_insurance / 12);
  const initialHolding = $derived(monthlyTax + monthlyIns + house.monthly_hoa);

  // Month 1 Cost Breakdown for visual bar
  const m1 = $derived(scenario?.monthly_statement?.[1]);
  const m1Principal = $derived((m1?.mortgage?.principal_paid || 0));
  const m1Interest = $derived((m1?.mortgage?.interest_paid || 0));
  const m1Tax = $derived(m1?.house?.monthly_property_tax || 0);
  const m1Ins = $derived(m1?.house?.monthly_insurance || 0);
  const m1Hoa = $derived(m1?.house?.monthly_hoa || 0);
  const m1Total = $derived(m1Principal + m1Interest + m1Tax + m1Ins + m1Hoa || 1);

  const pctPrincipal = $derived(isValidNumber(m1Principal / m1Total) ? ((m1Principal / m1Total) * 100).toFixed(1) : '0.0');
  const pctInterest = $derived(isValidNumber(m1Interest / m1Total) ? ((m1Interest / m1Total) * 100).toFixed(1) : '0.0');
  const pctTax = $derived(isValidNumber(m1Tax / m1Total) ? ((m1Tax / m1Total) * 100).toFixed(1) : '0.0');
  const pctIns = $derived(isValidNumber(m1Ins / m1Total) ? ((m1Ins / m1Total) * 100).toFixed(1) : '0.0');
  const pctHoa = $derived(isValidNumber(m1Hoa / m1Total) ? ((m1Hoa / m1Total) * 100).toFixed(1) : '0.0');
</script>

<div class="space-y-6">
  <!-- Top KPI Grid -->
  <KpiCards />

  <!-- Detailed Property & Financing Summary Breakdown -->
  <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
    <!-- Property Details Table Card -->
    <Card icon="🏡" title="Property & Valuation">
      {#snippet headerRight()}
        <span class="text-xs font-mono text-emerald-400 font-bold">
          {formatCurrency(house.purchase_price)}
        </span>
      {/snippet}

      <table class="w-full text-xs">
        <tbody class="divide-y divide-zinc-800/40 font-mono">
          <tr class="py-2 flex justify-between">
            <td class="text-zinc-400">Purchase Price</td>
            <td class="text-zinc-200 font-semibold tabular-nums">{formatCurrency(house.purchase_price)}</td>
          </tr>
          <tr class="py-2 flex justify-between">
            <td class="text-zinc-400">Annual Property Tax Rate</td>
            <td class="text-zinc-200 font-semibold tabular-nums">{formatPercent(house.annual_property_tax_rate, 2)} ({formatCurrency(monthlyTax)}/mo)</td>
          </tr>
          <tr class="py-2 flex justify-between">
            <td class="text-zinc-400">Annual Home Insurance</td>
            <td class="text-zinc-200 font-semibold tabular-nums">{formatCurrency(house.annual_insurance)} ({formatCurrency(monthlyIns)}/mo)</td>
          </tr>
          <tr class="py-2 flex justify-between">
            <td class="text-zinc-400">Monthly HOA Fee</td>
            <td class="text-zinc-200 font-semibold tabular-nums">{formatCurrency(house.monthly_hoa)}/mo</td>
          </tr>
          <tr class="py-2 flex justify-between">
            <td class="text-amber-400 font-medium">Initial Monthly Holding Cost</td>
            <td class="text-amber-400 font-bold tabular-nums">{formatCurrency(initialHolding)}/mo</td>
          </tr>
        </tbody>
      </table>
    </Card>

    <!-- Financial Tools Table Card -->
    <Card icon="💳" title="Financing Structure">
      {#snippet headerRight()}
        <span class="text-xs font-mono text-zinc-400 font-medium">
          {purchase.tools.length} Instruments
        </span>
      {/snippet}

      <div class="space-y-3">
        {#each purchase.tools as tool}
          {#if 'Cash' in tool && tool.Cash}
            <div class="p-3 rounded-xl bg-zinc-950/80 border border-zinc-800/60 flex items-center justify-between text-xs">
              <div class="flex items-center gap-2.5">
                <span class="w-2 h-2 rounded-full bg-emerald-400"></span>
                <div>
                  <div class="font-semibold text-zinc-200">Cash Down Payment</div>
                  <div class="text-[11px] text-zinc-500 font-mono">Yield: {formatPercent(tool.Cash.rate, 2)} APR</div>
                </div>
              </div>
              <div class="text-right font-mono">
                <div class="font-bold text-emerald-400 tabular-nums">{formatCurrency(tool.Cash.amount)}</div>
                <div class="text-[10px] text-zinc-500">{house.purchase_price > 0 ? ((tool.Cash.amount / house.purchase_price) * 100).toFixed(1) : '0.0'}% of price</div>
              </div>
            </div>
          {:else if 'Mortgage' in tool && tool.Mortgage}
            <div class="p-3 rounded-xl bg-zinc-950/80 border border-zinc-800/60 flex items-center justify-between text-xs">
              <div class="flex items-center gap-2.5">
                <span class="w-2 h-2 rounded-full bg-indigo-400"></span>
                <div>
                  <div class="font-semibold text-zinc-200">Mortgage Loan ({tool.Mortgage.term} Years)</div>
                  <div class="text-[11px] text-zinc-500 font-mono">Rate: {formatPercent(tool.Mortgage.rate, 2)} Fixed</div>
                </div>
              </div>
              <div class="text-right font-mono">
                <div class="font-bold text-indigo-300 tabular-nums">{formatCurrency(tool.Mortgage.amount)}</div>
                <div class="text-[10px] text-zinc-500">{house.purchase_price > 0 ? ((tool.Mortgage.amount / house.purchase_price) * 100).toFixed(1) : '0.0'}% LTV</div>
              </div>
            </div>
          {:else if 'Loc' in tool && tool.Loc}
            <div class="p-3 rounded-xl bg-zinc-950/80 border border-zinc-800/60 flex items-center justify-between text-xs">
              <div class="flex items-center gap-2.5">
                <span class="w-2 h-2 rounded-full bg-amber-400"></span>
                <div>
                  <div class="font-semibold text-zinc-200">Line of Credit (LOC)</div>
                  <div class="text-[11px] text-zinc-500 font-mono">Rate: {formatPercent(tool.Loc.rate, 2)} Variable</div>
                </div>
              </div>
              <div class="text-right font-mono">
                <div class="font-bold text-amber-300 tabular-nums">{formatCurrency(tool.Loc.amount)}</div>
                <div class="text-[10px] text-zinc-500">{house.purchase_price > 0 ? ((tool.Loc.amount / house.purchase_price) * 100).toFixed(1) : '0.0'}% of price</div>
              </div>
            </div>
          {/if}
        {/each}
      </div>
    </Card>
  </div>

  <!-- Bottom Row: 30-Year Milestones & Monthly Payment Composition -->
  <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
    <!-- Milestone Progress Card -->
    <Card icon="📈" title="Equity Milestones & Principal Trajectory" class="lg:col-span-2">
      {#snippet headerRight()}
        <span class="text-xs font-mono text-zinc-400">Amortization Milestones</span>
      {/snippet}

      <div class="grid grid-cols-1 sm:grid-cols-3 gap-3.5">
        <!-- Year 5 Milestone -->
        <div class="p-3.5 rounded-xl bg-zinc-950/70 border border-zinc-800/60 space-y-2">
          <div class="flex items-center justify-between">
            <span class="text-xs font-mono font-semibold text-zinc-400">Year 5</span>
            <span class="text-[11px] font-mono px-1.5 py-0.5 rounded bg-zinc-800 text-zinc-300">
              {y5 && house.purchase_price > 0 ? `${(((house.purchase_price - y5.ending_remaining_balance) / house.purchase_price) * 100).toFixed(0)}% Equity` : 'N/A'}
            </span>
          </div>
          <div class="text-lg font-bold font-mono text-white tabular-nums">
            {y5 ? formatCurrency(y5.ending_remaining_balance) : '$0'}
          </div>
          <div class="text-[11px] font-mono text-zinc-500">Remaining Debt Balance</div>
        </div>

        <!-- Year 10 Milestone -->
        <div class="p-3.5 rounded-xl bg-zinc-950/70 border border-zinc-800/60 space-y-2">
          <div class="flex items-center justify-between">
            <span class="text-xs font-mono font-semibold text-indigo-300">Year 10</span>
            <span class="text-[11px] font-mono px-1.5 py-0.5 rounded bg-indigo-950/80 text-indigo-300 border border-indigo-800/50">
              {y10 && house.purchase_price > 0 ? `${(((house.purchase_price - y10.ending_remaining_balance) / house.purchase_price) * 100).toFixed(0)}% Equity` : 'N/A'}
            </span>
          </div>
          <div class="text-lg font-bold font-mono text-indigo-200 tabular-nums">
            {y10 ? formatCurrency(y10.ending_remaining_balance) : '$0'}
          </div>
          <div class="text-[11px] font-mono text-zinc-500">Remaining Debt Balance</div>
        </div>

        <!-- Payoff Goal -->
        <div class="p-3.5 rounded-xl bg-zinc-950/70 border border-zinc-800/60 space-y-2">
          <div class="flex items-center justify-between">
            <span class="text-xs font-mono font-semibold text-emerald-300">Payoff Goal</span>
            <span class="text-[11px] font-mono px-1.5 py-0.5 rounded bg-emerald-950/80 text-emerald-300 border border-emerald-800/50">
              100% Equity
            </span>
          </div>
          <div class="text-lg font-bold font-mono text-emerald-400 tabular-nums">
            {analysis && isValidNumber(analysis.payoff_month) ? `${formatYears(analysis.payoff_month)} Years` : '30.0 Years'}
          </div>
          <div class="text-[11px] font-mono text-zinc-500">
            {analysis && isValidNumber(analysis.payoff_month) ? `Month ${analysis.payoff_month} to debt-free` : 'Month 360'}
          </div>
        </div>
      </div>
    </Card>

    <!-- Monthly Payment Breakdown Card -->
    <Card icon="📊" title="Monthly Outlay Split">
      {#snippet headerRight()}
        <span class="text-xs font-mono text-emerald-400 font-bold">
          {formatCurrency(m1Total)}/mo
        </span>
      {/snippet}

      <!-- Segmented Bar -->
      <div class="h-3 w-full rounded-full bg-zinc-950 overflow-hidden flex shadow-inner">
        <div class="bg-indigo-500 h-full" style="width: {pctPrincipal}%" title="Principal: {pctPrincipal}%"></div>
        <div class="bg-rose-500 h-full" style="width: {pctInterest}%" title="Interest: {pctInterest}%"></div>
        <div class="bg-amber-500 h-full" style="width: {pctTax}%" title="Property Tax: {pctTax}%"></div>
        <div class="bg-cyan-500 h-full" style="width: {pctIns}%" title="Insurance: {pctIns}%"></div>
        <div class="bg-purple-500 h-full" style="width: {pctHoa}%" title="HOA: {pctHoa}%"></div>
      </div>

      <!-- Legend -->
      <div class="grid grid-cols-2 gap-2 text-[11px] font-mono text-zinc-400">
        <div class="flex items-center justify-between">
          <span class="flex items-center gap-1.5"><span class="w-2 h-2 rounded-full bg-indigo-500"></span>Principal</span>
          <span class="text-zinc-200 font-semibold">{formatCurrency(m1Principal)}</span>
        </div>
        <div class="flex items-center justify-between">
          <span class="flex items-center gap-1.5"><span class="w-2 h-2 rounded-full bg-rose-500"></span>Interest</span>
          <span class="text-zinc-200 font-semibold">{formatCurrency(m1Interest)}</span>
        </div>
        <div class="flex items-center justify-between">
          <span class="flex items-center gap-1.5"><span class="w-2 h-2 rounded-full bg-amber-500"></span>Property Tax</span>
          <span class="text-zinc-200 font-semibold">{formatCurrency(m1Tax)}</span>
        </div>
        <div class="flex items-center justify-between">
          <span class="flex items-center gap-1.5"><span class="w-2 h-2 rounded-full bg-cyan-500"></span>Insurance</span>
          <span class="text-zinc-200 font-semibold">{formatCurrency(m1Ins)}</span>
        </div>
      </div>
    </Card>
  </div>
</div>
