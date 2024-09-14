import "./App.css";
import { ExpensePage } from "./pages/expenses-page/ExpensesPage";

function App() {
  // setGreetMsg(await invoke("greet", { name }));

  return (
    <div className="container">
      <h1>Tagged Expense Manager</h1>
      <ExpensePage />
    </div>
  );
}

export default App;
