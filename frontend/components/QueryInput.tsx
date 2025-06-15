"use client";

import { FormEvent, useState } from "react";

interface QueryInputProps {
  onSubmit: (query: string) => void;
  loading: boolean;
}

export default function QueryInput({ onSubmit, loading }: QueryInputProps) {
  const [query, setQuery] = useState("");

  function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    onSubmit(query);
  }

  return (
    <form onSubmit={handleSubmit} className="flex flex-col w-full">
      <div className="flex items-center w-full gap-4 border border-gray-500 rounded-md p-2">
        <input
          type="text"
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          placeholder="Enter your query"
          className="bg-transparent outline-none w-full"
        />
        <button
          type="submit"
          className={`border border-gray-500 rounded-md p-2 shrink-0 ${loading ? "opacity-50 cursor-not-allowed" : ""}`}
          disabled={loading}
        >
          {loading ? "Searching..." : "Search"}
        </button>
      </div>
      <span
        className={`text-sm self-end pt-1 ${query.length < 10 ? "text-orange-600" : "text-gray-500"}`}
      >
        {query.length} characters
      </span>
    </form>
  );
}
