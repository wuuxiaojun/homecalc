import type { Purchase, SlotId } from '../state/types';

const STORAGE_KEY_SLOTS = 'homecalc_v2_slots';
const STORAGE_KEY_CUSTOM = 'homecalc_v2_custom_scenarios';

export interface SavedUserScenario {
  id: string;
  name: string;
  savedAt: string;
  purchase: Purchase;
}

export function saveSlotsToStorage(slot1: Purchase, slot2: Purchase, slot3: Purchase): void {
  try {
    const data = { slot1, slot2, slot3 };
    localStorage.setItem(STORAGE_KEY_SLOTS, JSON.stringify(data));
  } catch (e) {
    console.warn('LocalStorage save failed:', e);
  }
}

export function loadSlotsFromStorage(): { slot1?: Purchase; slot2?: Purchase; slot3?: Purchase } | null {
  try {
    const raw = localStorage.getItem(STORAGE_KEY_SLOTS);
    if (!raw) return null;
    return JSON.parse(raw);
  } catch {
    return null;
  }
}

export function getCustomScenarios(): SavedUserScenario[] {
  try {
    const raw = localStorage.getItem(STORAGE_KEY_CUSTOM);
    if (!raw) return [];
    return JSON.parse(raw);
  } catch {
    return [];
  }
}

export function saveCustomScenario(purchase: Purchase): SavedUserScenario {
  const customList = getCustomScenarios();
  const newEntry: SavedUserScenario = {
    id: 'user_' + Date.now() + '_' + Math.random().toString(36).substring(2, 7),
    name: purchase.name || 'Untitled Scenario',
    savedAt: new Date().toISOString(),
    purchase: JSON.parse(JSON.stringify(purchase))
  };
  customList.unshift(newEntry);
  localStorage.setItem(STORAGE_KEY_CUSTOM, JSON.stringify(customList));
  return newEntry;
}

export function deleteCustomScenario(id: string): void {
  const customList = getCustomScenarios().filter(s => s.id !== id);
  localStorage.setItem(STORAGE_KEY_CUSTOM, JSON.stringify(customList));
}
