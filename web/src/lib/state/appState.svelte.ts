import type { Purchase, Scenario, ScenarioAnalysis, ScenarioComparison, ScenarioSlot, SlotId } from './types';
import {
  ensureWasmInitialized,
  isWasmReady,
  computeScenarioSync,
  computeAnalysisSync,
  computeComparisonSync
} from '../engine/engineBridge';
import { loadSlotsFromStorage, saveSlotsToStorage } from '../services/persistence';

export function createBlankPurchase(name = 'New Scenario'): Purchase {
  return {
    name,
    house: {
      purchase_price: 1_000_000,
      annual_property_tax_rate: 1.25,
      annual_insurance: 2_400,
      monthly_hoa: 100
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

  // Scenario slots storage (All 3 start completely empty by default)
  slot1 = $state<ScenarioSlot>({
    id: 1,
    name: 'Slot 1 (Empty)',
    purchase: null,
    scenario: null,
    analysis: null,
    error: null
  });

  slot2 = $state<ScenarioSlot>({
    id: 2,
    name: 'Slot 2 (Empty)',
    purchase: null,
    scenario: null,
    analysis: null,
    error: null
  });

  slot3 = $state<ScenarioSlot>({
    id: 3,
    name: 'Slot 3 (Empty)',
    purchase: null,
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

  // Helper: check if slot is empty
  isSlotEmpty(id: SlotId): boolean {
    return this.getSlot(id).purchase === null;
  }

  // Helper: count populated slots
  get populatedSlotsCount(): number {
    let count = 0;
    if (this.slot1.purchase) count++;
    if (this.slot2.purchase) count++;
    if (this.slot3.purchase) count++;
    return count;
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
    if (!slot.purchase) {
      slot.scenario = null;
      slot.analysis = null;
      slot.error = null;
      return;
    }
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
    if (!slot.purchase) return;
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

  // Create a fresh new scenario in a slot
  createScenarioInSlot(slotId: SlotId, customPurchase?: Purchase) {
    const p = customPurchase ? JSON.parse(JSON.stringify(customPurchase)) : createBlankPurchase(`Scenario in Slot ${slotId}`);
    this.loadPurchaseIntoSlot(slotId, p);
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

  // Clear / remove scenario from a slot
  clearSlot(slotId: SlotId) {
    const slot = this.getSlot(slotId);
    slot.purchase = null;
    slot.scenario = null;
    slot.analysis = null;
    slot.name = `Slot ${slotId} (Empty)`;
    slot.error = null;
    this.persistSlots();
  }

  // Duplicate one slot to another
  duplicateSlot(sourceId: SlotId, targetId: SlotId) {
    const src = this.getSlot(sourceId);
    if (!src.purchase) return;
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
