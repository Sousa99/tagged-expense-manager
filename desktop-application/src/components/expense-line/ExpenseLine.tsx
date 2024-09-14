import styles from './ExpenseLine.module.scss';

export interface ExpenseLineProps {
  /**
   * Title associated with the expense
   */
  expense_title: string,
  /**
   * Timestamp associated with the expense
   */
  expense_time: string,
  /**
   * Description associated with the expense
   */
  expense_description: string,
  /**
   * Value associated with the expense, expressed in cents
   */
  expense_value: number,
}

/**
 * Component used to display an expense entry.
 * 
 * This is one of the most crucial components within the application.  
 * It is meant to be re-used within the application in various places and therefore to be highly adaptive.
 */
export const ExpenseLine = (props: ExpenseLineProps): JSX.Element => {

  return (
    <div className={styles['expense-line']} data-testid='expense-line-component'>
      <div className={styles['description']}>
        <div className={styles['title']}>
          <p>{props.expense_title}</p>
          <p>{props.expense_time}</p>
        </div>
        <p>{props.expense_description}</p>
      </div>
      <p>{props.expense_value}</p>
    </div>
  )
}