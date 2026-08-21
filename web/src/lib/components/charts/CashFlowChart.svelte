<script lang="ts">
  import { appState } from '../../state/appState.svelte';

  const slot = $derived(appState.activeSlot);
  const yearlyData = $derived(slot.scenario?.yearly_statement || []);

  const maxYearlyPaid = $derived.by(() => {
    if (yearlyData.length === 0) return 100_000;
    const maxVal = Math.max(...yearlyData.map(y => y.annual_paid));
    return Math.max(maxVal * 1.1, 50_000);
  });

  const width = 800;
  const height = 260;
  const padLeft = 65;
  const padRight = 20;
  const padTop = 20;
  const padBottom = 35;

  const chartW = $derived(width - padLeft - padRight);
  const chartH = $derived(height - padTop - padBottom);
  const barWidth = $derived(yearlyData.length > 0 ? (chartW / yearlyData.length) * 0.7 : 16);

  function getBarX(idx: number): number {
    if (yearlyData.length === 0) return padLeft;
    return padLeft + (idx / yearlyData.length) * chartW + (chartW / yearlyData.length - barWidth) / 2;
  }

  function getBarH(val: number): number {
    return Math.max(2, (val / maxYearlyPaid) * chartH);
  }

  function getY(val: number): number {
    return padTop + chartH - getBarH(val);
  }
</script>

<div class="p-5 rounded-2xl bg-zinc-900/60 border border-zinc-800/80 space-y-4">
  <div class="flex items-center justify-between border-b border-zinc-800/60 pb-3">
    <div>
      <h3 class="text-sm font-semibold text-zinc-200 flex items-center gap-2">
        <span>📊</span>
        <span>Annual Cash Outflow Trajectory</span>
      </h3>
      <p class="text-xs text-zinc-500">Year-by-year required payments, prepayments, and holding costs</p>
    </div>
    <div class="text-xs font-mono text-zinc-400">
      {yearlyData.length} Years Horizon
    </div>
  </div>

  <div class="w-full overflow-hidden">
    <svg viewBox="0 0 {width} {height}" class="w-full h-auto select-none">
      <!-- Y-Axis Grid Lines -->
      {#each [0, 0.33, 0.66, 1.0] as frac}
        {@const val = maxYearlyPaid * frac}
        {@const y = padTop + chartH - frac * chartH}
        <line x1={padLeft} y1={y} x2={width - padRight} y2={y} stroke="#27272a" stroke-width="1" stroke-dasharray="3 3" />
        <text x={padLeft - 8} y={y + 4} fill="#71717a" font-size="10" font-family="monospace" text-anchor="end">
          ${(val / 1000).toFixed(0)}k
        </text>
      {/each}

      <!-- Bars -->
      {#each yearlyData as yRow, idx}
        {@const x = getBarX(idx)}
        {@const debtH = getBarH(yRow.annual_debt_paid)}
        {@const extraH = getBarH(yRow.annual_extra_payment)}
        {@const holdingH = getBarH(yRow.annual_holding_cost)}
        {@const totalH = getBarH(yRow.annual_paid)}

        <!-- Stacked Column Bar -->
        <g class="cursor-pointer group">
          <rect
            x={x}
            y={padTop + chartH - totalH}
            width={barWidth}
            height={totalH}
            fill="#6366f1"
            rx="3"
            class="hover:fill-emerald-400 transition-colors opacity-90"
          >
            <title>Year {yRow.year}: Paid ${Math.round(yRow.annual_paid).toLocaleString()} (Interest: ${Math.round(yRow.annual_interest_paid).toLocaleString()}, Tax Savings: -${Math.round(yRow.annual_tax_savings).toLocaleString()})</title>
          </rect>

          <!-- Year Label (every 5 years or 1st/last) -->
          {#if yRow.year === 1 || yRow.year % 5 === 0 || yRow.year === yearlyData.length}
            <text
              x={x + barWidth / 2}
              y={padTop + chartH + 18}
              fill="#a1a1aa"
              font-size="10"
              font-family="monospace"
              text-anchor="middle"
            >
              Y{yRow.year}
            </text>
          {/if}
        </g>
      {/each}
    </svg>
  </div>
</div>
