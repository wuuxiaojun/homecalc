<script lang="ts">
  import { appState } from '../../state/appState.svelte';

  let targetTool = $state<'mortgage' | 'loc'>('mortgage');
  let inputMonth = $state<number>(12);
  let inputAmount = $state<number>(10000);

  // Recurring generator state
  let showRecurring = $state(false);
  let recurFrequency = $state<'annual' | 'monthly'>('annual');
  let recurStartMonth = $state<number>(1);
  let recurDurationYears = $state<number>(5);
  let recurAmount = $state<number>(5000);

  const purchase = $derived(appState.activeSlot.purchase);
  const hasMortgage = $derived(purchase.tools.some(t => 'Mortgage' in t));
  const hasLoc = $derived(purchase.tools.some(t => 'Loc' in t));

  // Combined sorted list of repayments
  const repaymentsList = $derived.by(() => {
    const list: { tool: 'mortgage' | 'loc'; month: number; amount: number }[] = [];
    for (const [mStr, amt] of Object.entries(purchase.mortgage_repay)) {
      list.push({ tool: 'mortgage', month: parseInt(mStr, 10), amount: amt });
    }
    for (const [mStr, amt] of Object.entries(purchase.loc_repay)) {
      list.push({ tool: 'loc', month: parseInt(mStr, 10), amount: amt });
    }
    return list.sort((a, b) => a.month - b.month);
  });

  const totalExtraPrincipal = $derived(repaymentsList.reduce((sum, r) => sum + r.amount, 0));

  function handleAddOneTime() {
    if (inputMonth <= 0 || inputAmount <= 0) return;
    appState.addExtraPayment(targetTool, inputMonth, inputAmount);
  }

  function handleApplyRecurring() {
    if (recurAmount <= 0 || recurDurationYears <= 0) return;
    appState.updateActivePurchase((p) => {
      const step = recurFrequency === 'annual' ? 12 : 1;
      const count = recurFrequency === 'annual' ? recurDurationYears : recurDurationYears * 12;
      for (let i = 0; i < count; i++) {
        const m = recurStartMonth + i * step;
        if (m > 360) break;
        if (targetTool === 'mortgage') {
          p.mortgage_repay[m] = (p.mortgage_repay[m] || 0) + recurAmount;
        } else {
          p.loc_repay[m] = (p.loc_repay[m] || 0) + recurAmount;
        }
      }
    });
    showRecurring = false;
  }

  function handleRemove(tool: 'mortgage' | 'loc', month: number) {
    appState.removeExtraPayment(tool, month);
  }

  function handleClearAll() {
    if (confirm('Clear all scheduled extra principal repayments?')) {
      appState.updateActivePurchase((p) => {
        p.mortgage_repay = {};
        p.loc_repay = {};
      });
    }
  }
</script>

<div class="space-y-5">
  <!-- Header info & total extra scheduled -->
  <div class="p-3.5 rounded-xl bg-zinc-900/60 border border-zinc-800/80 flex items-center justify-between">
    <div>
      <div class="text-xs font-semibold text-zinc-200 flex items-center gap-1.5">
        <span>⚡ Scheduled Prepayments</span>
        <span class="px-1.5 py-0.5 rounded text-[10px] font-mono bg-zinc-800 text-zinc-300">
          {repaymentsList.length} rules
        </span>
      </div>
      <div class="text-[11px] text-zinc-400 mt-0.5">Applied directly to loan principal</div>
    </div>
    <div class="text-right">
      <div class="text-sm font-bold font-mono text-emerald-400 tabular-nums">
        ${totalExtraPrincipal.toLocaleString()}
      </div>
      <div class="text-[10px] text-zinc-500 font-mono">Total Extra</div>
    </div>
  </div>

  <!-- Add New Extra Payment Form -->
  <div class="p-3.5 rounded-xl bg-zinc-900/60 border border-zinc-800/80 space-y-3">
    <div class="text-xs font-semibold text-zinc-200">➕ Add Single Prepayment</div>

    <!-- Loan Selector -->
    <div class="grid grid-cols-2 gap-2">
      <button
        class="py-1 text-xs font-medium rounded-lg border transition-all flex items-center justify-center gap-1.5 {targetTool === 'mortgage' ? 'bg-indigo-950/80 border-indigo-600 text-indigo-200 font-semibold' : 'bg-zinc-950 border-zinc-800 text-zinc-400 hover:text-zinc-200'}"
        onclick={() => targetTool = 'mortgage'}
      >
        <span>🏦</span>
        <span>Mortgage</span>
      </button>
      <button
        class="py-1 text-xs font-medium rounded-lg border transition-all flex items-center justify-center gap-1.5 {targetTool === 'loc' ? 'bg-amber-950/80 border-amber-600 text-amber-200 font-semibold' : 'bg-zinc-950 border-zinc-800 text-zinc-400 hover:text-zinc-200'}"
        onclick={() => targetTool = 'loc'}
      >
        <span>💳</span>
        <span>LOC</span>
      </button>
    </div>

    <!-- Month & Amount Inputs -->
    <div class="grid grid-cols-2 gap-3">
      <div>
        <label for="repay-month-input" class="text-[11px] text-zinc-400 block mb-1">Month Number</label>
        <input
          id="repay-month-input"
          type="number"
          min="1"
          max="360"
          class="w-full px-2 py-1 text-right rounded-md bg-zinc-950 border border-zinc-800 text-xs font-mono font-semibold text-zinc-200 tabular-nums focus:border-emerald-500 focus:outline-none"
          bind:value={inputMonth}
        />
      </div>

      <div>
        <label for="repay-amount-input" class="text-[11px] text-zinc-400 block mb-1">Extra Amount ($)</label>
        <input
          id="repay-amount-input"
          type="number"
          step="1000"
          min="100"
          class="w-full px-2 py-1 text-right rounded-md bg-zinc-950 border border-zinc-800 text-xs font-mono font-semibold text-emerald-400 tabular-nums focus:border-emerald-500 focus:outline-none"
          bind:value={inputAmount}
        />
      </div>
    </div>

    <!-- Quick Month / Amount Presets -->
    <div class="flex items-center justify-between text-[10px] font-mono text-zinc-500 pt-1">
      <div class="flex items-center gap-1">
        <span>Month:</span>
        <button class="hover:text-zinc-300" onclick={() => inputMonth = 12}>M12</button>
        <button class="hover:text-zinc-300" onclick={() => inputMonth = 24}>M24</button>
        <button class="hover:text-zinc-300" onclick={() => inputMonth = 60}>M60</button>
      </div>
      <div class="flex items-center gap-1">
        <span>+$:</span>
        <button class="hover:text-zinc-300" onclick={() => inputAmount = 10000}>10k</button>
        <button class="hover:text-zinc-300" onclick={() => inputAmount = 25000}>25k</button>
        <button class="hover:text-zinc-300" onclick={() => inputAmount = 50000}>50k</button>
      </div>
    </div>

    <button
      class="w-full py-1.5 rounded-lg bg-emerald-600 hover:bg-emerald-500 text-white font-medium text-xs shadow-sm transition-colors flex items-center justify-center gap-1.5"
      onclick={handleAddOneTime}
    >
      <span>⚡</span>
      <span>Add Prepayment Rule</span>
    </button>
  </div>

  <!-- Recurring Prepayment Generator Toggle -->
  <div class="p-3.5 rounded-xl bg-zinc-900/40 border border-zinc-800/80 space-y-3">
    <button
      class="w-full flex items-center justify-between text-xs font-semibold text-zinc-300"
      onclick={() => showRecurring = !showRecurring}
    >
      <span class="flex items-center gap-1.5">
        <span>🔄</span>
        <span>Recurring Prepayment Rule</span>
      </span>
      <span class="text-zinc-500 text-xs font-mono">{showRecurring ? '▲ Close' : '▼ Open'}</span>
    </button>

    {#if showRecurring}
      <div class="space-y-3 pt-2 border-t border-zinc-800">
        <div class="grid grid-cols-2 gap-2">
          <div>
            <label for="recur-freq-select" class="text-[11px] text-zinc-400 block mb-1">Frequency</label>
            <select
              id="recur-freq-select"
              class="w-full px-2 py-1 rounded bg-zinc-950 border border-zinc-800 text-xs text-zinc-200 focus:outline-none"
              bind:value={recurFrequency}
            >
              <option value="annual">Every 12 Months</option>
              <option value="monthly">Every Month</option>
            </select>
          </div>

          <div>
            <label for="recur-years-input" class="text-[11px] text-zinc-400 block mb-1">Duration (Years)</label>
            <input
              id="recur-years-input"
              type="number"
              min="1"
              max="30"
              class="w-full px-2 py-1 text-right rounded bg-zinc-950 border border-zinc-800 text-xs font-mono text-zinc-200 tabular-nums focus:outline-none"
              bind:value={recurDurationYears}
            />
          </div>
        </div>

        <div class="grid grid-cols-2 gap-2">
          <div>
            <label for="recur-start-input" class="text-[11px] text-zinc-400 block mb-1">Starting Month</label>
            <input
              id="recur-start-input"
              type="number"
              min="1"
              max="360"
              class="w-full px-2 py-1 text-right rounded bg-zinc-950 border border-zinc-800 text-xs font-mono text-zinc-200 tabular-nums focus:outline-none"
              bind:value={recurStartMonth}
            />
          </div>
          <div>
            <label for="recur-amount-input" class="text-[11px] text-zinc-400 block mb-1">Amount per period ($)</label>
            <input
              id="recur-amount-input"
              type="number"
              step="500"
              min="100"
              class="w-full px-2 py-1 text-right rounded bg-zinc-950 border border-zinc-800 text-xs font-mono text-emerald-400 tabular-nums focus:outline-none"
              bind:value={recurAmount}
            />
          </div>
        </div>

        <button
          class="w-full py-1.5 rounded-lg bg-indigo-600 hover:bg-indigo-500 text-white font-medium text-xs transition-colors"
          onclick={handleApplyRecurring}
        >
          Generate Recurring Prepayments
        </button>
      </div>
    {/if}
  </div>

  <!-- Active Repayments List -->
  {#if repaymentsList.length > 0}
    <div class="space-y-2">
      <div class="flex items-center justify-between text-xs font-semibold text-zinc-300">
        <span>Active Prepayment Schedule</span>
        <button
          class="text-[11px] text-rose-400 hover:text-rose-300 transition-colors"
          onclick={handleClearAll}
        >
          Clear All
        </button>
      </div>

      <div class="max-h-48 overflow-y-auto space-y-1.5 pr-1">
        {#each repaymentsList as item}
          <div class="p-2 rounded-lg bg-zinc-900 border border-zinc-800/80 flex items-center justify-between text-xs">
            <div class="flex items-center gap-2">
              <span class="text-xs">{item.tool === 'mortgage' ? '🏦' : '💳'}</span>
              <span class="font-mono text-zinc-300 font-medium">Month {item.month}</span>
              <span class="text-[10px] text-zinc-500 font-mono">(Yr {Math.ceil(item.month / 12)})</span>
            </div>
            <div class="flex items-center gap-2.5">
              <span class="font-mono font-bold text-emerald-400 tabular-nums">
                +${item.amount.toLocaleString()}
              </span>
              <button
                class="w-5 h-5 rounded hover:bg-zinc-800 text-zinc-500 hover:text-rose-400 flex items-center justify-center transition-colors text-sm"
                onclick={() => handleRemove(item.tool, item.month)}
                title="Delete rule"
              >
                ✕
              </button>
            </div>
          </div>
        {/each}
      </div>
    </div>
  {/if}
</div>
