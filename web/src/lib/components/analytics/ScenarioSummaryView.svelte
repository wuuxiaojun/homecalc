<script lang="ts">
  import { appState } from '../../state/appState.svelte';
  import KpiCards from './KpiCards.svelte';

  const purchase = $derived(appState.activeSlot.purchase);
  const house = $derived(purchase.house);
  const scenario = $derived(appState.activeSlot.scenario);
</script>

<div class="space-y-6">
  <!-- Top KPI Grid -->
  <KpiCards />

  <!-- Detailed Property & Financing Summary Breakdown -->
  <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
    <!-- Property Details Table Card -->
    <div class="p-5 rounded-2xl bg-zinc-900/60 border border-zinc-800/80 space-y-4">
      <div class="flex items-center justify-between border-b border-zinc-800/60 pb-3">
        <h3 class="text-sm font-semibold text-zinc-200 flex items-center gap-2">
          <span>🏡</span>
          <span>Property & Valuation</span>
        </h3>
        <span class="text-xs font-mono text-emerald-400 font-bold">
          ${house.purchase_price.toLocaleString()}
        </span>
      </div>

      <table class="w-full text-xs">
        <tbody class="divide-y divide-zinc-800/40 font-mono">
          <tr class="py-2 flex justify-between">
            <td class="text-zinc-400">Purchase Price</td>
            <td class="text-zinc-200 font-semibold tabular-nums">${house.purchase_price.toLocaleString()}</td>
          </tr>
          <tr class="py-2 flex justify-between">
            <td class="text-zinc-400">Annual Property Tax Rate</td>
            <td class="text-zinc-200 font-semibold tabular-nums">{house.annual_property_tax_rate.toFixed(2)}% (${Math.round((house.purchase_price * house.annual_property_tax_rate * 0.01) / 12).toLocaleString()}/mo)</td>
          </tr>
          <tr class="py-2 flex justify-between">
            <td class="text-zinc-400">Annual Home Insurance</td>
            <td class="text-zinc-200 font-semibold tabular-nums">${house.annual_insurance.toLocaleString()} (${Math.round(house.annual_insurance / 12).toLocaleString()}/mo)</td>
          </tr>
          <tr class="py-2 flex justify-between">
            <td class="text-zinc-400">Monthly HOA Fee</td>
            <td class="text-zinc-200 font-semibold tabular-nums">${house.monthly_hoa.toLocaleString()}/mo</td>
          </tr>
          <tr class="py-2 flex justify-between">
            <td class="text-amber-400 font-medium">Initial Monthly Holding Cost</td>
            <td class="text-amber-400 font-bold tabular-nums">${Math.round((house.purchase_price * house.annual_property_tax_rate * 0.01) / 12 + house.annual_insurance / 12 + house.monthly_hoa).toLocaleString()}/mo</td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Financial Tools Table Card -->
    <div class="p-5 rounded-2xl bg-zinc-900/60 border border-zinc-800/80 space-y-4">
      <div class="flex items-center justify-between border-b border-zinc-800/60 pb-3">
        <h3 class="text-sm font-semibold text-zinc-200 flex items-center gap-2">
          <span>💳</span>
          <span>Financing Structure</span>
        </h3>
        <span class="text-xs font-mono text-zinc-400 font-medium">
          {purchase.tools.length} Instruments
        </span>
      </div>

      <div class="space-y-3">
        {#each purchase.tools as tool}
          {#if 'Cash' in tool && tool.Cash}
            <div class="p-3 rounded-xl bg-zinc-950/80 border border-zinc-800/60 flex items-center justify-between text-xs">
              <div class="flex items-center gap-2.5">
                <span class="w-2 h-2 rounded-full bg-emerald-400"></span>
                <div>
                  <div class="font-semibold text-zinc-200">Cash Down Payment</div>
                  <div class="text-[11px] text-zinc-500 font-mono">Yield: {tool.Cash.rate.toFixed(2)}% APR</div>
                </div>
              </div>
              <div class="text-right font-mono">
                <div class="font-bold text-emerald-400 tabular-nums">${tool.Cash.amount.toLocaleString()}</div>
                <div class="text-[10px] text-zinc-500">{((tool.Cash.amount / house.purchase_price) * 100).toFixed(1)}% of price</div>
              </div>
            </div>
          {:else if 'Mortgage' in tool && tool.Mortgage}
            <div class="p-3 rounded-xl bg-zinc-950/80 border border-zinc-800/60 flex items-center justify-between text-xs">
              <div class="flex items-center gap-2.5">
                <span class="w-2 h-2 rounded-full bg-indigo-400"></span>
                <div>
                  <div class="font-semibold text-zinc-200">Mortgage Loan ({tool.Mortgage.term} Years)</div>
                  <div class="text-[11px] text-zinc-500 font-mono">Rate: {tool.Mortgage.rate.toFixed(2)}% Fixed</div>
                </div>
              </div>
              <div class="text-right font-mono">
                <div class="font-bold text-indigo-300 tabular-nums">${tool.Mortgage.amount.toLocaleString()}</div>
                <div class="text-[10px] text-zinc-500">{((tool.Mortgage.amount / house.purchase_price) * 100).toFixed(1)}% LTV</div>
              </div>
            </div>
          {:else if 'Loc' in tool && tool.Loc}
            <div class="p-3 rounded-xl bg-zinc-950/80 border border-zinc-800/60 flex items-center justify-between text-xs">
              <div class="flex items-center gap-2.5">
                <span class="w-2 h-2 rounded-full bg-amber-400"></span>
                <div>
                  <div class="font-semibold text-zinc-200">Line of Credit (LOC)</div>
                  <div class="text-[11px] text-zinc-500 font-mono">Rate: {tool.Loc.rate.toFixed(2)}% Variable</div>
                </div>
              </div>
              <div class="text-right font-mono">
                <div class="font-bold text-amber-300 tabular-nums">${tool.Loc.amount.toLocaleString()}</div>
                <div class="text-[10px] text-zinc-500">{((tool.Loc.amount / house.purchase_price) * 100).toFixed(1)}% of price</div>
              </div>
            </div>
          {/if}
        {/each}
      </div>
    </div>
  </div>
</div>
