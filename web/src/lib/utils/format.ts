/**
 * Safe formatting utilities with strict non-finite number guards.
 * If any input is NaN, Infinity, -Infinity, null, or undefined,
 * it returns the specified fallback string (defaulting to "N/A").
 */

export function isValidNumber(val: any): val is number {
  return typeof val === 'number' && isFinite(val) && !isNaN(val);
}

export function formatCurrency(val: number | null | undefined, fallback = 'N/A'): string {
  if (!isValidNumber(val)) return fallback;
  return '$' + Math.round(val).toLocaleString();
}

export function formatCurrencyExact(val: number | null | undefined, fallback = 'N/A'): string {
  if (!isValidNumber(val)) return fallback;
  return '$' + val.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 });
}

export function formatNumber(val: number | null | undefined, fallback = 'N/A'): string {
  if (!isValidNumber(val)) return fallback;
  return Math.round(val).toLocaleString();
}

export function formatPercent(val: number | null | undefined, decimals = 1, fallback = 'N/A'): string {
  if (!isValidNumber(val)) return fallback;
  return val.toFixed(decimals) + '%';
}

export function formatYears(months: number | null | undefined, fallback = 'N/A'): string {
  if (!isValidNumber(months)) return fallback;
  return (months / 12).toFixed(1);
}
