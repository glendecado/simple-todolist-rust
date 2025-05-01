import React, { useState } from "react";

export default function AddTodoForm({ onAdd }) {
  const [title, setTitle] = useState("");
  const [isFocused, setIsFocused] = useState(false);

  const handleSubmit = (e) => {
    e.preventDefault();
    if (!title.trim()) return;
    onAdd(title);
    setTitle("");
  };

  return (
    <form onSubmit={handleSubmit} className="w-full max-w-md mx-auto">
      <div className="relative flex items-center mb-4">
        <input
          type="text"
          placeholder="Add a new task..."
          value={title}
          onChange={(e) => setTitle(e.target.value)}
          onFocus={() => setIsFocused(true)}
          onBlur={() => setIsFocused(false)}
          className={`w-full px-5 py-4 rounded-lg border-2 bg-white text-gray-800 font-medium text-lg focus:outline-none transition-all duration-200 ${
            isFocused ? "border-indigo-500 shadow-md" : "border-gray-200"
          }`}
        />
        <button
          type="submit"
          className={`absolute right-2 px-4 py-2 rounded-md font-semibold text-white transition-all duration-200 ${
            title.trim()
              ? "bg-indigo-600 hover:bg-indigo-700"
              : "bg-gray-400 cursor-not-allowed"
          }`}
          disabled={!title.trim()}
        >
          Add
        </button>
      </div>
    </form>
  );
}