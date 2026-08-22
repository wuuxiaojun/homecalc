<script lang="ts">
  import { appState } from '../../state/appState.svelte';

  let timeHorizon = $state<'all' | '5y' | '10y' | 'payoff'>('all');
  let showComparisonOverlay = $state(true);
  let hoveredMonth = $state<number | null>(null);

  const slot = $derived(appState.activeSlot);
  const scenario = $derived(slot.scenario);
  const baselineScenario = $derived(appState.getSlot(appState.comparisonBaselineId).scenario);
  const altScenario = $derived(appState.getSlot(appState.comparisonAlternativeId).scenario);

  const monthlyData = $derived(scenario?.monthly_statement || []);
  const maxMonth = $derived(monthlyData.length > 0 ? monthlyData[monthlyData.length - 1].month : 360);

  // Filtered range based on timeHorizon
  const visibleLimit = $derived.by(() => {
    switch (timeHorizon) {
      case '5y': return Math.min(60, maxMonth);
      case '10y': return Math.min(120, maxMonth);
      case 'payoff': return maxMonth;
      case 'all': default: return Math.max(maxMonth, 360);
    }
  });

  const filteredMonthly = $derived(monthlyData.filter(d => d.month <= visibleLimit));

  // Max balance for Y-axis scaling
  const maxBalance = $derived.by(() => {
    const initBal = scenario?.purchase.house.purchase_price || 1_000_000;
    return Math.max(initBal, 100_000);
  });

  // Chart dimensions
  const width = 800;
  const height = 300;
  const padLeft = 70;
  const padRight = 30;
  const padTop = 20;
  const padBottom = 40;

  const chartW = $derived(width - padLeft - padRight);
  const chartH = $derived(height - padTop - padBottom);

  function getX(month: number): number {
    const lim = visibleLimit > 0 ? visibleLimit : 360;
    return padLeft + (month / lim) * chartW;
  }

  function getY(val: number): number {
    return padTop + chartH - (val / maxBalance) * chartH;
  }

  // SVG Path Builders
  const totalBalancePath = $derived.by(() => {
    if (filteredMonthly.length === 0) return '';
    return filteredMonthly
      .map((d, i) => `${i === 0 ? 'M' : 'L'} ${getX(d.month).toFixed(1)} ${getY(d.total_remaining_balance).toFixed(1)}`)
      .join(' ');
  });

  const totalAreaPath = $derived.by(() => {
    if (filteredMonthly.length === 0) return '';
    const line = filteredMonthly
      .map((d, i) => `${i === 0 ? 'M' : 'L'} ${getX(d.month).toFixed(1)} ${getY(d.total_remaining_balance).toFixed(1)}`)
      .join(' ');
    const lastX = getX(filteredMonthly[filteredMonthly.length - 1].month);
    const firstX = getX(filteredMonthly[0].month);
    const bottomY = padTop + chartH;
    return `${line} L ${lastX} ${bottomY} L ${firstX} ${bottomY} Z`;
  });

  const mortgagePath = $derived.by(() => {
    if (filteredMonthly.length === 0) return '';
    return filteredMonthly
      .map((d, i) => `${i === 0 ? 'M' : 'L'} ${getX(d.month).toFixed(1)} ${getY(d.mortgage?.remaining_balance || 0).toFixed(1)}`)
      .join(' ');
  });

  // Comparison Alternative Path
  const altBalancePath = $derived.by(() => {
    if (!showComparisonOverlay || !altScenario) return '';
    const altMonthly = altScenario.monthly_statement.filter(d => d.month <= visibleLimit);
    if (altMonthly.length === 0) return '';
    return altMonthly
      .map((d, i) => `${i === 0 ? 'M' : 'L'} ${getX(d.month).toFixed(1)} ${getY(d.total_remaining_balance).toFixed(1)}`)
      .join(' ');
  });

  // Hover data
  const hoveredData = $derived.by(() => {
    if (hoveredMonth === null) return null;
    return monthlyData.find(d => d.month === hoveredMonth) || null;
  });

  function handleMouseMove(e: MouseEvent) {
    const target = e.currentTarget as SVGElement | null;
    if (!target) return;
    const svgRect = target.getBoundingClientRect();
    const clientX = e.clientX - svgRect.left;
    const relX = clientX - padLeft;
    if (relX < 0 || relX > chartW) {
      hoveredMonth = null;
      return;
    }
    const monthRatio = relX / chartW;
    const exactMonth = Math.round(monthRatio * visibleLimit);
    hoveredMonth = Math.max(0, Math.min(visibleLimit, exactMonth));
  }

  function handleMouseLeave() {
    hoveredMonth = null;
  }
</script>

<div class="p-5 rounded-2xl bg-zinc-900/60 border border-zinc-800/80 space-y-4">
  <!-- Chart Controls & Header -->
  <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3 border-b border-zinc-800/60 pb-3">
    <div>
      <h3 class="text-sm font-semibold text-zinc-200 flex items-center gap-2">
        <span>📉</span>
        <span>Amortization Balance Trajectory</span>
      </h3>
      <p class="text-xs text-zinc-500">Remaining loan principal balance amortization over time</p>
    </div>

    <div class="flex items-center gap-2">
      <!-- Horizon Switcher -->
      <div class="flex items-center bg-zinc-950 p-0.5 rounded-lg border border-zinc-800 text-[11px] font-mono">
        <button
          class="px-2 py-1 rounded transition-colors {timeHorizon === 'all' ? 'bg-zinc-800 text-white font-bold' : 'text-zinc-400 hover:text-zinc-200'}"
          onclick={() => timeHorizon = 'all'}
        >
          30Y
        </button>
        <button
          class="px-2 py-1 rounded transition-colors {timeHorizon === '10y' ? 'bg-zinc-800 text-white font-bold' : 'text-zinc-400 hover:text-zinc-200'}"
          onclick={() => timeHorizon = '10y'}
        >
          10Y
        </button>
        <button
          class="px-2 py-1 rounded transition-colors {timeHorizon === '5y' ? 'bg-zinc-800 text-white font-bold' : 'text-zinc-400 hover:text-zinc-200'}"
          onclick={() => timeHorizon = '5y'}
        >
          5Y
        </button>
        <button
          class="px-2 py-1 rounded transition-colors {timeHorizon === 'payoff' ? 'bg-zinc-800 text-white font-bold' : 'text-zinc-400 hover:text-zinc-200'}"
          onclick={() => timeHorizon = 'payoff'}
        >
          Payoff
        </button>
      </div>

      <!-- Compare toggle -->
      <button
        class="px-2 py-1 text-[11px] font-medium rounded-lg border transition-colors flex items-center gap-1 {showComparisonOverlay ? 'bg-indigo-950/80 border-indigo-700 text-indigo-300' : 'bg-zinc-900 border-zinc-800 text-zinc-500'}"
        onclick={() => showComparisonOverlay = !showComparisonOverlay}
        title="Toggle Comparison Overlay"
      >
        <span>⚖️ Overlay S{appState.comparisonAlternativeId}</span>
      </button>
    </div>
  </div>

  <!-- Interactive SVG Canvas -->
  <div class="relative w-full overflow-hidden">
    <svg
      role="img"
      aria-label="Amortization trajectory chart"
      viewBox="0 0 {width} {height}"
      class="w-full h-auto max-h-[340px] select-none cursor-crosshair"
      onmousemove={handleMouseMove}
      onmouseleave={handleMouseLeave}
    >
      <defs>
        <linearGradient id="emeraldArea" x1="0" y1="0" x2="0" y2="1">
          <stop offset="0%" stop-color="#10b981" stop-opacity="0.2" />
          <stop offset="100%" stop-color="#10b981" stop-opacity="0.0" />
        </linearGradient>
      </defs>

      <!-- Y Grid Lines & Labels -->
      {#each [0, 0.25, 0.5, 0.75, 1.0] as fraction}
        {@const val = maxBalance * fraction}
        {@const y = getY(val)}
        <line x1={padLeft} y1={y} x2={width - padRight} y2={y} stroke="#27272a" stroke-width="1" stroke-dasharray="4 4" />
        <text x={padLeft - 10} y={y + 4} fill="#71717a" font-size="10" font-family="monospace" text-anchor="end">
          ${(val / 1000).toFixed(0)}k
        </text>
      {/each}

      <!-- X Grid Lines & Labels -->
      {#each [0, 60, 120, 180, 240, 300, 360] as m}
        {#if m <= visibleLimit}
          {@const x = getX(m)}
          <line x1={x} y1={padTop} x2={x} y2={padTop + chartH} stroke="#27272a" stroke-width="1" stroke-dasharray="4 4" />
          <text x={x} y={padTop + chartH + 20} fill="#71717a" font-size="10" font-family="monospace" text-anchor="middle">
            {m === 0 ? 'M0' : `Yr ${m / 12}`}
          </text>
        {/if}
      {/each}

      <!-- Shaded Area -->
      {#if totalAreaPath}
        <path d={totalAreaPath} fill="url(#emeraldArea)" />
      {/if}

      <!-- Alternative Comparison Curve (if enabled) -->
      {#if altBalancePath}
        <path
          d={altBalancePath}
          fill="none"
          stroke="#818cf8"
          stroke-width="2.5"
          stroke-dasharray="6 3"
          stroke-linecap="round"
        />
      {/if}

      <!-- Mortgage Component Line -->
      {#if mortgagePath && scenario?.purchase.tools.some(t => 'Loc' in t)}
        <path
          d={mortgagePath}
          fill="none"
          stroke="#6366f1"
          stroke-width="1.5"
          stroke-linecap="round"
          opacity="0.6"
        />
      {/if}

      <!-- Active Total Balance Curve -->
      {#if totalBalancePath}
        <path
          d={totalBalancePath}
          fill="none"
          stroke="#10b981"
          stroke-width="3"
          stroke-linecap="round"
          stroke-linejoin="round"
        />
      {/if}

      <!-- Hover Indicator Crosshair -->
      {#if hoveredMonth !== null}
        {@const hX = getX(hoveredMonth)}
        <line x1={hX} y1={padTop} x2={hX} y2={padTop + chartH} stroke="#a1a1aa" stroke-width="1.5" stroke-dasharray="3 3" />
        {#if hoveredData}
          {@const hY = getY(hoveredData.total_remaining_balance)}
          <circle cx={hX} cy={hY} r="5" fill="#10b981" stroke="#09090b" stroke-width="2" />
        {/if}
      {/if}
    </svg>

    <!-- Floating Hover Tooltip -->
    {#if hoveredData}
      <div
        class="absolute top-3 left-20 bg-zinc-950/95 border border-zinc-700/80 p-3 rounded-xl shadow-2xl text-xs font-mono backdrop-blur-md pointer-events-none space-y-1 z-20"
      >
        <div class="font-bold text-white flex items-center justify-between gap-4 border-b border-zinc-800 pb-1">
          <span>Month {hoveredData.month}</span>
          <span class="text-zinc-400 font-normal">Year {(hoveredData.month / 12).toFixed(1)}</span>
        </div>
        <div class="flex items-center justify-between gap-4 text-emerald-400">
          <span>Total Balance:</span>
          <span class="font-bold tabular-nums">${Math.round(hoveredData.total_remaining_balance).toLocaleString()}</span>
        </div>
        {#if hoveredData.mortgage}
          <div class="flex items-center justify-between gap-4 text-indigo-300 text-[11px]">
            <span>Mortgage:</span>
            <span class="tabular-nums">${Math.round(hoveredData.mortgage.remaining_balance).toLocaleString()}</span>
          </div>
        {/if}
        {#if hoveredData.loc}
          <div class="flex items-center justify-between gap-4 text-amber-300 text-[11px]">
            <span>LOC:</span>
            <span class="tabular-nums">${Math.round(hoveredData.loc.remaining_balance).toLocaleString()}</span>
          </div>
        {/if}
        {#if hoveredData.total_extra_payment > 0}
          <div class="flex items-center justify-between gap-4 text-amber-400 font-bold text-[11px] pt-0.5 border-t border-zinc-800">
            <span>Extra Paid:</span>
            <span class="tabular-nums">+${Math.round(hoveredData.total_extra_payment).toLocaleString()}</span>
          </div>
        {/if}
      </div>
    {/if}
  </div>

  <!-- Legend Bar -->
  <div class="flex flex-wrap items-center justify-between text-xs font-mono text-zinc-400 pt-2 border-t border-zinc-800/60">
    <div class="flex items-center gap-4">
      <span class="flex items-center gap-1.5">
        <span class="w-3 h-1 bg-emerald-500 rounded"></span>
        <span class="text-zinc-200">Active ({slot.name})</span>
      </span>
      {#if showComparisonOverlay && altScenario}
        <span class="flex items-center gap-1.5">
          <span class="w-3 h-1 bg-indigo-400 rounded border-dashed"></span>
          <span class="text-indigo-300">Alt ({appState.getSlot(appState.comparisonAlternativeId).name})</span>
        </span>
      {/if}
    </div>
    <div class="text-[11px] text-zinc-500">
      Hover crosshair to inspect month balance
    </div>
  </div>
</div>
