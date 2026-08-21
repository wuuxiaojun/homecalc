export interface House {
  purchase_price: number;
  annual_property_tax_rate: number; // in percentage, e.g. 1.25
  annual_insurance: number;
  monthly_hoa: number;
}

export interface Cash {
  amount: number;
  rate: number; // annual yield in %, e.g. 4.0
}

export interface Mortgage {
  amount: number;
  rate: number; // annual interest rate in %, e.g. 6.5
  term: number; // term in years, e.g. 30
}

export interface Loc {
  amount: number;
  rate: number; // annual interest rate in %, e.g. 7.0
}

export type Tool =
  | { Cash: Cash; Mortgage?: never; Loc?: never }
  | { Mortgage: Mortgage; Cash?: never; Loc?: never }
  | { Loc: Loc; Cash?: never; Mortgage?: never };

export interface Purchase {
  name: string;
  house: House;
  tools: Tool[];
  mortgage_repay: Record<number, number>; // month -> amount
  loc_repay: Record<number, number>;      // month -> amount
}

export interface CashStatement {
  cash_now: number;
  cash_interest: number;
}

export interface MortgageStatement {
  monthly_payment: number;
  principal_paid: number;
  interest_paid: number;
  extra_payment: number;
  remaining_balance: number;
}

export interface LocStatement {
  monthly_payment: number;
  extra_payment: number;
  remaining_balance: number;
}

export interface HouseStatement {
  monthly_property_tax: number;
  monthly_insurance: number;
  monthly_hoa: number;
}

export interface MonthlyStatementRow {
  month: number;
  cash: CashStatement | null;
  mortgage: MortgageStatement | null;
  loc: LocStatement | null;
  house: HouseStatement;
  total_debt_paid: number;
  total_extra_payment: number;
  total_holding_cost: number;
  total_paid: number;
  total_remaining_balance: number;
}

export interface YearlyStatementRow {
  year: number;
  annual_cash_interest: number;
  annual_interest_paid: number;
  annual_debt_paid: number;
  annual_tax_savings: number;
  annual_extra_payment: number;
  annual_holding_cost: number;
  annual_paid: number;
  ending_remaining_balance: number;
}

export interface TotalStatement {
  total_cash_interest: number;
  total_holding_cost: number;
  total_interest_paid: number;
  total_tax_savings: number;
  total_paid: number;
}

export interface Scenario {
  purchase: Purchase;
  monthly_statement: MonthlyStatementRow[];
  yearly_statement: YearlyStatementRow[];
  total_statement: TotalStatement;
}

export interface ScenarioAnalysis {
  waste_ratio: number;
  tax_savings_ratio: number;
  payoff_month: number;
  effective_monthly_cost: number;
}

export interface ScenarioComparison {
  baseline_payoff_month: number;
  alternative_payoff_month: number;
  months_saved: number;
  baseline_extra_payment: number;
  alternative_extra_payment: number;
  delta_extra_payment: number;
  baseline_interest_paid: number;
  alternative_interest_paid: number;
  delta_interest_paid: number;
  baseline_cash_interest: number;
  alternative_cash_interest: number;
  delta_cash_interest: number;
  baseline_tax_savings: number;
  alternative_tax_savings: number;
  delta_tax_savings: number;
  baseline_gross_paid: number;
  alternative_gross_paid: number;
  delta_gross_paid: number;
  baseline_pv: number;
  alternative_pv: number;
  delta_pv: number;
  irr: number | null;
}

export type SlotId = 1 | 2 | 3;

export interface ScenarioSlot {
  id: SlotId;
  name: string;
  purchase: Purchase | null;
  scenario: Scenario | null;
  analysis: ScenarioAnalysis | null;
  error?: string | null;
}
