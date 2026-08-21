<script lang="ts">
  import { appState } from '../../state/appState.svelte';

  const purchase = $derived(appState.activeSlot.purchase);
  const house = $derived(purchase?.house || {
    purchase_price: 1_000_000,
    annual_property_tax_rate: 1.25,
    annual_insurance: 2400,
    monthly_hoa: 100
  });

  const monthlyTax = $derived((house.purchase_price * house.annual_property_tax_rate * 0.01) / 12);
  const monthlyInsurance = $derived(house.annual_insurance / 12);
  const totalMonthlyHolding = $derived(monthlyTax + monthlyInsurance + house.monthly_hoa);

  function updatePrice(val: number) {
    const clamped = Math.max(0, val);
    appState.updateActivePurchase((p) => {
      p.house.purchase_price = clamped;
      // Auto-rebalance cash if needed
      const mort = p.tools.find(t => 'Mortgage' in t)?.Mortgage?.amount || 0;
      const loc = p.tools.find(t => 'Loc' in t)?.Loc?.amount || 0;
      const cashTool = p.tools.find(t => 'Cash' in t);
      if (cashTool && 'Cash' in cashTool) {
        cashTool.Cash.amount = Math.max(0, clamped - (mort + loc));
      }
    });
  }

  function updateTaxRate(val: number) {
    appState.updateActivePurchase((p) => {
      p.house.annual_property_tax_rate = Math.max(0, Math.min(20, val));
    });
  }

  function updateInsurance(val: number) {
    appState.updateActivePurchase((p) => {
      p.house.annual_insurance = Math.max(0, val);
    });
  }

  function updateHoa(val: number) {
    appState.updateActivePurchase((p) => {
      p.house.monthly_hoa = Math.max(0, val);
    });
  }

  function updateName(name: string) {
    appState.updateActivePurchase((p) => {
      p.name = name;
      appState.activeSlot.name = name;
    });
  }
</script>

{#if purchase}
  <div class="space-y-5">
    <!-- Scenario Name -->
    <div>
      <label for="scenario-name" class="block text-xs font-semibold text-zinc-300 mb-1.5">Scenario Name</label>
      <input
        id="scenario-name"
        type="text"
        class="w-full px-3 py-2 rounded-lg bg-zinc-900 border border-zinc-800 focus:border-emerald-500 focus:ring-1 focus:ring-emerald-500 text-sm font-medium text-white transition-all"
        value={purchase.name}
        oninput={(e) => updateName(e.currentTarget.value)}
        placeholder="e.g. Dream House Option A"
      />
    </div>

    <!-- Purchase Price -->
    <div class="p-3.5 rounded-xl bg-zinc-900/60 border border-zinc-800/80 space-y-3">
      <div class="flex items-center justify-between">
        <label for="purchase-price-input" class="text-xs font-semibold text-zinc-300 flex items-center gap-1.5">
          <span>🏡 Purchase Price</span>
        </label>
        <div class="flex items-center gap-1">
          <span class="text-xs font-mono text-zinc-500">$</span>
          <input
            id="purchase-price-input"
            type="number"
            step="10000"
            min="0"
            class="w-32 px-2 py-1 text-right rounded-md bg-zinc-950 border border-zinc-800 text-sm font-mono font-bold text-emerald-400 tabular-nums focus:border-emerald-500 focus:outline-none"
            value={house.purchase_price}
            oninput={(e) => updatePrice(parseFloat(e.currentTarget.value) || 0)}
          />
        </div>
      </div>

      <input
        type="range"
        min="100000"
        max="5000000"
        step="25000"
        aria-label="Purchase Price Slider"
        class="w-full h-1.5 bg-zinc-800 rounded-lg appearance-none cursor-pointer accent-emerald-500"
        value={house.purchase_price}
        oninput={(e) => updatePrice(parseFloat(e.currentTarget.value) || 0)}
      />

      <!-- Quick Increment Presets -->
      <div class="flex items-center gap-1.5 pt-1">
        <span class="text-[10px] text-zinc-500 font-mono">Quick Adjust:</span>
        <button
          class="px-2 py-0.5 text-[11px] font-mono rounded bg-zinc-800/80 hover:bg-zinc-700 text-zinc-300 transition-colors"
          onclick={() => updatePrice(house.purchase_price - 50000)}
        >
          -$50k
        </button>
        <button
          class="px-2 py-0.5 text-[11px] font-mono rounded bg-zinc-800/80 hover:bg-zinc-700 text-zinc-300 transition-colors"
          onclick={() => updatePrice(house.purchase_price + 50000)}
        >
          +$50k
        </button>
        <button
          class="px-2 py-0.5 text-[11px] font-mono rounded bg-zinc-800/80 hover:bg-zinc-700 text-zinc-300 transition-colors"
          onclick={() => updatePrice(house.purchase_price + 100000)}
        >
          +$100k
        </button>
      </div>
    </div>

    <!-- Property Tax Rate -->
    <div class="p-3.5 rounded-xl bg-zinc-900/60 border border-zinc-800/80 space-y-2.5">
      <div class="flex items-center justify-between">
        <label for="tax-rate-input" class="text-xs font-semibold text-zinc-300">🏛️ Annual Property Tax</label>
        <div class="flex items-center gap-1">
          <input
            id="tax-rate-input"
            type="number"
            step="0.05"
            min="0"
            max="10"
            class="w-20 px-2 py-1 text-right rounded-md bg-zinc-950 border border-zinc-800 text-xs font-mono font-semibold text-zinc-200 tabular-nums focus:border-emerald-500 focus:outline-none"
            value={house.annual_property_tax_rate}
            oninput={(e) => updateTaxRate(parseFloat(e.currentTarget.value) || 0)}
          />
          <span class="text-xs font-mono text-zinc-400">%</span>
        </div>
      </div>

      <input
        type="range"
        min="0"
        max="4.0"
        step="0.05"
        aria-label="Annual Property Tax Slider"
        class="w-full h-1.5 bg-zinc-800 rounded-lg appearance-none cursor-pointer accent-emerald-500"
        value={house.annual_property_tax_rate}
        oninput={(e) => updateTaxRate(parseFloat(e.currentTarget.value) || 0)}
      />

      <div class="flex justify-between text-[11px] text-zinc-500 font-mono">
        <span>Monthly Est: ${Math.round(monthlyTax).toLocaleString()}/mo</span>
        <span>US Avg: ~1.1%</span>
      </div>
    </div>

    <!-- Annual Insurance & Monthly HOA (Side-by-side) -->
    <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
      <!-- Annual Insurance -->
      <div class="p-3.5 rounded-xl bg-zinc-900/60 border border-zinc-800/80 space-y-2">
        <div class="flex items-center justify-between">
          <label for="insurance-input" class="text-xs font-semibold text-zinc-300">🛡️ Insurance</label>
          <span class="text-[11px] font-mono text-zinc-500">$/year</span>
        </div>
        <input
          id="insurance-input"
          type="number"
          step="100"
          min="0"
          class="w-full px-2 py-1 text-right rounded-md bg-zinc-950 border border-zinc-800 text-xs font-mono font-semibold text-zinc-200 tabular-nums focus:border-emerald-500 focus:outline-none"
          value={house.annual_insurance}
          oninput={(e) => updateInsurance(parseFloat(e.currentTarget.value) || 0)}
        />
        <p class="text-[10px] text-zinc-500 font-mono text-right">${Math.round(monthlyInsurance).toLocaleString()}/mo</p>
      </div>

      <!-- Monthly HOA -->
      <div class="p-3.5 rounded-xl bg-zinc-900/60 border border-zinc-800/80 space-y-2">
        <div class="flex items-center justify-between">
          <label for="hoa-input" class="text-xs font-semibold text-zinc-300">🏢 HOA Fee</label>
          <span class="text-[11px] font-mono text-zinc-500">$/month</span>
        </div>
        <input
          id="hoa-input"
          type="number"
          step="25"
          min="0"
          class="w-full px-2 py-1 text-right rounded-md bg-zinc-950 border border-zinc-800 text-xs font-mono font-semibold text-zinc-200 tabular-nums focus:border-emerald-500 focus:outline-none"
          value={house.monthly_hoa}
          oninput={(e) => updateHoa(parseFloat(e.currentTarget.value) || 0)}
        />
        <p class="text-[10px] text-zinc-500 font-mono text-right">${Math.round(house.monthly_hoa).toLocaleString()}/mo</p>
      </div>
    </div>

    <!-- Holding Cost Aggregate Callout -->
    <div class="p-3 rounded-xl bg-zinc-950 border border-zinc-800/80 flex items-center justify-between">
      <div class="flex items-center gap-2">
        <span class="text-base">🏚️</span>
        <div>
          <div class="text-xs font-medium text-zinc-300">Initial Holding Cost</div>
          <div class="text-[10px] text-zinc-500">Tax + Insurance + HOA</div>
        </div>
      </div>
      <div class="text-right">
        <div class="text-sm font-bold font-mono text-amber-400 tabular-nums">
          ${Math.round(totalMonthlyHolding).toLocaleString()} <span class="text-[10px] font-normal text-zinc-400">/mo</span>
        </div>
        <div class="text-[10px] text-zinc-500 font-mono">
          ${Math.round(totalMonthlyHolding * 12).toLocaleString()} /yr
        </div>
      </div>
    </div>
  </div>
{/if}
