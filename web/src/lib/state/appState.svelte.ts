import type { Purchase, Scenario, ScenarioAnalysis, ScenarioComparison, ScenarioSlot, SlotId } from './types';
import {
  ensureWasmInitialized,
  isWasmReady,
  computeScenarioSync,
  computeAnalysisSync,
  computeComparisonSync,
  getDefaultStartingCash
} from '../engine/engineBridge';
import { loadSlotsFromStorage, saveSlotsToStorage } from '../services/persistence';

export function createDefaultScenario(name = 'Standard 30Y Mortgage'): Purchase {
  return {
    name,
    house: {
      purchase_price: 1_000_000,
      annual_property_tax_rate: 1.20,
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

  // Scenario slots storage (All 3 initialized with exact name "Standard 30Y Mortgage")
  slot1 = $state<ScenarioSlot>({
    id: 1,
    name: 'Standard 30Y Mortgage',
    purchase: createDefaultScenario('Standard 30Y Mortgage'),
    scenario: null,
    analysis: null,
    error: null
  });

  slot2 = $state<ScenarioSlot>({
    id: 2,
    name: 'Standard 30Y Mortgage',
    purchase: createDefaultScenario('Standard 30Y Mortgage'),
    scenario: null,
    analysis: null,
    error: null
  });

  slot3 = $state<ScenarioSlot>({
    id: 3,
    name: 'Standard 30Y Mortgage',
    purchase: createDefaultScenario('Standard 30Y Mortgage'),
    scenario: null,
    analysis: null,
    error: null
  });

  constructor() {
    this.init();
  }

  // Normalize any legacy/previous default names stored in user's browser localStorage
  private normalizeLegacyName(p: Purchase) {
    if (!p.name || p.name.includes('Default Scenario') || p.name.includes('New Scenario') || p.name.startsWith('Slot ')) {
      p.name = 'Standard 30Y Mortgage';
    }
    if (p.house && p.house.annual_property_tax_rate === 1.25 && p.house.purchase_price === 1_000_000) {
      p.house.annual_property_tax_rate = 1.20;
    }
  }

  async init() {
    await ensureWasmInitialized();
    this.isInitialized = true;

    // Restore from localStorage if user had previous sessions
    const saved = loadSlotsFromStorage();
    if (saved) {
      if (saved.slot1) {
        this.normalizeLegacyName(saved.slot1);
        this.loadPurchaseIntoSlot(1, saved.slot1, false);
      }
      if (saved.slot2) {
        this.normalizeLegacyName(saved.slot2);
        this.loadPurchaseIntoSlot(2, saved.slot2, false);
      }
      if (saved.slot3) {
        this.normalizeLegacyName(saved.slot3);
        this.loadPurchaseIntoSlot(3, saved.slot3, false);
      }
      this.persistSlots();
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

  // Validate purchase and derive cash down
  validateAndDeriveCash(purchase: Purchase): { isValid: boolean; error: string | null; derivedCashDown: number } {
    const housePrice = purchase.house.purchase_price;
    const mort = purchase.tools.find(t => 'Mortgage' in t)?.Mortgage?.amount || 0;
    const loc = purchase.tools.find(t => 'Loc' in t)?.Loc?.amount || 0;
    const totalDebt = mort + loc;
    const derivedCashDown = housePrice - totalDebt;
    const startingCashLimit = getDefaultStartingCash();

    // Check 1: Total borrowed > Purchase price
    if (totalDebt > housePrice) {
      const excess = totalDebt - housePrice;
      return {
        isValid: false,
        error: `Total borrowed ($${totalDebt.toLocaleString()}) exceeds purchase price ($${housePrice.toLocaleString()}) by $${excess.toLocaleString()}.`,
        derivedCashDown
      };
    }

    // Check 2: Derived Cash Down > Engine Starting Cash Limit
    if (derivedCashDown > startingCashLimit) {
      const excess = derivedCashDown - startingCashLimit;
      return {
        isValid: false,
        error: `Required cash down payment ($${derivedCashDown.toLocaleString()}) exceeds maximum available starting cash ($${startingCashLimit.toLocaleString()}) by $${excess.toLocaleString()}.`,
        derivedCashDown
      };
    }

    return {
      isValid: true,
      error: null,
      derivedCashDown
    };
  }

  // Recalculate single slot with validation guard
  recalculateSlot(slotId: SlotId) {
    if (!isWasmReady()) return;
    const slot = this.getSlot(slotId);
    const validation = this.validateAndDeriveCash(slot.purchase);

    if (!validation.isValid) {
      slot.error = validation.error;
      // Skip WASM recalculation, freeze and preserve previous valid scenario and analysis
      return;
    }

    // Valid state: sync derived cash down into tools and clear error
    slot.error = null;
    let cashTool = slot.purchase.tools.find(t => 'Cash' in t);
    if (cashTool && 'Cash' in cashTool && cashTool.Cash) {
      cashTool.Cash.amount = validation.derivedCashDown;
    } else {
      slot.purchase.tools.push({ Cash: { amount: validation.derivedCashDown, rate: 4.0 } });
    }

    try {
      const scenario = computeScenarioSync(slot.purchase);
      const analysis = computeAnalysisSync(scenario);
      slot.scenario = scenario;
      slot.analysis = analysis;
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
    slot.name = slot.purchase.name;
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
    slot.name = purchase.name || 'Standard 30Y Mortgage';
    this.recalculateSlot(slotId);
    if (persist) {
      this.persistSlots();
    }
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
