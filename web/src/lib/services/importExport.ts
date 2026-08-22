import type { Purchase, ScenarioSlot } from '../state/types';

export async function saveTextFile(
  content: string,
  suggestedName: string,
  mimeType: string,
  extension: string,
  description: string
): Promise<void> {
  // 1. If File System Access API is supported, open native "Save As" file picker dialog
  if (typeof window !== 'undefined' && 'showSaveFilePicker' in window) {
    try {
      const handle = await (window as any).showSaveFilePicker({
        suggestedName,
        types: [
          {
            description,
            accept: { [mimeType]: [extension] }
          }
        ]
      });
      const writable = await handle.createWritable();
      await writable.write(content);
      await writable.close();
      return;
    } catch (err: any) {
      if (err.name === 'AbortError') {
        // User clicked Cancel in the file chooser dialog
        return;
      }
      console.warn('showSaveFilePicker failed, falling back to download anchor:', err);
    }
  }

  // 2. Fallback to standard anchor download
  const blob = new Blob([content], { type: `${mimeType};charset=utf-8;` });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = suggestedName;
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
  URL.revokeObjectURL(url);
}

export async function exportPurchaseToJson(purchase: Purchase): Promise<void> {
  const jsonStr = JSON.stringify(purchase, null, 2);
  const sanitizedName = (purchase.name || 'scenario').toLowerCase().replace(/[^a-z0-9]/g, '_');
  const suggestedName = `${sanitizedName}.json`;
  await saveTextFile(jsonStr, suggestedName, 'application/json', '.json', 'Homecalc Scenario JSON File');
}

export async function exportFullReportJson(slot: ScenarioSlot): Promise<void> {
  const reportData = {
    exportedAt: new Date().toISOString(),
    engine: 'Homecalc Rust WASM v2.0.0',
    purchase: slot.purchase,
    analysis: slot.analysis,
    totalStatement: slot.scenario?.total_statement,
    yearlyStatement: slot.scenario?.yearly_statement
  };
  const jsonStr = JSON.stringify(reportData, null, 2);
  const sanitizedName = (slot.purchase.name || 'scenario').toLowerCase().replace(/[^a-z0-9]/g, '_');
  const suggestedName = `${sanitizedName}_full_analysis_report.json`;
  await saveTextFile(jsonStr, suggestedName, 'application/json', '.json', 'Homecalc Analysis Report JSON');
}

export async function exportCsvFile(csvContent: string, suggestedFilename: string): Promise<void> {
  await saveTextFile(csvContent, suggestedFilename, 'text/csv', '.csv', 'CSV Spreadsheet File');
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
