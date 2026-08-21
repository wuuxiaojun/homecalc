<script lang="ts">
  import { appState } from '../../state/appState.svelte';
  import PropertyInputs from './PropertyInputs.svelte';
  import ToolInputs from './ToolInputs.svelte';
  import RepaymentScheduleEditor from './RepaymentScheduleEditor.svelte';
  import type { SlotId } from '../../state/types';

  let activeTab = $state<'property' | 'tools' | 'repayments'>('property');

  function handleReset() {
    if (confirm(`Reset ${appState.activeSlot.name} to default settings?`)) {
      appState.resetSlot(appState.activeSlotId);
    }
  }

  function handleDuplicate(targetId: SlotId) {
    appState.duplicateSlot(appState.activeSlotId, targetId);
    appState.setActiveSlot(targetId);
  }
</script>

<div class="p-4 lg:p-6 space-y-5">
  <!-- Slot Info & Quick Actions Bar -->
  <div class="flex items-center justify-between pb-3 border-b border-zinc-800/70">
    <div class="flex items-center gap-2">
      <div class="w-2.5 h-2.5 rounded-full bg-emerald-400"></div>
      <span class="text-xs font-mono font-semibold uppercase tracking-wider text-zinc-300">
        Slot {appState.activeSlotId} Active
      </span>
    </div>

    <!-- Actions Dropdown / Buttons -->
    <div class="flex items-center gap-1.5">
      <button
        class="px-2 py-1 text-[11px] font-medium rounded bg-zinc-900 border border-zinc-800 hover:bg-zinc-800 text-zinc-400 hover:text-zinc-200 transition-colors"
        onclick={handleReset}
        title="Reset Slot"
      >
        ↺ Reset
      </button>
      <div class="flex items-center bg-zinc-900 border border-zinc-800 rounded p-0.5 text-[11px] text-zinc-400 font-mono">
        <span class="px-1 text-[10px] text-zinc-500">Copy to:</span>
        {#if appState.activeSlotId !== 1}
          <button class="px-1.5 py-0.5 hover:text-white transition-colors" onclick={() => handleDuplicate(1)}>S1</button>
        {/if}
        {#if appState.activeSlotId !== 2}
          <button class="px-1.5 py-0.5 hover:text-white transition-colors" onclick={() => handleDuplicate(2)}>S2</button>
        {/if}
        {#if appState.activeSlotId !== 3}
          <button class="px-1.5 py-0.5 hover:text-white transition-colors" onclick={() => handleDuplicate(3)}>S3</button>
        {/if}
      </div>
    </div>
  </div>

  <!-- Inner Parameter Tabs -->
  <div class="flex items-center bg-zinc-900/80 p-1 rounded-xl border border-zinc-800/80">
    <button
      class="flex-1 py-1.5 text-xs font-medium rounded-lg transition-all flex items-center justify-center gap-1 {activeTab === 'property' ? 'bg-zinc-800 text-white font-semibold shadow-sm' : 'text-zinc-400 hover:text-zinc-200'}"
      onclick={() => activeTab = 'property'}
    >
      <span>🏡</span>
      <span>Property</span>
    </button>
    <button
      class="flex-1 py-1.5 text-xs font-medium rounded-lg transition-all flex items-center justify-center gap-1 {activeTab === 'tools' ? 'bg-zinc-800 text-white font-semibold shadow-sm' : 'text-zinc-400 hover:text-zinc-200'}"
      onclick={() => activeTab = 'tools'}
    >
      <span>💳</span>
      <span>Financing</span>
    </button>
    <button
      class="flex-1 py-1.5 text-xs font-medium rounded-lg transition-all flex items-center justify-center gap-1 {activeTab === 'repayments' ? 'bg-zinc-800 text-white font-semibold shadow-sm' : 'text-zinc-400 hover:text-zinc-200'}"
      onclick={() => activeTab = 'repayments'}
    >
      <span>⚡</span>
      <span>Prepay</span>
      {#if Object.keys(appState.activeSlot.purchase.mortgage_repay).length > 0 || Object.keys(appState.activeSlot.purchase.loc_repay).length > 0}
        <span class="w-1.5 h-1.5 rounded-full bg-emerald-400"></span>
      {/if}
    </button>
  </div>

  <!-- Tab Content -->
  {#if activeTab === 'property'}
    <PropertyInputs />
  {:else if activeTab === 'tools'}
    <ToolInputs />
  {:else if activeTab === 'repayments'}
    <RepaymentScheduleEditor />
  {/if}
</div>
