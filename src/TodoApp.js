import React, { useMemo, useState, useEffect } from "react";
import axios from "axios";

const TodoApp = () => {
  const [todos, setTodos] = useState([]);
  const [input, setInput] = useState("");
  const [showCompleted, setShowCompleted] = useState(true);
  const [statusMessage, setStatusMessage] = useState("");

  useEffect(() => {
    const fetchTodos = async () => {
      try {
        const response = await axios.get("http://localhost:3001/todos");
        setTodos(response.data.oppgaver);
      } catch (error) {
        console.error("Error fetching todos:", error);
      }
    };

    fetchTodos();
  }, []);

  const updateTodos = async (updatedTodos) => {
    try {
      await axios.put("http://localhost:3001/todos", {
        oppgaver: updatedTodos,
      });
      setTodos(updatedTodos);
    } catch (error) {
      console.error("Error updating todos:", error);
    }
  };

  const addTodo = async (e) => {
    e.preventDefault();
    if (input.trim() === "") return;
    const nextIndex =
      todos.sort((a, b) => b.index - a.index)[0]?.index + 1 || 0;

    const updatedTodos = [
      ...todos,
      { oppgave: input, fullført: false, index: nextIndex },
    ];
    await updateTodos(updatedTodos);
    setStatusMessage(`Oppgaven: ${input} ble lagt til`);
    setInput("");
  };

  const deleteTodo = async (index, oppgave) => {
    const updatedTodos = todos.filter((item) => index !== item.index);
    await updateTodos(updatedTodos);
    setStatusMessage(`Oppgaven: ${oppgave} ble slettet`);
  };

  const toggleComplete = (index) => {
    const updatedTodos = todos.reduce((acc, todo) => {
      if (todo.index === index) {
        todo.fullført = !todo.fullført;
      }
      acc.push(todo);
      return acc;
    }, []);
    setTodos(updatedTodos);
  };

  const filterdTodos = useMemo(() => {
    if (showCompleted) return todos;
    return todos ? todos.filter((todo) => !todo.fullført) : [];
  }, [todos, showCompleted]);

  return (
    <div>
      <h1>Huskeliste</h1>

      <div role="status" aria-live="polite" className="vh">
        {statusMessage}
      </div>
      <button
        className="toggle-btn"
        aria-pressed={showCompleted}
        onClick={() => setShowCompleted(!showCompleted)}
      >
        <span className="tick"></span>
        <span className="text"> Vis fullførte oppgaver</span>
      </button>
      <ul>
        {filterdTodos.map((todo) => {
          const itemId = `todo-${todo.index}`;
          return (
            <li key={todo.index}>
              <input
                className="vh"
                id={itemId}
                type="checkbox"
                checked={todo.fullført}
                onChange={() => toggleComplete(todo.index)}
              />
              <label htmlFor={itemId} className="text">
                <span className="tick"></span>
                <span className="text">{todo.oppgave}</span>
              </label>
              <button
                onClick={() => deleteTodo(todo.index, todo.oppgave)}
                aria-label={`slett ${todo.oppgave}`}
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
