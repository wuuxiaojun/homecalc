<script lang="ts">
  import { appState } from '../../state/appState.svelte';
  import { saveCustomScenario } from '../../services/persistence';
  import PropertyInputs from './PropertyInputs.svelte';
  import ToolInputs from './ToolInputs.svelte';
  import RepaymentScheduleEditor from './RepaymentScheduleEditor.svelte';

  let activeTab = $state<'property' | 'tools' | 'repayments'>('property');
  let savedSuccess = $state(false);

  function handleSaveToLibrary() {
    saveCustomScenario(appState.activeSlot.purchase);
    savedSuccess = true;
    setTimeout(() => savedSuccess = false, 2000);
  }
</script>

<div class="p-4 lg:p-6 space-y-5 flex-1 flex flex-col">
  <div class="space-y-5">
    <!-- Slot Info & Quick Actions Bar -->
    <div class="flex items-center justify-between pb-3 border-b border-zinc-800/70">
      <div class="flex items-center gap-2">
        <div class="w-2.5 h-2.5 rounded-full bg-emerald-400"></div>
        <span class="text-xs font-mono font-semibold uppercase tracking-wider text-zinc-300">
          Slot {appState.activeSlotId}
        </span>
      </div>

      <!-- Actions Buttons -->
      <div class="flex items-center gap-1.5">
        <button
          class="px-2.5 py-1 text-[11px] font-medium rounded-lg bg-zinc-900 border border-zinc-800 hover:bg-zinc-800 text-zinc-300 transition-colors shadow-sm"
          onclick={handleSaveToLibrary}
          title="Save to My Scenarios"
        >
          {savedSuccess ? '✓ Saved!' : '💾 Save'}
        </button>
      </div>
    </div>

    <!-- Validation Error Callout (No quick-adjust buttons) -->
    {#if appState.activeSlot.error}
      <div class="p-3.5 rounded-2xl bg-rose-950/80 border border-rose-800/80 text-rose-200 text-xs font-mono flex items-start gap-2.5 shadow-sm">
        <span class="text-base shrink-0">⚠️</span>
        <div class="space-y-1">
          <div class="font-bold text-rose-300 uppercase tracking-wide text-[10px]">Validation Error</div>
          <div class="leading-relaxed">{appState.activeSlot.error}</div>
        </div>
      </div>
    {/if}

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
</div>
