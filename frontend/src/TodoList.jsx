import React from "react";
import TodoItem from "./TodoItem";

export default function TodoList({ todos, onUpdate, onDelete }) {
  return (
    <ul className="flex flex-col gap-4">
      {todos.map((todo) => (
        <TodoItem
          key={todo.id}
          todo={todo}
          onUpdate={onUpdate}
          onDelete={onDelete}
        />
      ))}
    </ul>
  );
}
