import initWasm, {
  wasm_create_scenario_from_json,
  wasm_analyze_scenario_from_json,
  wasm_compare_scenarios_from_json,
  wasm_calculate_scenario_pv_from_json,
  wasm_solve_irr,
  wasm_engine_version,
  wasm_default_starting_cash
} from '../wasm/engine_wasm.js';
import type { Purchase, Scenario, ScenarioAnalysis, ScenarioComparison } from '../state/types';

let wasmInitialized = false;
let initPromise: Promise<void> | null = null;

export async function ensureWasmInitialized(): Promise<void> {
  if (wasmInitialized) return;
  if (!initPromise) {
    initPromise = (async () => {
      try {
        await initWasm();
        wasmInitialized = true;
      } catch (err) {
        console.error('Failed to initialize WASM module:', err);
        throw err;
      }
    })();
  }
  return initPromise;
}

export function isWasmReady(): boolean {
  return wasmInitialized;
}

export function computeScenarioSync(purchase: Purchase): Scenario {
  const jsonInput = JSON.stringify(purchase);
  const jsonOutput = wasm_create_scenario_from_json(jsonInput);
  return JSON.parse(jsonOutput) as Scenario;
}

export function computeAnalysisSync(scenario: Scenario): ScenarioAnalysis {
  const jsonInput = JSON.stringify(scenario);
  const jsonOutput = wasm_analyze_scenario_from_json(jsonInput);
  return JSON.parse(jsonOutput) as ScenarioAnalysis;
}

export function computeComparisonSync(baseline: Scenario, alternative: Scenario): ScenarioComparison {
  const jsonBaseline = JSON.stringify(baseline);
  const jsonAlternative = JSON.stringify(alternative);
  const jsonOutput = wasm_compare_scenarios_from_json(jsonBaseline, jsonAlternative);
  return JSON.parse(jsonOutput) as ScenarioComparison;
}

export function computePvSync(scenario: Scenario): number {
  const jsonInput = JSON.stringify(scenario);
  return wasm_calculate_scenario_pv_from_json(jsonInput);
}

export function solveIrrSync(cashFlows: number[]): number | null {
  const jsonInput = JSON.stringify(cashFlows);
  const irr = wasm_solve_irr(jsonInput);
  return irr ?? null;
}

export function getEngineVersion(): string {
  try {
    return wasm_engine_version();
  } catch {
    return '2.0.2';
  }
}

export function getDefaultStartingCash(): number {
  try {
    return wasm_default_starting_cash();
  } catch {
    return 1_000_000.00;
  }
}
