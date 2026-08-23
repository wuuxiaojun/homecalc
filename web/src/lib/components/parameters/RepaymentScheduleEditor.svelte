<script lang="ts">
  import { appState } from '../../state/appState.svelte';
  import Card from '../common/Card.svelte';

  let targetTool = $state<'mortgage' | 'loc'>('mortgage');
  let inputMonth = $state<number>(12);
  let inputAmount = $state<number>(10000);

  const purchase = $derived(appState.activeSlot.purchase);

  const mortPrincipal = $derived(purchase?.tools.find(t => 'Mortgage' in t)?.Mortgage?.amount || 0);
  const locPrincipal = $derived(purchase?.tools.find(t => 'Loc' in t)?.Loc?.amount || 0);
  const activePrincipal = $derived(targetTool === 'mortgage' ? mortPrincipal : locPrincipal);

  // Combined sorted list of repayments
  const repaymentsList = $derived.by(() => {
    if (!purchase) return [];
    const list: { tool: 'mortgage' | 'loc'; month: number; amount: number }[] = [];
    for (const [mStr, amt] of Object.entries(purchase.mortgage_repay || {})) {
      list.push({ tool: 'mortgage', month: parseInt(mStr, 10), amount: amt });
    }
    for (const [mStr, amt] of Object.entries(purchase.loc_repay || {})) {
      list.push({ tool: 'loc', month: parseInt(mStr, 10), amount: amt });
    }
    return list.sort((a, b) => a.month - b.month);
  });

  const totalExtraPrincipal = $derived(repaymentsList.reduce((sum, r) => sum + r.amount, 0));

  function handleAddOneTime() {
    const validMonth = Math.max(1, Math.min(360, inputMonth || 1));
    const maxAmt = Math.max(1, activePrincipal);
    const validAmount = Math.max(1, Math.min(maxAmt, inputAmount || 1));
    if (validAmount <= 0) return;
    appState.addExtraPayment(targetTool, validMonth, validAmount);
  }

  function handleRemove(tool: 'mortgage' | 'loc', month: number) {
    appState.removeExtraPayment(tool, month);
  }

  function handleClearAll() {
    if (confirm('Clear all scheduled extra principal prepayments?')) {
      appState.updateActivePurchase((p) => {
        p.mortgage_repay = {};
        p.loc_repay = {};
      });
    }
  }
</script>

{#if purchase}
  <div class="space-y-4">
    <!-- Header summary Card -->
    <Card icon="⚡" title="Scheduled Prepayments">
      {#snippet headerRight()}
        <span class="px-1.5 py-0.5 rounded text-[10px] font-mono bg-zinc-800 text-zinc-300">
          {repaymentsList.length} scheduled
        </span>
      {/snippet}

      <div class="flex items-center justify-between">
        <span class="text-xs text-zinc-400">Applied directly to principal</span>
        <span class="text-sm font-bold font-mono text-emerald-400 tabular-nums">
          ${totalExtraPrincipal.toLocaleString()}
        </span>
      </div>
    </Card>

    <!-- Add New Extra Payment Card -->
    <Card icon="➕" title="Add Extra Principal Prepayment">
      <!-- Loan Selector -->
      <div class="grid grid-cols-2 gap-2">
        <button
          class="py-1.5 text-xs font-medium rounded-lg border transition-all flex items-center justify-center gap-1.5 {targetTool === 'mortgage' ? 'bg-indigo-950/80 border-indigo-600 text-indigo-200 font-semibold shadow-sm' : 'bg-zinc-950 border-zinc-800 text-zinc-400 hover:text-zinc-200'}"
          onclick={() => targetTool = 'mortgage'}
        >
          <span>🏦</span>
          <span>Mortgage</span>
        </button>
        <button
          class="py-1.5 text-xs font-medium rounded-lg border transition-all flex items-center justify-center gap-1.5 {targetTool === 'loc' ? 'bg-amber-950/80 border-amber-600 text-amber-200 font-semibold shadow-sm' : 'bg-zinc-950 border-zinc-800 text-zinc-400 hover:text-zinc-200'}"
          onclick={() => targetTool = 'loc'}
        >
          <span>💳</span>
          <span>LOC</span>
        </button>
      </div>

      <!-- Month & Amount Inputs -->
      <div class="grid grid-cols-2 gap-3">
        <div>
          <label for="repay-month-input" class="text-[11px] text-zinc-400 block mb-1">Month (1-360)</label>
          <input
            id="repay-month-input"
            type="number"
            min="1"
            max="360"
            class="w-full px-2.5 py-1.5 text-right rounded-lg bg-zinc-950 border border-zinc-800 text-xs font-mono font-semibold text-zinc-200 tabular-nums focus:border-emerald-500 focus:outline-none"
            value={inputMonth}
            oninput={(e) => {
              const val = parseInt(e.currentTarget.value, 10);
              if (!isNaN(val)) inputMonth = val;
            }}
            onblur={(e) => {
              const val = parseInt(e.currentTarget.value, 10);
              inputMonth = isNaN(val) ? 1 : Math.max(1, Math.min(360, val));
              e.currentTarget.value = String(inputMonth);
            }}
          />
        </div>

        <div>
          <label for="repay-amount-input" class="text-[11px] text-zinc-400 block mb-1">Extra Amount ($)</label>
          <input
            id="repay-amount-input"
            type="number"
            step="1000"
            min="1"
            max={Math.max(1, activePrincipal)}
            class="w-full px-2.5 py-1.5 text-right rounded-lg bg-zinc-950 border border-zinc-800 text-xs font-mono font-semibold text-emerald-400 tabular-nums focus:border-emerald-500 focus:outline-none"
            value={inputAmount}
            oninput={(e) => {
              const val = parseFloat(e.currentTarget.value);
              if (!isNaN(val)) inputAmount = val;
            }}
            onblur={(e) => {
              const val = parseFloat(e.currentTarget.value);
              const maxAmt = Math.max(1, activePrincipal);
              inputAmount = isNaN(val) ? 1 : Math.max(1, Math.min(maxAmt, val));
              e.currentTarget.value = String(inputAmount);
            }}
          />
        </div>
      </div>

      <button
        class="w-full py-2 rounded-lg bg-emerald-600 hover:bg-emerald-500 text-white font-semibold text-xs shadow-sm transition-colors flex items-center justify-center gap-1.5"
        onclick={handleAddOneTime}
      >
        <span>⚡</span>
        <span>Add Prepayment Entry</span>
      </button>
    </Card>

    <!-- Active Repayments List Card -->
    {#if repaymentsList.length > 0}
      <Card icon="📋" title="Active Schedule ({repaymentsList.length})">
        {#snippet headerRight()}
          <button
            class="text-[11px] text-rose-400 hover:text-rose-300 transition-colors font-medium"
            onclick={handleClearAll}
          >
            Clear All
          </button>
        {/snippet}

        <div class="max-h-56 overflow-y-auto space-y-1.5 pr-1">
          {#each repaymentsList as item}
            <div class="p-2.5 rounded-lg bg-zinc-950/70 border border-zinc-800/80 flex items-center justify-between text-xs">
              <div class="flex items-center gap-2">
                <span class="text-sm">{item.tool === 'mortgage' ? '🏦' : '💳'}</span>
                <span class="font-mono text-zinc-200 font-semibold">Month {item.month}</span>
                <span class="text-[10px] text-zinc-500 font-mono">(Yr {Math.ceil(item.month / 12)})</span>
              </div>
              <div class="flex items-center gap-2.5">
                <span class="font-mono font-bold text-emerald-400 tabular-nums">
                  +${item.amount.toLocaleString()}
                </span>
                <button
                  class="w-6 h-6 rounded hover:bg-zinc-800 text-zinc-500 hover:text-rose-400 flex items-center justify-center transition-colors text-sm"
                  onclick={() => handleRemove(item.tool, item.month)}
                  title="Delete entry"
                >
                  ✕
                </button>
              </div>
            </div>
          {/each}
        </div>
      </Card>
    {/if}
  </div>
{/if}
