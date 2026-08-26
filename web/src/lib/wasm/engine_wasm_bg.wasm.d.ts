/* tslint:disable */
/* eslint-disable */
export const memory: WebAssembly.Memory;
export const wasm_analyze_scenario: (a: any) => [number, number, number];
export const wasm_analyze_scenario_from_json: (a: number, b: number) => [number, number, number, number];
export const wasm_calculate_scenario_pv_from_json: (a: number, b: number) => [number, number, number];
export const wasm_compare_scenarios: (a: any, b: any) => [number, number, number];
export const wasm_compare_scenarios_from_json: (a: number, b: number, c: number, d: number) => [number, number, number, number];
export const wasm_create_scenario: (a: any) => [number, number, number];
export const wasm_create_scenario_from_json: (a: number, b: number) => [number, number, number, number];
export const wasm_default_starting_cash: () => number;
export const wasm_engine_version: () => [number, number];
export const wasm_solve_irr: (a: number, b: number) => [number, number, number, number];
export const __wbindgen_malloc: (a: number, b: number) => number;
export const __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
export const __wbindgen_exn_store: (a: number) => void;
export const __externref_table_alloc: () => number;
export const __wbindgen_externrefs: WebAssembly.Table;
export const __externref_table_dealloc: (a: number) => void;
export const __wbindgen_free: (a: number, b: number, c: number) => void;
export const __wbindgen_start: () => void;
