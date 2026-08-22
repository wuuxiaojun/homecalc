<script lang="ts">
  import BalanceTrajectoryChart from './BalanceTrajectoryChart.svelte';
  import CostBreakdownChart from './CostBreakdownChart.svelte';
  import CashFlowChart from './CashFlowChart.svelte';
</script>

<div class="space-y-6">
  <!-- Top: Amortization Balance Trajectory Curve -->
  <BalanceTrajectoryChart />

  <!-- Bottom: Cost Breakdown & Cash Flow Columns -->
  <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
    <CostBreakdownChart />
    <CashFlowChart />
  </div>
</div>
