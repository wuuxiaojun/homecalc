<script lang="ts">
  import { appState } from '../../state/appState.svelte';
  import Card from '../common/Card.svelte';
  import { formatCurrency, formatPercent, formatYears, isValidNumber } from '../../utils/format';

  const slot = $derived(appState.activeSlot);
  const scenario = $derived(slot.scenario);
  const analysis = $derived(slot.analysis);
  const total = $derived(scenario?.total_statement);
  const displayPurchase = $derived(scenario?.purchase || slot.purchase);
  const house = $derived(displayPurchase.house);

  const monthlyTax = $derived((house.purchase_price * house.annual_property_tax_rate * 0.01) / 12);
  const monthlyIns = $derived(house.annual_insurance / 12);
  const initialHolding = $derived(monthlyTax + monthlyIns + house.monthly_hoa);

  // Statements
  const monthlyStatements = $derived(scenario?.monthly_statement || []);
  const yearlyStatements = $derived(scenario?.yearly_statement || []);
  const payoffMonth = $derived(analysis?.payoff_month || 0);
  const hasDebt = $derived(displayPurchase.tools.some(t => ('Mortgage' in t && (t.Mortgage?.amount ?? 0) > 0) || ('Loc' in t && (t.Loc?.amount ?? 0) > 0)));

  // Calculate 3-stage monthly progression
  const stageData = $derived.by(() => {
    if (hasDebt && monthlyStatements.length > 1) {
      const midMonth = payoffMonth > 1 ? Math.floor(payoffMonth / 2) : 1;
      const endMonth = payoffMonth > 0 ? payoffMonth : 1;

      const m1 = monthlyStatements[1] || monthlyStatements[0];
      const m2 = monthlyStatements[midMonth] || m1;
      const m3 = monthlyStatements[endMonth] || m1;

      const parseRow = (m: typeof m1, label: string, sublabel: string) => {
        const principal = m.mortgage?.principal_paid || 0;
        const interest = (m.mortgage?.interest_paid || 0) + (m.loc?.monthly_payment || 0);
        const holding = (m.house?.monthly_property_tax || 0) + (m.house?.monthly_insurance || 0) + (m.house?.monthly_hoa || 0);
        const cashYield = m.cash?.cash_interest || 0;
        const netOutlay = Math.max(0, principal + interest + holding - cashYield);
        const gross = principal + interest + holding || 1;

        return {
          label,
          sublabel,
          principal,
          interest,
          holding,
          cashYield,
          netOutlay,
          pctPrincipal: ((principal / gross) * 100).toFixed(1),
          pctInterest: ((interest / gross) * 100).toFixed(1),
          pctHolding: ((holding / gross) * 100).toFixed(1)
        };
      };

      return [
        parseRow(m1, 'Initial (Mo 1)', 'Year 1'),
        parseRow(m2, `Midpoint (Mo ${m2.month})`, `Yr ${Math.ceil(m2.month / 12)}`),
        parseRow(m3, `Payoff (Mo ${m3.month})`, `Yr ${Math.ceil(m3.month / 12)}`)
      ];
    } else {
      // All-Cash / Debt-Free 3-stage holding cost projection over 30 years
      const y1 = yearlyStatements.find(y => y.year === 1);
      const y15 = yearlyStatements.find(y => y.year === 15);
      const y30 = yearlyStatements.find(y => y.year === 30);

      const parseYear = (y: typeof y1, yearNum: number, label: string) => {
        const holding = y && y.annual_holding_cost > 0 ? y.annual_holding_cost / 12 : initialHolding * Math.pow(1.025, yearNum - 1);
        const cashYield = y ? y.annual_cash_interest / 12 : 0;
        const netOutlay = Math.max(0, holding - cashYield);

        return {
          label,
          sublabel: `Year ${yearNum}`,
          principal: 0,
          interest: 0,
          holding,
          cashYield,
          netOutlay,
          pctPrincipal: '0.0',
          pctInterest: '0.0',
          pctHolding: '100.0'
        };
      };

      return [
        parseYear(y1, 1, 'Initial (Mo 1)'),
        parseYear(y15, 15, 'Year 15 (Mo 180)'),
        parseYear(y30, 30, 'Year 30 (Mo 360)')
      ];
    }
  });

  const wasteRatio = $derived(isValidNumber(analysis?.waste_ratio) ? (analysis.waste_ratio * 100) : null);
  const taxSavingsRatio = $derived(isValidNumber(analysis?.tax_savings_ratio) ? (analysis.tax_savings_ratio * 100) : null);
</script>

<div class="space-y-6">
  <!-- ========================================================================= -->
  <!-- PART 1: STRUCTURAL FOUNDATION (Top Row: Property + Financing)             -->
  <!-- ========================================================================= -->
  <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
    <!-- 1. Property Details Table Card -->
    <Card icon="🏡" title="Property">
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

    <!-- 2. Financial Tools Table Card -->
    <Card icon="💳" title="Financing Structure">
      {#snippet headerRight()}
        <span class="text-xs font-mono text-zinc-400 font-medium">
          {displayPurchase.tools.length} Instruments
        </span>
      {/snippet}

      <div class="space-y-3">
        {#each displayPurchase.tools as tool}
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
                <div class="text-[10px] text-zinc-500">{house.purchase_price > 0 ? ((tool.Mortgage.amount / house.purchase_price) * 100).toFixed(1) : '0.0'}% of price</div>
              </div>
            </div>
          {:else if 'Loc' in tool && tool.Loc}
            <div class="p-3 rounded-xl bg-zinc-950/80 border border-zinc-800/60 flex items-center justify-between text-xs">
              <div class="flex items-center gap-2.5">
                <span class="w-2 h-2 rounded-full bg-amber-400"></span>
                <div>
                  <div class="font-semibold text-zinc-200">Line of Credit (LOC)</div>
                  <div class="text-[11px] text-zinc-500 font-mono">Rate: {formatPercent(tool.Loc.rate, 2)} Fixed</div>
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

  <!-- ========================================================================= -->
  <!-- PART 2: MONTHLY CASH FLOW DYNAMICS (3 Stages + Effective Monthly Outlay)  -->
  <!-- ========================================================================= -->
  <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
    <!-- 3-Stage Monthly Cash Flow Progression Card -->
    <Card icon="📊" title="Monthly Payment Split" class="lg:col-span-2">
      {#snippet headerRight()}
        <span class="text-xs font-mono text-zinc-400">
          {payoffMonth > 0 ? `Payoff in Month ${payoffMonth}` : 'All-Cash / Debt-Free'}
        </span>
      {/snippet}

      <div class="grid grid-cols-1 sm:grid-cols-3 gap-3.5">
        {#each stageData as stage}
          <div class="p-3.5 rounded-xl bg-zinc-950/70 border border-zinc-800/60 space-y-2.5 flex flex-col justify-between">
            <div>
              <div class="flex items-center justify-between pb-1.5 border-b border-zinc-800/50">
                <span class="text-xs font-mono font-bold text-zinc-200">{stage.label}</span>
                <span class="text-[10px] font-mono text-zinc-500">{stage.sublabel}</span>
              </div>

              <!-- Net Outlay -->
              <div class="pt-2">
                <div class="text-xs text-zinc-500 font-mono">Net Outlay</div>
                <div class="text-lg font-bold font-mono text-white tabular-nums">
                  {formatCurrency(stage.netOutlay)}<span class="text-xs text-zinc-400 font-normal">/mo</span>
                </div>
              </div>

              <!-- Mini Stacked Bar -->
              <div class="h-2 w-full rounded-full bg-zinc-900 overflow-hidden flex my-2">
                {#if parseFloat(stage.pctPrincipal) > 0}
                  <div class="bg-indigo-500 h-full" style="width: {stage.pctPrincipal}%" title="Principal: {stage.pctPrincipal}%"></div>
                {/if}
                {#if parseFloat(stage.pctInterest) > 0}
                  <div class="bg-rose-500 h-full" style="width: {stage.pctInterest}%" title="Interest: {stage.pctInterest}%"></div>
                {/if}
                {#if parseFloat(stage.pctHolding) > 0}
                  <div class="bg-amber-500 h-full" style="width: {stage.pctHolding}%" title="Holding: {stage.pctHolding}%"></div>
                {/if}
              </div>

              <!-- Itemized breakdown -->
              <div class="space-y-1 text-[11px] font-mono text-zinc-400 pt-1">
                {#if hasDebt}
                  <div class="flex justify-between">
                    <span class="flex items-center gap-1"><span class="w-1.5 h-1.5 rounded-full bg-indigo-500"></span>Principal</span>
                    <span class="text-zinc-200 font-semibold">{formatCurrency(stage.principal)}</span>
                  </div>
                  <div class="flex justify-between">
                    <span class="flex items-center gap-1"><span class="w-1.5 h-1.5 rounded-full bg-rose-500"></span>Interest</span>
                    <span class="text-zinc-200 font-semibold">{formatCurrency(stage.interest)}</span>
                  </div>
                {/if}
                <div class="flex justify-between">
                  <span class="flex items-center gap-1"><span class="w-1.5 h-1.5 rounded-full bg-amber-500"></span>Holding</span>
                  <span class="text-zinc-200 font-semibold">{formatCurrency(stage.holding)}</span>
                </div>
                {#if stage.cashYield > 0}
                  <div class="flex justify-between text-emerald-400">
                    <span>- Cash Yield</span>
                    <span class="font-semibold">-{formatCurrency(stage.cashYield)}</span>
                  </div>
                {/if}
              </div>
            </div>
          </div>
        {/each}
      </div>
    </Card>

    <!-- Effective Monthly Outlay Card -->
    <Card icon="💳" title="Effective Outlay">
      {#snippet headerRight()}
        <span class="text-[11px] font-mono px-2 py-0.5 rounded-full bg-indigo-950 text-indigo-300 border border-indigo-800/40">
          All-In Average
        </span>
      {/snippet}

      <div class="space-y-3 flex-1 flex flex-col justify-between">
        <div>
          <div class="text-2xl lg:text-3xl font-bold font-mono tracking-tight text-indigo-300 tabular-nums">
            {formatCurrency(analysis?.effective_monthly_cost)}<span class="text-xs text-zinc-400 font-normal"> / month</span>
          </div>
          <p class="text-xs text-zinc-400 leading-relaxed mt-2">
            True all-in lifetime average monthly cost factoring in debt service, property holding costs, IRS tax deductions, and cash interest yield.
          </p>
        </div>

        <div class="p-3 rounded-xl bg-zinc-950/70 border border-zinc-800/50 space-y-1.5 text-xs font-mono">
          <div class="flex justify-between text-zinc-400">
            <span>Lifetime Net Total:</span>
            <span class="text-white font-semibold">{formatCurrency(total?.total_paid)}</span>
          </div>
          <div class="flex justify-between text-zinc-400">
            <span>Payoff Horizon:</span>
            <span class="text-indigo-300 font-semibold">{payoffMonth > 0 ? `${payoffMonth} Months` : 'Immediate'}</span>
          </div>
        </div>
      </div>
    </Card>
  </div>

  <!-- ========================================================================= -->
  <!-- PART 3: LIFETIME FINANCIAL TOTALS (Payoff -> Interest -> Tax -> Total)    -->
  <!-- ========================================================================= -->
  <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
    <!-- 1. Payoff Timeline -->
    <Card icon="⏱️" title="Payoff Timeline">
      {#snippet headerRight()}
        <span class="text-[11px] font-mono px-2 py-0.5 rounded-full bg-zinc-800 text-zinc-300">
          {isValidNumber(payoffMonth) ? `Month ${payoffMonth}` : 'N/A'}
        </span>
      {/snippet}

      <div class="flex items-baseline gap-2">
        <span class="text-2xl lg:text-3xl font-bold font-mono tracking-tight text-white tabular-nums">
          {formatYears(payoffMonth)}
        </span>
        <span class="text-xs text-zinc-400 font-medium">Years to Debt-Free</span>
      </div>
      <div class="text-[11px] font-mono text-zinc-500">
        {#if isValidNumber(payoffMonth) && payoffMonth < 360 && payoffMonth > 0}
          <span class="text-emerald-400 font-semibold">⚡ {360 - payoffMonth} months faster</span> than 30-year base
        {:else if isValidNumber(payoffMonth) && payoffMonth === 360}
          <span>Standard 30-year amortization schedule</span>
        {:else}
          <span>Immediate equity / all-cash</span>
        {/if}
      </div>
    </Card>

    <!-- 2. Total Interest Paid & Waste Ratio -->
    <Card icon="📉" title="Total Interest">
      {#snippet headerRight()}
        <span class="text-[11px] font-mono px-2 py-0.5 rounded-full bg-rose-950 text-rose-300 border border-rose-800/40">
          Waste: {formatPercent(wasteRatio)}
        </span>
      {/snippet}

      <div>
        <span class="text-2xl lg:text-3xl font-bold font-mono tracking-tight text-rose-400 tabular-nums">
          {formatCurrency(total?.total_interest_paid)}
        </span>
      </div>
      <div class="text-[11px] font-mono text-zinc-500">
        Borrowing friction cost over lifetime
      </div>
    </Card>

    <!-- 3. Tax Savings Realized -->
    <Card icon="🧾" title="Tax Savings">
      {#snippet headerRight()}
        <span class="text-[11px] font-mono px-2 py-0.5 rounded-full bg-emerald-950 text-emerald-300 border border-emerald-800/40">
          Offset: {formatPercent(taxSavingsRatio)}
        </span>
      {/snippet}

      <div>
        <span class="text-2xl lg:text-3xl font-bold font-mono tracking-tight text-emerald-400 tabular-nums">
          {formatCurrency(total?.total_tax_savings)}
        </span>
      </div>
      <div class="text-[11px] font-mono text-zinc-500">
        IRS mortgage interest deduction (24% bracket)
      </div>
    </Card>

    <!-- 4. Total Lifetime Outlay -->
    <Card icon="💸" title="Lifetime Outlay">
      {#snippet headerRight()}
        <span class="text-[11px] font-mono px-2 py-0.5 rounded-full bg-zinc-800 text-zinc-300">
          Grand Total
        </span>
      {/snippet}

      <div>
        <span class="text-2xl lg:text-3xl font-bold font-mono tracking-tight text-white tabular-nums">
          {formatCurrency(total?.total_paid)}
        </span>
      </div>
      <div class="text-[11px] font-mono text-zinc-500">
        Cash earned: +{formatCurrency(total?.total_cash_interest)}
      </div>
    </Card>
  </div>
</div>
