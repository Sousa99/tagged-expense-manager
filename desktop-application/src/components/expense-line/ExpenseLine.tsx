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

  const wholePrice = Math.floor(props.expense_value / 100)
    .toString();
  const partPrice = Math.floor(props.expense_value % 100)
    .toString()
    .padStart(2, '0');
  const priceFomatted = `${wholePrice}.${partPrice} €`

  return (
    <div className={styles['expense-line']} data-testid='expense-line-component'>
      <div className={styles['description']}>
        <div className={styles['header']}>
          <p id='title' className={styles['title']}>{props.expense_title}</p>
          <p id='timestamp' className={styles['timestamp']}>{props.expense_time}</p>
        </div>
        <p>{props.expense_description}</p>
      </div>
      <p className={styles['price']}>{priceFomatted}</p>
    </div>
  )
}