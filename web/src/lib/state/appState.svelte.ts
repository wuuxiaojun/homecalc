import type { Purchase, Scenario, ScenarioAnalysis, ScenarioComparison, ScenarioSlot, SlotId } from './types';
import {
  ensureWasmInitialized,
  isWasmReady,
  computeScenarioSync,
  computeAnalysisSync,
  computeComparisonSync
} from '../engine/engineBridge';
import { loadSlotsFromStorage, saveSlotsToStorage } from '../services/persistence';

export function createDefaultScenario(name = 'Standard 30Y Mortgage'): Purchase {
  return {
    name,
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
}

export class AppState {
  // Reactive Svelte 5 state variables
  isInitialized = $state<boolean>(false);
  activeSlotId = $state<SlotId>(1);
  isComparisonMode = $state<boolean>(false);
  comparisonBaselineId = $state<SlotId>(1);
  comparisonAlternativeId = $state<SlotId>(2);
  activeView = $state<'overview' | 'charts' | 'statements' | 'comparison'>('overview');
  activeParamTab = $state<'property' | 'tools' | 'repayments'>('property');
  selectedMonth = $state<number | null>(null);

  // Scenario slots storage (All 3 initialized with the same default scenario name "Standard 30Y Mortgage")
  slot1 = $state<ScenarioSlot>({
    id: 1,
    name: 'Slot 1',
    purchase: createDefaultScenario('Standard 30Y Mortgage'),
    scenario: null,
    analysis: null,
    error: null
  });

  slot2 = $state<ScenarioSlot>({
    id: 2,
    name: 'Slot 2',
    purchase: createDefaultScenario('Standard 30Y Mortgage'),
    scenario: null,
    analysis: null,
    error: null
  });

  slot3 = $state<ScenarioSlot>({
    id: 3,
    name: 'Slot 3',
    purchase: createDefaultScenario('Standard 30Y Mortgage'),
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

    // Restore from localStorage if user had previous sessions
    const saved = loadSlotsFromStorage();
    if (saved) {
      if (saved.slot1) this.loadPurchaseIntoSlot(1, saved.slot1, false);
      if (saved.slot2) this.loadPurchaseIntoSlot(2, saved.slot2, false);
      if (saved.slot3) this.loadPurchaseIntoSlot(3, saved.slot3, false);
    } else {
      this.persistSlots();
    }

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
    const baselineSlot = this.getSlot(this.comparisonBaselineId);
    const alternativeSlot = this.getSlot(this.comparisonAlternativeId);
    if (!baselineSlot.scenario || !alternativeSlot.scenario) return null;
    try {
      return computeComparisonSync(baselineSlot.scenario, alternativeSlot.scenario);
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
    this.persistSlots();
  }

  // Set active slot
  setActiveSlot(id: SlotId) {
    this.activeSlotId = id;
    if (this.isComparisonMode) {
      this.comparisonAlternativeId = id;
    }
  }

  // Load a purchase into a specific slot
  loadPurchaseIntoSlot(slotId: SlotId, purchase: Purchase, persist = true) {
    const slot = this.getSlot(slotId);
    slot.purchase = JSON.parse(JSON.stringify(purchase));
    slot.name = purchase.name || `Slot ${slotId}`;
    this.recalculateSlot(slotId);
    if (persist) {
      this.persistSlots();
    }
  }

  // Reset a slot back to the baseline Standard 30Y Mortgage scenario
  resetSlot(slotId: SlotId) {
    this.loadPurchaseIntoSlot(slotId, createDefaultScenario('Standard 30Y Mortgage'));
  }

  // Duplicate one slot to another
  duplicateSlot(sourceId: SlotId, targetId: SlotId) {
    const src = this.getSlot(sourceId);
    const target = this.getSlot(targetId);
    target.purchase = JSON.parse(JSON.stringify(src.purchase));
    target.name = `${src.purchase.name} (Copy)`;
    target.purchase.name = target.name;
    this.recalculateSlot(targetId);
    this.persistSlots();
  }

  // Save current slots to LocalStorage
  persistSlots() {
    saveSlotsToStorage(this.slot1.purchase, this.slot2.purchase, this.slot3.purchase);
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
