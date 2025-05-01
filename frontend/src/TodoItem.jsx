import React from "react";

export default function TodoItem({ todo, onUpdate, onDelete }) {
  const toggleDone = () => {
    onUpdate(todo.id, { done: !todo.done });
  };

  return (
    <li className="group relative">
      <div className={`flex items-center gap-4 p-5 rounded-xl bg-white shadow-sm transition-all duration-300 hover:shadow-md hover:-translate-y-0.5 border border-gray-100 ${
        todo.done ? "opacity-90" : ""
      }`}>
        {/* Animated Checkbox */}
        <button
          onClick={toggleDone}
          className={`flex-shrink-0 w-6 h-6 rounded-lg border-2 flex items-center justify-center transition-all duration-300 ${
            todo.done 
              ? "bg-indigo-500 border-indigo-500 shadow-inner" 
              : "border-gray-300 hover:border-indigo-300 group-hover:shadow-sm"
          }`}
          aria-label={todo.done ? 'Mark as incomplete' : 'Mark as complete'}
        >
          {todo.done && (
            <svg 
              xmlns="http://www.w3.org/2000/svg" 
              className="w-4 h-4"
              viewBox="0 0 24 24" 
              fill="none" 
              stroke="white" 
              strokeWidth="3" 
              strokeLinecap="round"
            >
              <path d="M20 6L9 17l-5-5" />
            </svg>
          )}
        </button>

        {/* Todo Text with Smooth Transition */}
        <span className={`flex-1 text-lg font-medium transition-all duration-300 ${
          todo.done 
            ? "text-gray-400 line-through decoration-indigo-400" 
            : "text-gray-700"
        }`}>
          {todo.title}
        </span>

        {/* Delete Button with Micro-interaction */}
        <button
          onClick={() => onDelete(todo.id)}
          className="flex-shrink-0 w-8 h-8 rounded-full flex items-center justify-center text-gray-400 hover:text-white hover:bg-red-500 transition-all duration-300 transform hover:scale-110 group-hover:opacity-100 focus:outline-none focus:ring-2 focus:ring-red-200"
          aria-label="Delete todo"
        >
          <svg 
            xmlns="http://www.w3.org/2000/svg" 
            className="w-5 h-5" 
            viewBox="0 0 24 24" 
            fill="none" 
            stroke="currentColor" 
            strokeWidth="2" 
            strokeLinecap="round" 
            strokeLinejoin="round"
          >
            <line x1="18" y1="6" x2="6" y2="18"></line>
            <line x1="6" y1="6" x2="18" y2="18"></line>
          </svg>
        </button>
      </div>

      {/* Subtle Completion Indicator */}
      {todo.done && (
        <div className="absolute inset-0 rounded-xl pointer-events-none border-2 border-indigo-100"></div>
      )}
    </li>
  );
}