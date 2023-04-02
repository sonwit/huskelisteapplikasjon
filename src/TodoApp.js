import React, { useMemo, useState } from "react";

const TodoApp = () => {
  const [todos, setTodos] = useState([]);
  const [input, setInput] = useState("");
  const [showCompleted, setShowCompleted] = useState(false);
  const [statusMessage, setStatusMessage] = useState("");

  const addTodo = (e) => {
    e.preventDefault();
    if (input.trim() === "") return;
    setTodos([...todos, { text: input, completed: false }]);
    setStatusMessage(`Oppgaven: ${input} ble lagt til`);
    setInput("");
  };

  const deleteTodo = (index) => {
    setStatusMessage(
      `Oppgaven: ${todos[index]?.text || "ukjent"} ble lagt slettet`
    );
    setTodos(todos.filter((_, i) => i !== index));
  };

  const toggleComplete = (index) => {
    const updatedTodos = [...todos];
    updatedTodos[index].completed = !updatedTodos[index].completed;
    setTodos(updatedTodos);
  };

  const filterdTodos = useMemo(() => {
    if (showCompleted) return todos;
    return todos.filter((todo) => !todo.completed);
  }, [todos, showCompleted]);

  return (
    <div>
      <h1>Huskeliste</h1>

      <div role="status" aria-live="polite" className="vh">
        {/* <!-- add content to hear it spoken --> */}
        {statusMessage}
      </div>

      <ul>
        {filterdTodos.map((todo, index) => {
          const itemId = `todo-${index}`;
          return (
            <li key={index}>
              <input
                className="vh"
                id={itemId}
                type="checkbox"
                checked={todo.completed}
                onChange={() => toggleComplete(index)}
              />
              <label htmlFor={itemId} className="text">
                <span class="tick"></span>
                <span class="text">{todo.text}</span>
              </label>
              <button
                onClick={() => deleteTodo(index)}
                aria-label={`slett ${todo.text}`}
              >
                &times;
              </button>
            </li>
          );
        })}
      </ul>
      {filterdTodos.length === 0 && (
        <div className="empty-state">
          <p>
            Enten har du allerede gjort alt du skal, eller så er det fortsatt
            ting å legge til på listen din. Legg til din første oppgave &#x2193;
          </p>
        </div>
      )}
      <button
        className="toggle-btn"
        aria-pressed={showCompleted}
        onClick={() => setShowCompleted(!showCompleted)}
      >
         <span class="tick"></span>
        <span class="text"> Vis fullførte oppgaver</span>

      </button>
      <form onSubmit={addTodo}>
        <label className="vh" htmlFor="addTodoName">
          Legg til ny oppgave
        </label>
        <input
          id="addTodoName"
          type="text"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          placeholder="Legg til ny oppgave"
        />

        <button type="submit">Legg til</button>
      </form>
    </div>
  );
};

export default TodoApp;
