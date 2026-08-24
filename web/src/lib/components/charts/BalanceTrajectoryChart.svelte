<script lang="ts">
  import { appState } from '../../state/appState.svelte';
  import Card from '../common/Card.svelte';

  // Independent toggle state for overlays
  let overlaySlots = $state<Record<number, boolean>>({
    1: false,
    2: true,
    3: true
  });

  let hoveredMonth = $state<number | null>(null);

  const slot = $derived(appState.activeSlot);
  const scenario = $derived(slot.scenario);

  const slot1 = $derived(appState.getSlot(1));
  const slot2 = $derived(appState.getSlot(2));
  const slot3 = $derived(appState.getSlot(3));

  const monthlyData = $derived(scenario?.monthly_statement || []);
  const visibleLimit = 360;

  // Max balance for Y-axis scaling across all active / visible scenarios
  const maxBalance = $derived.by(() => {
    let maxVal = scenario?.purchase.house.purchase_price || 1_000_000;
    if (overlaySlots[1] && slot1?.scenario) maxVal = Math.max(maxVal, slot1.scenario.purchase.house.purchase_price);
    if (overlaySlots[2] && slot2?.scenario) maxVal = Math.max(maxVal, slot2.scenario.purchase.house.purchase_price);
    if (overlaySlots[3] && slot3?.scenario) maxVal = Math.max(maxVal, slot3.scenario.purchase.house.purchase_price);
    return Math.max(maxVal, 100_000);
  });

  // Chart dimensions
  const width = 800;
  const height = 360;
  const padLeft = 70;
  const padRight = 30;
  const padTop = 25;
  const padBottom = 45;

  const chartW = $derived(width - padLeft - padRight);
  const chartH = $derived(height - padTop - padBottom);

  function getX(month: number): number {
    return padLeft + (month / visibleLimit) * chartW;
  }

  function getY(val: number): number {
    return padTop + chartH - (val / maxBalance) * chartH;
  }

  function buildBalancePath(monthly: typeof monthlyData) {
    if (!monthly || monthly.length === 0) return '';
    return monthly
      .map((d, i) => `${i === 0 ? 'M' : 'L'} ${getX(d.month).toFixed(1)} ${getY(d.total_remaining_balance).toFixed(1)}`)
      .join(' ');
  }

  // Active Scenario Path
  const totalBalancePath = $derived(buildBalancePath(monthlyData));

  const totalAreaPath = $derived.by(() => {
    if (monthlyData.length === 0) return '';
    const line = buildBalancePath(monthlyData);
    const lastX = getX(monthlyData[monthlyData.length - 1].month);
    const firstX = getX(monthlyData[0].month);
    const bottomY = padTop + chartH;
    return `${line} L ${lastX} ${bottomY} L ${firstX} ${bottomY} Z`;
  });

  const mortgagePath = $derived.by(() => {
    if (monthlyData.length === 0) return '';
    return monthlyData
      .map((d, i) => `${i === 0 ? 'M' : 'L'} ${getX(d.month).toFixed(1)} ${getY(d.mortgage?.remaining_balance || 0).toFixed(1)}`)
      .join(' ');
  });

  // Overlay Paths for all 3 slots
  const s1Path = $derived(overlaySlots[1] && appState.activeSlotId !== 1 && slot1?.scenario ? buildBalancePath(slot1.scenario.monthly_statement) : '');
  const s2Path = $derived(overlaySlots[2] && appState.activeSlotId !== 2 && slot2?.scenario ? buildBalancePath(slot2.scenario.monthly_statement) : '');
  const s3Path = $derived(overlaySlots[3] && appState.activeSlotId !== 3 && slot3?.scenario ? buildBalancePath(slot3.scenario.monthly_statement) : '');

  // Other slot IDs to render overlay buttons for
  const otherSlotIds = $derived(([1, 2, 3] as const).filter((id): id is 1 | 2 | 3 => id !== appState.activeSlotId));

  function toggleOverlay(slotId: 1 | 2 | 3) {
    overlaySlots[slotId] = !overlaySlots[slotId];
  }

  // Hover data
  const hoveredData = $derived.by(() => {
    if (hoveredMonth === null) return null;
    return monthlyData.find(d => d.month === hoveredMonth) || null;
  });

  function handleMouseMove(e: MouseEvent) {
    const target = e.currentTarget as SVGSVGElement | null;
    if (!target) return;

    // Use native SVG screenCTM inverse matrix for sub-pixel exact coordinate projection
    const ctm = target.getScreenCTM();
    if (!ctm) return;

    const pt = target.createSVGPoint();
    pt.x = e.clientX;
    pt.y = e.clientY;
    const svgPoint = pt.matrixTransform(ctm.inverse());

    const svgX = svgPoint.x;
    const relX = svgX - padLeft;

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

<Card icon="📉" title="Amortization Balance Trajectory">
  {#snippet headerRight()}
    <div class="flex items-center gap-2">
      <!-- Overlay toggle buttons for all other slots -->
      {#each otherSlotIds as otherId}
        {@const otherSlot = appState.getSlot(otherId)}
        {@const isEnabled = overlaySlots[otherId]}
        <button
          class="px-2.5 py-1 text-[11px] font-medium rounded-lg border transition-colors flex items-center gap-1.5 {isEnabled ? (otherId === 2 ? 'bg-indigo-950/80 border-indigo-700 text-indigo-300' : otherId === 3 ? 'bg-amber-950/80 border-amber-700 text-amber-300' : 'bg-emerald-950/80 border-emerald-700 text-emerald-300') : 'bg-zinc-900 border-zinc-800 text-zinc-500 hover:text-zinc-300'}"
          onclick={() => toggleOverlay(otherId)}
          title="Toggle Comparison Overlay for Slot {otherId}"
        >
          <span>⚖️ Overlay S{otherId}</span>
        </button>
      {/each}
    </div>
  {/snippet}

  <!-- Interactive SVG Canvas -->
  <div class="relative w-full overflow-hidden">
    <svg
      role="img"
      aria-label="Amortization trajectory chart"
      viewBox="0 0 {width} {height}"
      class="w-full h-auto max-h-[420px] select-none cursor-crosshair"
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
        {@const x = getX(m)}
        <line x1={x} y1={padTop} x2={x} y2={padTop + chartH} stroke="#27272a" stroke-width="1" stroke-dasharray="4 4" />
        <text x={x} y={padTop + chartH + 20} fill="#71717a" font-size="10" font-family="monospace" text-anchor="middle">
          {m === 0 ? 'M0' : `Yr ${m / 12}`}
        </text>
      {/each}

      <!-- Shaded Area for active scenario -->
      {#if totalAreaPath}
        <path d={totalAreaPath} fill="url(#emeraldArea)" />
      {/if}

      <!-- Slot 1 Overlay Curve (if enabled and not active) -->
      {#if s1Path}
        <path
          d={s1Path}
          fill="none"
          stroke="#34d399"
          stroke-width="2.5"
          stroke-dasharray="6 3"
          stroke-linecap="round"
        />
      {/if}

      <!-- Slot 2 Overlay Curve (if enabled and not active) -->
      {#if s2Path}
        <path
          d={s2Path}
          fill="none"
          stroke="#818cf8"
          stroke-width="2.5"
          stroke-dasharray="6 3"
          stroke-linecap="round"
        />
      {/if}

      <!-- Slot 3 Overlay Curve (if enabled and not active) -->
      {#if s3Path}
        <path
          d={s3Path}
          fill="none"
          stroke="#fbbf24"
          stroke-width="2.5"
          stroke-dasharray="4 3"
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

  <!-- Legend Bar (Direct Scenario Names, Note Removed) -->
  <div class="flex flex-wrap items-center gap-4 text-xs font-mono text-zinc-400 pt-2 border-t border-zinc-800/60">
    <!-- Active Scenario Legend Item -->
    <span class="flex items-center gap-1.5">
      <span class="w-3 h-1 bg-emerald-500 rounded"></span>
      <span class="text-zinc-200 font-semibold">{slot.name}</span>
    </span>

    <!-- Overlay S1 Legend Item -->
    {#if overlaySlots[1] && appState.activeSlotId !== 1 && slot1?.scenario}
      <span class="flex items-center gap-1.5">
        <span class="w-3 h-1 bg-emerald-400 rounded border-dashed"></span>
        <span class="text-emerald-300">{slot1.name}</span>
      </span>
    {/if}

    <!-- Overlay S2 Legend Item -->
    {#if overlaySlots[2] && appState.activeSlotId !== 2 && slot2?.scenario}
      <span class="flex items-center gap-1.5">
        <span class="w-3 h-1 bg-indigo-400 rounded border-dashed"></span>
        <span class="text-indigo-300">{slot2.name}</span>
      </span>
    {/if}

    <!-- Overlay S3 Legend Item -->
    {#if overlaySlots[3] && appState.activeSlotId !== 3 && slot3?.scenario}
      <span class="flex items-center gap-1.5">
        <span class="w-3 h-1 bg-amber-400 rounded border-dashed"></span>
        <span class="text-amber-300">{slot3.name}</span>
      </span>
    {/if}
  </div>
</Card>
