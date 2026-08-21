import type { Purchase, ScenarioSlot } from '../state/types';

export function exportPurchaseToJson(purchase: Purchase): void {
  const jsonStr = JSON.stringify(purchase, null, 2);
  const blob = new Blob([jsonStr], { type: 'application/json' });
  const url = URL.createObjectURL(blob);
  const sanitizedName = (purchase.name || 'scenario').toLowerCase().replace(/[^a-z0-9]/g, '_');
  
  const a = document.createElement('a');
  a.href = url;
  a.download = `${sanitizedName}.json`;
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
  URL.revokeObjectURL(url);
}

export function exportFullReportJson(slot: ScenarioSlot): void {
  const reportData = {
    exportedAt: new Date().toISOString(),
    engine: 'Homecalc Rust WASM v2.0',
    purchase: slot.purchase,
    analysis: slot.analysis,
    totalStatement: slot.scenario?.total_statement,
    yearlyStatement: slot.scenario?.yearly_statement
  };
  const jsonStr = JSON.stringify(reportData, null, 2);
  const blob = new Blob([jsonStr], { type: 'application/json' });
  const url = URL.createObjectURL(blob);
  const sanitizedName = (slot.purchase.name || 'scenario').toLowerCase().replace(/[^a-z0-9]/g, '_');
  
  const a = document.createElement('a');
  a.href = url;
  a.download = `${sanitizedName}_full_analysis_report.json`;
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
  URL.revokeObjectURL(url);
}

export function parseAndValidatePurchaseJson(jsonText: string): Purchase {
  const parsed = JSON.parse(jsonText);
  if (!parsed || typeof parsed !== 'object') {
    throw new Error('Invalid JSON format: expected an object.');
  }

  // Handle both direct Purchase format and Full Report wrapper
  const p: Purchase = parsed.purchase && typeof parsed.purchase === 'object' ? parsed.purchase : parsed;

  if (typeof p.name !== 'string' || !p.house || typeof p.house !== 'object') {
    throw new Error('JSON is missing required Purchase fields ("name", "house").');
  }

  if (typeof p.house.purchase_price !== 'number' || p.house.purchase_price < 0) {
    throw new Error('Invalid house.purchase_price in JSON.');
  }

  if (!Array.isArray(p.tools)) {
    throw new Error('Invalid tools array in JSON.');
  }

  // Ensure mortgage_repay and loc_repay are maps
  if (!p.mortgage_repay || typeof p.mortgage_repay !== 'object') {
    p.mortgage_repay = {};
  }
  if (!p.loc_repay || typeof p.loc_repay !== 'object') {
    p.loc_repay = {};
  }

  return p;
}

export async function importPurchaseFromFile(file: File): Promise<Purchase> {
  const text = await file.text();
  return parseAndValidatePurchaseJson(text);
}
