import "./App.css";
import { ExpensePage } from "./pages/expenses-page/ExpensesPage";

function App() {
  // setGreetMsg(await invoke("greet", { name }));

  return (
    <div className="container">
      <ExpensePage />
    </div>
  );
}

export default App;
