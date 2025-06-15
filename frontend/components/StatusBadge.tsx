"use client"

import { useState, useEffect } from "react";

export default function StatusBadge() {
  const [status, setStatus] = useState("Loading...");

  useEffect(() => {
    fetch("/api/health")
      .then((res) => {
        if (res.ok) {
          return res.json();
        }
        throw new Error("Failed to fetch status");
      })
      .then((data) => setStatus(data.status))
      .catch((err) => {
        console.error(err);
        setStatus("offline");
      });
  }, []);

  return (
    <div className="flex items-center gap-2 fixed top-0 right-0 p-2">
      <div className={`w-2 h-2 rounded-full ${status === "online" ? "bg-green-500" : "bg-red-500"}`}></div>
      <p className="text-sm">{status}</p>
    </div>
  );
}
