import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { ExpenseLine } from "../../components/expense-line/ExpenseLine";

export interface ExpensesPageProps {}

export const ExpensePage = (_props: ExpensesPageProps): JSX.Element => {

  const [ expenses, setExpenses ] = useState<Expense[]|null>(null);
  useEffect(() => {
    invoke<Expense[]|null>("get_expenses")
      .then((value) => setExpenses(value));
  }, []);

  if (expenses === null) {
    return (
      <p> No Expenses Registered</p>
    )
  }

  return (
    <div id='expense-list'>
      { expenses.map((expense) => (
        <ExpenseLine
          key={expense.id}
          expense_title={expense.title}
          expense_time={expense.timestamp}
          expense_description={expense.description}
          expense_value={expense.value_decimal}
        />
      ))}
    </div>
  )
}