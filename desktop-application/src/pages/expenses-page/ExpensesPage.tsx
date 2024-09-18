import styles from "./ExpensesPage.module.scss";

import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { Expense } from "../../interfaces/Expense";
import { ExpenseLine } from "../../components/expense-line/ExpenseLine";

export const ExpensePage = (): JSX.Element => {
  const [expenses, setExpenses] = useState<Expense[] | null>(null);
  useEffect(() => {
    invoke<Expense[] | null>("get_expenses").then((value) =>
      setExpenses(value),
    );
  }, []);

  if (expenses === null) {
    return <p> No Expenses Registered</p>;
  }

  return (
    <div className={styles["expense-list"]}>
      {expenses.map((expense) => (
        <ExpenseLine
          key={expense.id}
          expense_title={expense.title}
          expense_time={expense.timestamp}
          expense_description={expense.description}
          expense_value={expense.value_decimal}
        />
      ))}
    </div>
  );
};
