import React, { useMemo, useState, useEffect } from "react";

const TodoApp = () => {
  const [todos, setTodos] = useState([]);
  const [input, setInput] = useState("");
  const [showCompleted, setShowCompleted] = useState(true);
  const [statusMessage, setStatusMessage] = useState("");

  const fetchTodos = async () => {
    try {
      const req = await fetch("http://localhost:3000/api/todos");
      try {
        const res = await req.json();
        setTodos(res);
        console.log("🚀 ~ file: TodoApp.js:20 ~ fetchTodos ~ res.data:", res);
      } catch (error) {
        console.error("Error fetching todos:", error);
      }
    } catch (error) {
      console.error("Error fetching todos:", error);
    }
  };

  useEffect(() => {
    fetchTodos();
  }, []);
 

  const addTodo = async (e) => {
    e.preventDefault();
    if (input.trim() === "") return;
    try {
      await fetch("http://localhost:3000/api/todo", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ title: input }),
      });
      setInput("");
      fetchTodos();
      setStatusMessage(`Oppgaven: ${input} ble lagt til`);
    } catch (error) {
      console.error("Error updating todos:", error);
    }
  };

  const deleteTodo = async (oppgaveId) => {
    await fetch("http://localhost:3000/api/todo", {
      method: "DELETE",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ id: oppgaveId }),
    });
    fetchTodos();
    setStatusMessage(`Oppgaven: ${oppgaveId} ble slettet`);
  };

  const toggleComplete = async (oppgave) => {
    try {
      await fetch("http://localhost:3000/api/todo", {
        method: "PUT",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ ...oppgave, completed: !oppgave.completed }),
      });
      fetchTodos();
      setStatusMessage(`Oppgaven: ${oppgave.id} ble oppdatert`);
    } catch (error) {
      console.error("Error updating todos:", error);
    }
  };

  const filterdTodos = useMemo(() => {
    if (showCompleted) return todos;
    return todos ? todos.filter((todo) => !todo.completed) : [];
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
          const itemId = `todo-${todo.id}`;
          return (
            <li key={todo.id}>
              <input
                className="vh"
                id={itemId}
                type="checkbox"
                checked={todo.completed}
                onChange={() => toggleComplete(todo)}
              />
              <label htmlFor={itemId} className="text">
                <span className="tick"></span>
                <span className="text">{todo.title}</span>
              </label>
              <button
                onClick={() => deleteTodo(todo.id, todo.title)}
                aria-label={`slett ${todo.title}`}
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
