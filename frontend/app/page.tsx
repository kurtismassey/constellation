"use client";

import { useState } from "react";
import QueryInput from "@/components/QueryInput";

interface QueryResponse {
  status: string;
  query: string;
  response: string;
}

export default function Home() {
  const [response, setResponse] = useState<QueryResponse | null>(null);
  const [loading, setLoading] = useState(false);

  async function handleSubmit(query: string) {
    if (!query || loading) return;

    setLoading(true);
    setResponse(null);

    try {
      const res = await fetch("/api/query", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ query }),
      });

      if (!res.ok) {
        const errorText = await res.text();
        throw new Error(
          `Failed to submit query: ${res.statusText} - ${errorText}`,
        );
      }

      const data = await res.json();
      setResponse(data);
    } catch (error) {
      console.error("Error submitting query:", error);
      setResponse({
        status: "error",
        query,
        response:
          error instanceof Error ? error.message : "An unknown error occurred",
      });
    } finally {
      setLoading(false);
    }
  }

  return (
    <div className="flex flex-col items-center justify-center min-h-screen py-10">
      <h1 className="text-4xl font-bold">Constellation</h1>
      <p className="text-lg">web research</p>
      <div className="w-full max-w-3xl my-10 px-4">
        <QueryInput onSubmit={handleSubmit} loading={loading} />
      </div>
      {response && (
        <span className="whitespace-pre-wrap max-w-3xl w-full px-4">
          {response.response}
        </span>
      )}
    </div>
  );
}
