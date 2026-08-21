<script lang="ts">
  import { appState } from '../../state/appState.svelte';
  import BalanceTrajectoryChart from './BalanceTrajectoryChart.svelte';
  import CostBreakdownChart from './CostBreakdownChart.svelte';
  import CashFlowChart from './CashFlowChart.svelte';

  const slot = $derived(appState.activeSlot);
</script>

<div class="space-y-6">
  {#if !slot.purchase || !slot.scenario}
    <div class="p-12 rounded-2xl bg-zinc-900/40 border border-zinc-800/80 text-center space-y-3">
      <div class="text-3xl">📈</div>
      <h3 class="text-sm font-bold text-white">No Scenario to Analyze in Slot {slot.id}</h3>
      <p class="text-xs text-zinc-400 max-w-sm mx-auto">
        Create or load a purchase scenario into this slot to view amortization curves and cash flow trajectories.
      </p>
      <button
        class="mt-2 py-2 px-4 rounded-xl bg-emerald-600 hover:bg-emerald-500 text-white text-xs font-semibold transition-colors"
        onclick={() => appState.createScenarioInSlot(slot.id)}
      >
        ✨ Create Scenario in Slot {slot.id}
      </button>
    </div>
  {:else}
    <!-- Top: Amortization Balance Trajectory Curve -->
    <BalanceTrajectoryChart />

    <!-- Bottom: Cost Breakdown & Cash Flow Columns -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <CostBreakdownChart />
      <CashFlowChart />
    </div>
  {/if}
</div>
