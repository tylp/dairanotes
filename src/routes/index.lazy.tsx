import { createLazyFileRoute } from "@tanstack/react-router";
import { Login } from "./auth/login.lazy";

export const Route = createLazyFileRoute("/")({
  component: Login,
});
