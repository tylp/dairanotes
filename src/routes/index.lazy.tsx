import { createLazyFileRoute } from "@tanstack/react-router";
import { Index } from "./notes/index.lazy";

export const Route = createLazyFileRoute("/")({
  component: Index,
});
