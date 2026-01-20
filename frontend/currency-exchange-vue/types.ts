
export interface RatesResponse {
  date: string;
  base: string;
  rates: Record<string, number>;
}

export interface ConversionResult {
  from: string;
  amount: number;
  to: string;
  result: number;
  date: string;
}

export interface HistoryItem {
  id: string;
  from: string;
  to: string;
  amount: number;
  result: number;
  timestamp: number;
}
