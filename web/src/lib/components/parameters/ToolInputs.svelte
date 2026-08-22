<script lang="ts">
  import { appState } from '../../state/appState.svelte';
  import type { Cash, Loc, Mortgage, Tool } from '../../state/types';
  import Card from '../common/Card.svelte';

  const purchase = $derived(appState.activeSlot.purchase);
  const housePrice = $derived(purchase?.house.purchase_price || 0);

  // Extract tools
  const cashTool = $derived(purchase?.tools.find(t => 'Cash' in t)?.Cash as Cash | undefined);
  const mortgageTool = $derived(purchase?.tools.find(t => 'Mortgage' in t)?.Mortgage as Mortgage | undefined);
  const locTool = $derived(purchase?.tools.find(t => 'Loc' in t)?.Loc as Loc | undefined);

  const cashAmount = $derived(cashTool?.amount || 0);
  const mortgageAmount = $derived(mortgageTool?.amount || 0);
  const locAmount = $derived(locTool?.amount || 0);
  const totalBorrowed = $derived(mortgageAmount + locAmount);
  const totalFunding = $derived(cashAmount + totalBorrowed);
  const isOverAllocated = $derived(totalFunding > housePrice + 1);

  // Ratios
  const cashPercent = $derived(housePrice > 0 ? (cashAmount / housePrice) * 100 : 0);
  const mortgagePercent = $derived(housePrice > 0 ? (mortgageAmount / housePrice) * 100 : 0);
  const locPercent = $derived(housePrice > 0 ? (locAmount / housePrice) * 100 : 0);

  // Estimated Mortgage PMT preview
  const estimatedMortgagePmt = $derived.by(() => {
    if (!mortgageTool || mortgageTool.amount <= 0 || mortgageTool.term <= 0) return 0;
    const r = (mortgageTool.rate * 0.01) / 12;
    const n = mortgageTool.term * 12;
    if (r <= 0) return mortgageTool.amount / n;
    const factor = Math.pow(1 + r, n);
    return mortgageTool.amount * (r * factor) / (factor - 1);
  });

  // Estimated LOC Interest-only preview
  const estimatedLocPmt = $derived.by(() => {
    if (!locTool || locTool.amount <= 0) return 0;
    return locTool.amount * (locTool.rate * 0.01) / 12;
  });

  // Updaters
  function updateCashRate(rate: number) {
    appState.updateActivePurchase((p) => {
      let c = p.tools.find(t => 'Cash' in t);
      if (c && 'Cash' in c && c.Cash) {
        c.Cash.rate = Math.max(0, rate);
      } else {
        p.tools.push({ Cash: { amount: Math.max(0, housePrice - totalBorrowed), rate } });
      }
    });
  }

  function updateMortgage(updater: (m: Mortgage) => void) {
    appState.updateActivePurchase((p) => {
      let m = p.tools.find(t => 'Mortgage' in t);
      if (!m || !('Mortgage' in m) || !m.Mortgage) {
        const newM: Mortgage = { amount: 0, rate: 6.5, term: 30 };
        updater(newM);
        p.tools.push({ Mortgage: newM });
      } else {
        updater(m.Mortgage);
      }
      // Rebalance cash
      rebalanceCashInPurchase(p);
    });
  }

  function updateLoc(updater: (l: Loc) => void) {
    appState.updateActivePurchase((p) => {
      let l = p.tools.find(t => 'Loc' in t);
      if (!l || !('Loc' in l) || !l.Loc) {
        const newL: Loc = { amount: 0, rate: 7.0 };
        updater(newL);
        p.tools.push({ Loc: newL });
      } else {
        updater(l.Loc);
      }
      // Rebalance cash
      rebalanceCashInPurchase(p);
    });
  }

  function toggleTool(type: 'Mortgage' | 'Loc') {
    appState.updateActivePurchase((p) => {
      const idx = p.tools.findIndex(t => type in t);
      if (idx >= 0) {
        p.tools.splice(idx, 1);
      } else {
        if (type === 'Mortgage') {
          p.tools.push({ Mortgage: { amount: Math.max(0, p.house.purchase_price * 0.8), rate: 6.5, term: 30 } });
        } else {
          p.tools.push({ Loc: { amount: Math.max(0, p.house.purchase_price * 0.2), rate: 7.0 } });
        }
      }
      rebalanceCashInPurchase(p);
    });
  }

  function setLtvPreset(ltvPercent: number) {
    appState.updateActivePurchase((p) => {
      const targetMortgage = Math.round(p.house.purchase_price * (ltvPercent / 100));
      let m = p.tools.find(t => 'Mortgage' in t);
      if (m && 'Mortgage' in m && m.Mortgage) {
        m.Mortgage.amount = targetMortgage;
      } else if (targetMortgage > 0) {
        p.tools.push({ Mortgage: { amount: targetMortgage, rate: 6.5, term: 30 } });
      }
      const locIdx = p.tools.findIndex(t => 'Loc' in t);
      if (locIdx >= 0 && ltvPercent >= 100) {
        p.tools.splice(locIdx, 1);
      }
      rebalanceCashInPurchase(p);
    });
  }

  function rebalanceCashInPurchase(p: typeof purchase) {
    if (!p) return;
    const mort = p.tools.find(t => 'Mortgage' in t)?.Mortgage?.amount || 0;
    const loc = p.tools.find(t => 'Loc' in t)?.Loc?.amount || 0;
    const neededCash = Math.max(0, p.house.purchase_price - (mort + loc));

    let cashIdx = p.tools.findIndex(t => 'Cash' in t);
    if (cashIdx >= 0) {
      const c = p.tools[cashIdx];
      if ('Cash' in c && c.Cash) {
        c.Cash.amount = neededCash;
      }
    } else {
      p.tools.push({ Cash: { amount: neededCash, rate: 4.0 } });
    }
  }
</script>

{#if purchase}
  <div class="space-y-4">
    <!-- Capital Structure Segmented Bar Card -->
    <Card icon="📊" title="Capital Structure">
      {#snippet headerRight()}
        <span class="font-mono text-zinc-400 font-medium text-xs">Total: ${totalFunding.toLocaleString()}</span>
      {/snippet}

      <!-- Visual Stacked Bar -->
      <div class="w-full h-3 rounded-full bg-zinc-800 overflow-hidden flex">
        {#if cashPercent > 0}
          <div
            class="h-full bg-emerald-500 transition-all duration-300 relative group"
            style="width: {Math.min(100, cashPercent)}%"
            title="Cash Down: {cashPercent.toFixed(1)}%"
          ></div>
        {/if}
        {#if mortgagePercent > 0}
          <div
            class="h-full bg-indigo-500 transition-all duration-300 relative group"
            style="width: {Math.min(100, mortgagePercent)}%"
            title="Mortgage: {mortgagePercent.toFixed(1)}%"
          ></div>
        {/if}
        {#if locPercent > 0}
          <div
            class="h-full bg-amber-500 transition-all duration-300 relative group"
            style="width: {Math.min(100, locPercent)}%"
            title="LOC: {locPercent.toFixed(1)}%"
          ></div>
        {/if}
      </div>

      <!-- Legend & LTV quick tags -->
      <div class="flex items-center justify-between text-[11px] font-mono text-zinc-400 pt-1">
        <div class="flex items-center gap-3">
          <span class="flex items-center gap-1"><span class="w-2 h-2 rounded-full bg-emerald-400"></span> Cash {cashPercent.toFixed(0)}%</span>
          <span class="flex items-center gap-1"><span class="w-2 h-2 rounded-full bg-indigo-400"></span> Mort {mortgagePercent.toFixed(0)}%</span>
          {#if locPercent > 0}
            <span class="flex items-center gap-1"><span class="w-2 h-2 rounded-full bg-amber-400"></span> LOC {locPercent.toFixed(0)}%</span>
          {/if}
        </div>
        <div class="flex items-center gap-1">
          <button
            class="px-1.5 py-0.5 rounded bg-zinc-800 hover:bg-zinc-700 text-[10px] text-zinc-300 transition-colors"
            onclick={() => setLtvPreset(80)}
          >
            80% LTV
          </button>
          <button
            class="px-1.5 py-0.5 rounded bg-zinc-800 hover:bg-zinc-700 text-[10px] text-zinc-300 transition-colors"
            onclick={() => setLtvPreset(100)}
          >
            All Loan
          </button>
          <button
            class="px-1.5 py-0.5 rounded bg-zinc-800 hover:bg-zinc-700 text-[10px] text-zinc-300 transition-colors"
            onclick={() => setLtvPreset(0)}
          >
            All Cash
          </button>
        </div>
      </div>

      {#if isOverAllocated}
        <div class="p-2 rounded-lg bg-rose-950/80 border border-rose-800/80 text-[11px] text-rose-300 font-mono">
          ⚠️ Total borrowed (${totalBorrowed.toLocaleString()}) exceeds purchase price (${housePrice.toLocaleString()}).
        </div>
      {/if}
    </Card>

    <!-- 1. Cash Down Payment Card -->
    <Card icon="💵" title="Cash Down Payment">
      {#snippet headerRight()}
        <div class="text-sm font-mono font-bold text-emerald-400 tabular-nums">
          ${cashAmount.toLocaleString()}
        </div>
      {/snippet}

      <div class="flex items-center justify-between text-xs">
        <label for="cash-yield-input" class="text-zinc-400 text-[11px]">Annual Cash Yield / Opportunity Cost</label>
        <div class="flex items-center gap-1">
          <input
            id="cash-yield-input"
            type="number"
            step="0.1"
            min="0"
            max="15"
            class="w-16 px-1.5 py-0.5 text-right rounded bg-zinc-950 border border-zinc-800 text-xs font-mono font-semibold text-zinc-200 tabular-nums focus:border-emerald-500 focus:outline-none"
            value={cashTool?.rate || 4.0}
            oninput={(e) => updateCashRate(parseFloat(e.currentTarget.value) || 0)}
          />
          <span class="text-xs font-mono text-zinc-400">%</span>
        </div>
      </div>
    </Card>

    <!-- 2. Mortgage Loan Instrument Card -->
    <Card icon="🏦" title="Mortgage Loan">
      {#snippet headerRight()}
        <button
          type="button"
          role="switch"
          aria-checked={!!mortgageTool}
          aria-label="Toggle Mortgage Loan"
          class="relative inline-flex h-5 w-9 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 focus:ring-offset-zinc-900 {mortgageTool ? 'bg-indigo-600' : 'bg-zinc-700'}"
          onclick={() => toggleTool('Mortgage')}
        >
          <span
            class="pointer-events-none inline-block h-4 w-4 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out {mortgageTool ? 'translate-x-4' : 'translate-x-0'}"
          ></span>
        </button>
      {/snippet}

      {#if mortgageTool}
        <!-- Principal Amount -->
        <div class="space-y-1.5">
          <div class="flex items-center justify-between text-xs">
            <label for="mortgage-principal-input" class="text-zinc-400">Loan Principal</label>
            <div class="flex items-center gap-1">
              <span class="text-xs font-mono text-zinc-500">$</span>
              <input
                id="mortgage-principal-input"
                type="number"
                step="10000"
                min="0"
                class="w-28 px-1.5 py-0.5 text-right rounded bg-zinc-950 border border-zinc-800 text-xs font-mono font-bold text-indigo-300 tabular-nums focus:border-indigo-500 focus:outline-none"
                value={mortgageTool.amount}
                oninput={(e) => updateMortgage(m => m.amount = Math.max(0, parseFloat(e.currentTarget.value) || 0))}
              />
            </div>
          </div>
          <input
            type="range"
            min="0"
            max={housePrice}
            step="10000"
            aria-label="Mortgage Loan Principal Slider"
            class="w-full h-1.5 bg-zinc-800 rounded-lg appearance-none cursor-pointer accent-indigo-500"
            value={mortgageTool.amount}
            oninput={(e) => updateMortgage(m => m.amount = Math.max(0, parseFloat(e.currentTarget.value) || 0))}
          />
        </div>

        <!-- Rate & Term Grid -->
        <div class="grid grid-cols-2 gap-3 pt-1">
          <!-- Interest Rate -->
          <div class="space-y-1">
            <label for="mortgage-rate-input" class="text-[11px] text-zinc-400 block">Interest Rate</label>
            <div class="flex items-center gap-1">
              <input
                id="mortgage-rate-input"
                type="number"
                step="0.05"
                min="0"
                max="20"
                class="w-full px-2 py-1 text-right rounded bg-zinc-950 border border-zinc-800 text-xs font-mono font-semibold text-zinc-200 tabular-nums focus:border-indigo-500 focus:outline-none"
                value={mortgageTool.rate}
                oninput={(e) => updateMortgage(m => m.rate = Math.max(0, parseFloat(e.currentTarget.value) || 0))}
              />
              <span class="text-xs font-mono text-zinc-500">%</span>
            </div>
          </div>

          <!-- Term Selector -->
          <div class="space-y-1">
            <span class="text-[11px] text-zinc-400 block">Term (Years)</span>
            <div class="flex items-center gap-1">
              <button
                class="flex-1 py-1 text-xs font-mono rounded transition-colors {mortgageTool.term === 15 ? 'bg-indigo-600 text-white font-bold' : 'bg-zinc-800 text-zinc-400 hover:text-zinc-200'}"
                onclick={() => updateMortgage(m => m.term = 15)}
              >
                15y
              </button>
              <button
                class="flex-1 py-1 text-xs font-mono rounded transition-colors {mortgageTool.term === 30 ? 'bg-indigo-600 text-white font-bold' : 'bg-zinc-800 text-zinc-400 hover:text-zinc-200'}"
                onclick={() => updateMortgage(m => m.term = 30)}
              >
                30y
              </button>
            </div>
          </div>
        </div>

        <!-- Monthly PMT Pill -->
        <div class="p-2.5 rounded-lg bg-indigo-950/40 border border-indigo-800/40 flex items-center justify-between text-xs">
          <span class="text-indigo-300 font-medium">Estimated Monthly PMT</span>
          <span class="font-mono font-bold text-indigo-200 tabular-nums">
            ${Math.round(estimatedMortgagePmt).toLocaleString()} /mo
          </span>
        </div>
      {/if}
    </Card>

    <!-- 3. Line of Credit (LOC) Instrument Card -->
    <Card icon="💳" title="Line of Credit (LOC)">
      {#snippet headerRight()}
        <button
          type="button"
          role="switch"
          aria-checked={!!locTool}
          aria-label="Toggle Line of Credit"
          class="relative inline-flex h-5 w-9 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-amber-500 focus:ring-offset-2 focus:ring-offset-zinc-900 {locTool ? 'bg-amber-500' : 'bg-zinc-700'}"
          onclick={() => toggleTool('Loc')}
        >
          <span
            class="pointer-events-none inline-block h-4 w-4 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out {locTool ? 'translate-x-4' : 'translate-x-0'}"
          ></span>
        </button>
      {/snippet}

      {#if locTool}
        <!-- Principal Amount -->
        <div class="space-y-1.5">
          <div class="flex items-center justify-between text-xs">
            <label for="loc-amount-input" class="text-zinc-400">LOC Credit Amount</label>
            <div class="flex items-center gap-1">
              <span class="text-xs font-mono text-zinc-500">$</span>
              <input
                id="loc-amount-input"
                type="number"
                step="10000"
                min="0"
                class="w-28 px-1.5 py-0.5 text-right rounded bg-zinc-950 border border-zinc-800 text-xs font-mono font-bold text-amber-300 tabular-nums focus:border-amber-500 focus:outline-none"
                value={locTool.amount}
                oninput={(e) => updateLoc(l => l.amount = Math.max(0, parseFloat(e.currentTarget.value) || 0))}
              />
            </div>
          </div>
          <input
            type="range"
            min="0"
            max={housePrice}
            step="10000"
            aria-label="LOC Credit Amount Slider"
            class="w-full h-1.5 bg-zinc-800 rounded-lg appearance-none cursor-pointer accent-amber-500"
            value={locTool.amount}
            oninput={(e) => updateLoc(l => l.amount = Math.max(0, parseFloat(e.currentTarget.value) || 0))}
          />
        </div>

        <!-- Rate -->
        <div class="flex items-center justify-between text-xs pt-1">
          <label for="loc-rate-input" class="text-zinc-400 text-[11px]">Interest Rate (Interest-Only)</label>
          <div class="flex items-center gap-1">
            <input
              id="loc-rate-input"
              type="number"
              step="0.1"
              min="0"
              max="20"
              class="w-16 px-1.5 py-0.5 text-right rounded bg-zinc-950 border border-zinc-800 text-xs font-mono font-semibold text-zinc-200 tabular-nums focus:border-amber-500 focus:outline-none"
              value={locTool.rate}
              oninput={(e) => updateLoc(l => l.rate = Math.max(0, parseFloat(e.currentTarget.value) || 0))}
            />
            <span class="text-xs font-mono text-zinc-500">%</span>
          </div>
        </div>

        <!-- Monthly Interest-only Pill -->
        <div class="p-2.5 rounded-lg bg-amber-950/40 border border-amber-800/40 flex items-center justify-between text-xs">
          <span class="text-amber-300 font-medium">Monthly Interest Payment</span>
          <span class="font-mono font-bold text-amber-200 tabular-nums">
            ${Math.round(estimatedLocPmt).toLocaleString()} /mo
          </span>
        </div>
      {/if}
    </Card>
  </div>
{/if}
