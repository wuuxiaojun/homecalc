import type { Purchase, Scenario, ScenarioAnalysis, ScenarioComparison, ScenarioSlot, SlotId } from './types';
import {
  ensureWasmInitialized,
  isWasmReady,
  computeScenarioSync,
  computeAnalysisSync,
  computeComparisonSync
} from '../engine/engineBridge';

export const DEFAULT_SLOT_1_PURCHASE: Purchase = {
  name: 'Standard 30yr Conventional',
  house: {
    purchase_price: 1_000_000,
    annual_property_tax_rate: 1.25,
    annual_insurance: 2_400,
    monthly_hoa: 120
  },
  tools: [
    { Cash: { amount: 200_000, rate: 4.0 } },
    { Mortgage: { amount: 800_000, rate: 6.5, term: 30 } }
  ],
  mortgage_repay: {},
  loc_repay: {}
};

export const DEFAULT_SLOT_2_PURCHASE: Purchase = {
  name: 'Accelerated 15yr Fixed',
  house: {
    purchase_price: 1_000_000,
    annual_property_tax_rate: 1.25,
    annual_insurance: 2_400,
    monthly_hoa: 120
  },
  tools: [
    { Cash: { amount: 200_000, rate: 4.0 } },
    { Mortgage: { amount: 800_000, rate: 5.75, term: 15 } }
  ],
  mortgage_repay: {},
  loc_repay: {}
};

export const DEFAULT_SLOT_3_PURCHASE: Purchase = {
  name: 'Hybrid Mortgage + LOC Split',
  house: {
    purchase_price: 1_200_000,
    annual_property_tax_rate: 1.20,
    annual_insurance: 2_400,
    monthly_hoa: 150
  },
  tools: [
    { Cash: { amount: 240_000, rate: 4.0 } },
    { Mortgage: { amount: 720_000, rate: 6.25, term: 30 } },
    { Loc: { amount: 240_000, rate: 6.75 } }
  ],
  mortgage_repay: { 12: 50_000 },
  loc_repay: { 6: 20_000 }
};

export class AppState {
  // Reactive Svelte 5 state variables
  isInitialized = $state<boolean>(false);
  activeSlotId = $state<SlotId>(1);
  isComparisonMode = $state<boolean>(false);
  comparisonBaselineId = $state<SlotId>(1);
  comparisonAlternativeId = $state<SlotId>(2);
  activeView = $state<'overview' | 'charts' | 'statements' | 'comparison'>('overview');
  activeParamTab = $state<'property' | 'tools' | 'repayments' | 'library'>('property');
  selectedMonth = $state<number | null>(null);

  // Scenario slots storage
  slot1 = $state<ScenarioSlot>({
    id: 1,
    name: 'Slot 1',
    purchase: $state.snapshot(DEFAULT_SLOT_1_PURCHASE),
    scenario: null,
    analysis: null,
    error: null
  });

  slot2 = $state<ScenarioSlot>({
    id: 2,
    name: 'Slot 2',
    purchase: $state.snapshot(DEFAULT_SLOT_2_PURCHASE),
    scenario: null,
    analysis: null,
    error: null
  });

  slot3 = $state<ScenarioSlot>({
    id: 3,
    name: 'Slot 3',
    purchase: $state.snapshot(DEFAULT_SLOT_3_PURCHASE),
    scenario: null,
    analysis: null,
    error: null
  });

  constructor() {
    this.init();
  }

  async init() {
    await ensureWasmInitialized();
    this.isInitialized = true;
    this.recalculateAll();
  }

  // Get slot by ID
  getSlot(id: SlotId): ScenarioSlot {
    switch (id) {
      case 1:
        return this.slot1;
      case 2:
        return this.slot2;
      case 3:
        return this.slot3;
    }
  }

  // Active Slot getter
  get activeSlot(): ScenarioSlot {
    return this.getSlot(this.activeSlotId);
  }

  // Comparison result getter
  get comparison(): ScenarioComparison | null {
    if (!this.isInitialized) return null;
    const baseline = this.getSlot(this.comparisonBaselineId).scenario;
    const alternative = this.getSlot(this.comparisonAlternativeId).scenario;
    if (!baseline || !alternative) return null;
    try {
      return computeComparisonSync(baseline, alternative);
    } catch (err) {
      console.error('Comparison calculation failed:', err);
      return null;
    }
  }

  // Recalculate single slot
  recalculateSlot(slotId: SlotId) {
    if (!isWasmReady()) return;
    const slot = this.getSlot(slotId);
    try {
      const scenario = computeScenarioSync(slot.purchase);
      const analysis = computeAnalysisSync(scenario);
      slot.scenario = scenario;
      slot.analysis = analysis;
      slot.error = null;
    } catch (err: any) {
      slot.error = err?.message || String(err);
      console.error(`Slot ${slotId} simulation error:`, err);
    }
  }

  // Recalculate all 3 slots
  recalculateAll() {
    this.recalculateSlot(1);
    this.recalculateSlot(2);
    this.recalculateSlot(3);
  }

  // Update active purchase object
  updateActivePurchase(updater: (purchase: Purchase) => void) {
    const slot = this.activeSlot;
    updater(slot.purchase);
    this.recalculateSlot(this.activeSlotId);
  }

  // Set active slot
  setActiveSlot(id: SlotId) {
    this.activeSlotId = id;
    if (this.isComparisonMode) {
      this.comparisonAlternativeId = id;
    }
  }

  // Load a purchase into a specific slot
  loadPurchaseIntoSlot(slotId: SlotId, purchase: Purchase) {
    const slot = this.getSlot(slotId);
    slot.purchase = JSON.parse(JSON.stringify(purchase));
    slot.name = purchase.name || `Slot ${slotId}`;
    this.recalculateSlot(slotId);
  }

  // Duplicate one slot to another
  duplicateSlot(sourceId: SlotId, targetId: SlotId) {
    const src = this.getSlot(sourceId);
    const target = this.getSlot(targetId);
    target.purchase = JSON.parse(JSON.stringify(src.purchase));
    target.name = `${src.purchase.name} (Copy)`;
    target.purchase.name = target.name;
    this.recalculateSlot(targetId);
  }

  // Reset a slot to default
  resetSlot(slotId: SlotId) {
    const defaultPurchase =
      slotId === 1
        ? DEFAULT_SLOT_1_PURCHASE
        : slotId === 2
        ? DEFAULT_SLOT_2_PURCHASE
        : DEFAULT_SLOT_3_PURCHASE;
    this.loadPurchaseIntoSlot(slotId, defaultPurchase);
  }

  // Prepayment management
  addExtraPayment(tool: 'mortgage' | 'loc', month: number, amount: number) {
    this.updateActivePurchase((p) => {
      if (tool === 'mortgage') {
        p.mortgage_repay[month] = amount;
      } else {
        p.loc_repay[month] = amount;
      }
    });
  }

  removeExtraPayment(tool: 'mortgage' | 'loc', month: number) {
    this.updateActivePurchase((p) => {
      if (tool === 'mortgage') {
        delete p.mortgage_repay[month];
      } else {
        delete p.loc_repay[month];
      }
    });
  }
}

export const appState = new AppState();
