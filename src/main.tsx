import React from "react";
import ReactDOM from "react-dom/client";
import "./index.css";
import { QueryClientProvider } from "@tanstack/react-query";
import { queryClient } from "./lib/react-query";
import { makeServer } from "./lib/mirage/server";
import { Environment } from "./types/environment";
import { RouterProvider, createRouter } from "@tanstack/react-router";
import { routeTree } from "./route-tree.gen";

const router = createRouter({ routeTree });

declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <QueryClientProvider client={queryClient}>
      <RouterProvider router={router} />
    </QueryClientProvider>
  </React.StrictMode>,
);

if (import.meta.env.VITE_APP_ENV === Environment.development)
  makeServer({
    environment: import.meta.env.VITE_APP_ENV || Environment.development,
  }).start();
