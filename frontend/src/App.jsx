import React, { useEffect, useState } from "react";
import TodoList from "./TodoList";
import AddTodoForm from "./AddTodoForm";

export default function App() {
  const [todos, setTodos] = useState([]);

  const fetchTodos = async () => {
    const res = await fetch("http://localhost:8080/todos");
    const data = await res.json();
    setTodos(data);
  };

  useEffect(() => {
    fetchTodos();
  }, []);

  const addTodo = async (title) => {
    const res = await fetch(`http://localhost:8080/todos?title=${encodeURIComponent(title)}`, {
      method: "POST",
    });
    const newTodo = await res.json();
    setTodos((prev) => [...prev, newTodo]);
  };

  const updateTodo = async (id, updatedFields) => {
    await fetch(`http://localhost:8080/todos/${id}`, {
      method: "PUT",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(updatedFields),
    });
    fetchTodos();
  };

  const deleteTodo = async (id) => {
    await fetch(`http://localhost:8080/todos/${id}`, { method: "DELETE" });
    setTodos((prev) => prev.filter((todo) => todo.id !== id));
  };

  const centerStyle = {
    display: 'flex',
    justifyContent: 'center',
    alignItems: 'center',
    height: '100vh', // full viewport height
    width: '100vw',

  };


  return (
    <div className="min-h-screen bg-gradient-to-br from-indigo-50 to-white flex items-center justify-center p-4">
      <div className="w-full max-w-md">
        <div className="bg-white rounded-2xl shadow-xl overflow-hidden p-1">
          <div className="bg-gradient-to-r from-indigo-500 to-purple-600 p-6 text-center">
            <h1 className="text-3xl font-bold text-white flex items-center justify-center gap-3">
              <span className="inline-block animate-bounce">📝</span>
              <span>Todo List</span>
            </h1>
          </div>

          <div className="p-6 space-y-6">
            <AddTodoForm onAdd={addTodo} />
            <div className="space-y-3 max-h-[400px] overflow-y-auto pr-2 custom-scrollbar">
              <TodoList
                todos={todos}
                onUpdate={updateTodo}
                onDelete={deleteTodo}
              />
            </div>

            {todos.length > 0 && (
              <div className="text-center text-sm text-gray-500 pt-2 border-t border-gray-100">
                {todos.filter(t => t.done).length} of {todos.length} tasks completed
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
