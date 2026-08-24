<script lang="ts">
  import { appState } from '../../state/appState.svelte';
  import Card from '../common/Card.svelte';

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

<Card icon="📊" title="Annual Cash Outflow Trajectory">
  {#snippet headerRight()}
    <span class="text-xs font-mono text-zinc-400">
      {yearlyData.length} Years Horizon
    </span>
  {/snippet}

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
      {#each yearlyData as d, idx}
        {@const x = getBarX(idx)}
        {@const h = getBarH(d.annual_paid)}
        {@const y = getY(d.annual_paid)}
        {@const isLast = idx === yearlyData.length - 1}

        <!-- Bar rectangle -->
        <rect
          x={x}
          y={y}
          width={barWidth}
          height={h}
          rx="3"
          fill={isLast ? '#10b981' : '#6366f1'}
          opacity="0.85"
          class="hover:opacity-100 transition-opacity cursor-pointer"
        >
          <title>Year {d.year}: ${Math.round(d.annual_paid).toLocaleString()} paid</title>
        </rect>

        <!-- X Label (Every 5 years or first/last) -->
        {#if d.year === 1 || d.year % 5 === 0 || isLast}
          <text
            x={x + barWidth / 2}
            y={height - 10}
            fill="#71717a"
            font-size="10"
            font-family="monospace"
            text-anchor="middle"
          >
            Y{d.year}
          </text>
        {/if}
      {/each}
    </svg>
  </div>
</Card>
