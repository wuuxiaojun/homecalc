/* tslint:disable */
/* eslint-disable */

/**
 * Computes single scenario analysis metrics from a JavaScript `Scenario` object.
 */
export function wasm_analyze_scenario(val: any): any;

/**
 * Computes single scenario analysis metrics from a JSON string `Scenario`.
 */
export function wasm_analyze_scenario_from_json(scenario_json: string): string;

/**
 * Calculates the discounted Present Value (PV) of a scenario's cash outflows from a JSON string `Scenario`.
 */
export function wasm_calculate_scenario_pv_from_json(scenario_json: string): number;

/**
 * Computes comparative metrics between baseline (A) and alternative (B) scenarios.
 */
export function wasm_compare_scenarios(val_a: any, val_b: any): any;

/**
 * Computes comparative metrics between baseline (A) and alternative (B) JSON string scenarios.
 */
export function wasm_compare_scenarios_from_json(scenario_a_json: string, scenario_b_json: string): string;

/**
 * Simulates a purchase scenario from a JavaScript `Purchase` object and returns the full `Scenario`.
 */
export function wasm_create_scenario(val: any): any;

/**
 * Simulates a purchase scenario from a JSON string representation of `Purchase` and returns the JSON string `Scenario`.
 */
export function wasm_create_scenario_from_json(purchase_json: string): string;

/**
 * Returns the default starting cash limit constant from engine configuration.
 */
export function wasm_default_starting_cash(): number;

/**
 * Returns the engine version and build info.
 */
export function wasm_engine_version(): string;

/**
 * Solves Internal Rate of Return (IRR) from a JSON array of monthly cash flows.
 */
export function wasm_solve_irr(cash_flows_json: string): number | undefined;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly wasm_analyze_scenario: (a: any) => [number, number, number];
    readonly wasm_analyze_scenario_from_json: (a: number, b: number) => [number, number, number, number];
    readonly wasm_calculate_scenario_pv_from_json: (a: number, b: number) => [number, number, number];
    readonly wasm_compare_scenarios: (a: any, b: any) => [number, number, number];
    readonly wasm_compare_scenarios_from_json: (a: number, b: number, c: number, d: number) => [number, number, number, number];
    readonly wasm_create_scenario: (a: any) => [number, number, number];
    readonly wasm_create_scenario_from_json: (a: number, b: number) => [number, number, number, number];
    readonly wasm_default_starting_cash: () => number;
    readonly wasm_engine_version: () => [number, number];
    readonly wasm_solve_irr: (a: number, b: number) => [number, number, number, number];
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
