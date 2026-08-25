<script lang="ts">
  import { appState } from '../../state/appState.svelte';
  import Card from '../common/Card.svelte';

  const purchase = $derived(appState.activeSlot.purchase);
  const house = $derived(purchase?.house || {
    purchase_price: 1_000_000,
    annual_property_tax_rate: 1.20,
    annual_insurance: 2400,
    monthly_hoa: 100
  });

  const monthlyTax = $derived((house.purchase_price * house.annual_property_tax_rate * 0.01) / 12);
  const monthlyInsurance = $derived(house.annual_insurance / 12);
  const totalMonthlyHolding = $derived(monthlyTax + monthlyInsurance + house.monthly_hoa);

  function updatePrice(val: number) {
    appState.updateActivePurchase((p) => {
      p.house.purchase_price = val;
      // Auto-rebalance cash
      const mort = p.tools.find(t => 'Mortgage' in t)?.Mortgage?.amount || 0;
      const loc = p.tools.find(t => 'Loc' in t)?.Loc?.amount || 0;
      const cashTool = p.tools.find(t => 'Cash' in t);
      if (cashTool && 'Cash' in cashTool && cashTool.Cash) {
        cashTool.Cash.amount = Math.max(0, val - (mort + loc));
      }
    });
  }

  function updateTaxRate(val: number) {
    appState.updateActivePurchase((p) => {
      p.house.annual_property_tax_rate = val;
    });
  }

  function updateInsurance(val: number) {
    appState.updateActivePurchase((p) => {
      p.house.annual_insurance = val;
    });
  }

  function updateHoa(val: number) {
    appState.updateActivePurchase((p) => {
      p.house.monthly_hoa = val;
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
  <div class="space-y-4">
    <!-- Scenario Name Card -->
    <Card icon="📝" title="Scenario Name">
      <input
        id="scenario-name"
        type="text"
        class="w-full px-3 py-2 rounded-xl bg-zinc-950 border border-zinc-800 focus:border-emerald-500 focus:ring-1 focus:ring-emerald-500 text-sm font-medium text-white transition-all focus:outline-none"
        value={purchase.name}
        onblur={(e) => updateName(e.currentTarget.value.trim() || 'Untitled Scenario')}
        onkeydown={(e) => { if (e.key === 'Enter') e.currentTarget.blur(); }}
        placeholder="e.g. Standard 30Y Mortgage"
      />
    </Card>

    <!-- Purchase Price Card -->
    <Card icon="🏡" title="Purchase Price">
      {#snippet headerRight()}
        <div class="flex items-center gap-1">
          <span class="text-xs font-mono text-zinc-500">$</span>
          <input
            id="purchase-price-input"
            type="number"
            step="10000"
            min="0"
            max="100000000"
            class="w-32 px-2 py-1 text-right rounded-md bg-zinc-950 border border-zinc-800 text-sm font-mono font-bold text-emerald-400 tabular-nums focus:border-emerald-500 focus:outline-none"
            value={house.purchase_price}
            onblur={(e) => {
              const val = parseFloat(e.currentTarget.value);
              const clamped = isNaN(val) ? 0 : Math.max(0, Math.min(100_000_000, val));
              updatePrice(clamped);
              e.currentTarget.value = String(clamped);
            }}
            onkeydown={(e) => { if (e.key === 'Enter') e.currentTarget.blur(); }}
          />
        </div>
      {/snippet}
    </Card>

    <!-- 1. Property Tax Card -->
    <Card icon="🏛️" title="Property Tax">
      {#snippet headerRight()}
        <div class="flex items-center gap-1">
          <input
            id="tax-rate-input"
            type="number"
            step="0.05"
            min="0"
            max="10"
            class="w-24 px-2 py-1 text-right rounded-md bg-zinc-950 border border-zinc-800 text-xs font-mono font-semibold text-zinc-200 tabular-nums focus:border-emerald-500 focus:outline-none"
            value={house.annual_property_tax_rate}
            onblur={(e) => {
              const val = parseFloat(e.currentTarget.value);
              const clamped = isNaN(val) ? 0 : Math.max(0, Math.min(10.0, val));
              updateTaxRate(clamped);
              e.currentTarget.value = String(clamped);
            }}
            onkeydown={(e) => { if (e.key === 'Enter') e.currentTarget.blur(); }}
          />
          <span class="text-xs font-mono text-zinc-400">%</span>
        </div>
      {/snippet}

      <div class="flex items-center justify-between text-[11px] font-mono text-zinc-500">
        <span>${Math.round(house.purchase_price * house.annual_property_tax_rate * 0.01).toLocaleString()}/yr</span>
        <span class="text-zinc-300 font-medium">${Math.round(monthlyTax).toLocaleString()}/mo</span>
      </div>
    </Card>

    <!-- 2. Home Insurance Card -->
    <Card icon="🛡️" title="Home Insurance">
      {#snippet headerRight()}
        <div class="flex items-center gap-1">
          <span class="text-xs font-mono text-zinc-500">$</span>
          <input
            id="insurance-input"
            type="number"
            step="100"
            min="0"
            max="100000"
            class="w-24 px-2 py-1 text-right rounded-md bg-zinc-950 border border-zinc-800 text-xs font-mono font-semibold text-zinc-200 tabular-nums focus:border-emerald-500 focus:outline-none"
            value={house.annual_insurance}
            onblur={(e) => {
              const val = parseFloat(e.currentTarget.value);
              const clamped = isNaN(val) ? 0 : Math.max(0, Math.min(100_000, val));
              updateInsurance(clamped);
              e.currentTarget.value = String(clamped);
            }}
            onkeydown={(e) => { if (e.key === 'Enter') e.currentTarget.blur(); }}
          />
          <span class="text-xs font-mono text-zinc-400">/yr</span>
        </div>
      {/snippet}

      <div class="flex items-center justify-between text-[11px] font-mono text-zinc-500">
        <span>${Math.round(house.annual_insurance).toLocaleString()}/yr</span>
        <span class="text-zinc-300 font-medium">${Math.round(monthlyInsurance).toLocaleString()}/mo</span>
      </div>
    </Card>

    <!-- 3. HOA Fee Card -->
    <Card icon="🏢" title="HOA Fee">
      {#snippet headerRight()}
        <div class="flex items-center gap-1">
          <span class="text-xs font-mono text-zinc-500">$</span>
          <input
            id="hoa-input"
            type="number"
            step="25"
            min="0"
            max="10000"
            class="w-24 px-2 py-1 text-right rounded-md bg-zinc-950 border border-zinc-800 text-xs font-mono font-semibold text-zinc-200 tabular-nums focus:border-emerald-500 focus:outline-none"
            value={house.monthly_hoa}
            onblur={(e) => {
              const val = parseFloat(e.currentTarget.value);
              const clamped = isNaN(val) ? 0 : Math.max(0, Math.min(10_000, val));
              updateHoa(clamped);
              e.currentTarget.value = String(clamped);
            }}
            onkeydown={(e) => { if (e.key === 'Enter') e.currentTarget.blur(); }}
          />
          <span class="text-xs font-mono text-zinc-400">/mo</span>
        </div>
      {/snippet}

      <div class="flex items-center justify-between text-[11px] font-mono text-zinc-500">
        <span>${Math.round(house.monthly_hoa * 12).toLocaleString()}/yr</span>
        <span class="text-zinc-300 font-medium">${Math.round(house.monthly_hoa).toLocaleString()}/mo</span>
      </div>
    </Card>

    <!-- Holding Cost Aggregate Callout Card -->
    <Card icon="🏚️" title="Initial Holding Cost">
      {#snippet headerRight()}
        <div class="text-right">
          <div class="text-sm font-bold font-mono text-amber-400 tabular-nums">
            ${Math.round(totalMonthlyHolding).toLocaleString()} <span class="text-[10px] font-normal text-zinc-400">/mo</span>
          </div>
        </div>
      {/snippet}

      <div class="flex items-center justify-between text-[11px] font-mono text-zinc-500">
        <span>Tax + Insurance + HOA</span>
        <span>${Math.round(totalMonthlyHolding * 12).toLocaleString()} /yr</span>
      </div>
    </Card>
  </div>
{/if}
