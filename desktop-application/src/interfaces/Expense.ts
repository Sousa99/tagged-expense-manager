export interface Expense {
  id: number;
  created_at: number;
  title: string;
  description?: string;
  timestamp: string;
  value_decimal: number;
}
