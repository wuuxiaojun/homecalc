<script lang="ts">
  import { appState } from '../../state/appState.svelte';
  import { saveCustomScenario } from '../../services/persistence';
  import PropertyInputs from './PropertyInputs.svelte';
  import ToolInputs from './ToolInputs.svelte';
  import RepaymentScheduleEditor from './RepaymentScheduleEditor.svelte';
  import type { SlotId } from '../../state/types';

  let { onOpenLibrary, onOpenImport }: { onOpenLibrary?: () => void; onOpenImport?: () => void } = $props();

  let activeTab = $state<'property' | 'tools' | 'repayments'>('property');
  let savedSuccess = $state(false);

  function handleClear() {
    if (confirm(`Clear and remove scenario from Slot ${appState.activeSlotId}?`)) {
      appState.clearSlot(appState.activeSlotId);
    }
  }

  function handleSaveToLibrary() {
    if (!appState.activeSlot.purchase) return;
    saveCustomScenario(appState.activeSlot.purchase);
    savedSuccess = true;
    setTimeout(() => savedSuccess = false, 2000);
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
      <div class="w-2.5 h-2.5 rounded-full {appState.activeSlot.purchase ? 'bg-emerald-400' : 'bg-zinc-600'}"></div>
      <span class="text-xs font-mono font-semibold uppercase tracking-wider text-zinc-300">
        Slot {appState.activeSlotId} {appState.activeSlot.purchase ? 'Active' : '(Empty)'}
      </span>
    </div>

    <!-- Actions Dropdown / Buttons if populated -->
    {#if appState.activeSlot.purchase}
      <div class="flex items-center gap-1.5">
        <button
          class="px-2 py-1 text-[11px] font-medium rounded bg-zinc-900 border border-zinc-800 hover:bg-zinc-800 text-zinc-300 transition-colors"
          onclick={handleSaveToLibrary}
          title="Save to My Scenarios"
        >
          {savedSuccess ? '✓ Saved!' : '💾 Save'}
        </button>

        <button
          class="px-2 py-1 text-[11px] font-medium rounded bg-zinc-900 border border-zinc-800 hover:bg-rose-950/60 hover:border-rose-800 text-zinc-400 hover:text-rose-300 transition-colors"
          onclick={handleClear}
          title="Clear Slot"
        >
          🗑️ Clear
        </button>

        <div class="flex items-center bg-zinc-900 border border-zinc-800 rounded p-0.5 text-[11px] text-zinc-400 font-mono">
          <span class="px-1 text-[10px] text-zinc-500">Copy:</span>
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
    {/if}
  </div>

  {#if !appState.activeSlot.purchase}
    <!-- Empty Slot Welcome / Builder Card -->
    <div class="p-6 rounded-2xl bg-zinc-900/40 border border-zinc-800/80 text-center space-y-4">
      <div class="w-12 h-12 rounded-2xl bg-zinc-900 border border-zinc-800 flex items-center justify-center text-2xl mx-auto">
        📦
      </div>
      <div>
        <h3 class="text-sm font-bold text-white">Slot {appState.activeSlotId} is Empty</h3>
        <p class="text-xs text-zinc-400 mt-1 max-w-xs mx-auto leading-relaxed">
          Create a new purchase scenario, load one of your saved scenarios, or import a JSON file into this slot.
        </p>
      </div>

      <div class="space-y-2 pt-2">
        <button
          class="w-full py-2.5 px-4 rounded-xl bg-emerald-600 hover:bg-emerald-500 text-white text-xs font-semibold shadow-sm transition-colors flex items-center justify-center gap-2"
          onclick={() => appState.createScenarioInSlot(appState.activeSlotId)}
        >
          <span>✨</span>
          <span>Create New Scenario</span>
        </button>

        <button
          class="w-full py-2 px-4 rounded-xl bg-zinc-900 border border-zinc-800 hover:bg-zinc-800 text-zinc-300 text-xs font-medium transition-colors flex items-center justify-center gap-2"
          onclick={onOpenLibrary}
        >
          <span>📁</span>
          <span>Load Saved Scenario</span>
        </button>

        <button
          class="w-full py-2 px-4 rounded-xl bg-zinc-900 border border-zinc-800 hover:bg-zinc-800 text-zinc-300 text-xs font-medium transition-colors flex items-center justify-center gap-2"
          onclick={onOpenImport}
        >
          <span>📥</span>
          <span>Import JSON File</span>
        </button>
      </div>
    </div>
  {:else}
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
  {/if}
</div>
