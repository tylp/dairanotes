import React from "react";
import ReactDOM from "react-dom/client";
import { App } from "./app";
import "./index.css";
import { QueryClientProvider } from "@tanstack/react-query";
import { queryClient } from "./lib/react-query";
import { makeServer } from "./lib/mirage/server";
import { Environment } from "./types/environment";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <QueryClientProvider client={queryClient}>
      <App />
    </QueryClientProvider>
  </React.StrictMode>,
);

makeServer({
  environment: import.meta.env.APP_ENV || Environment.development,
}).start();
