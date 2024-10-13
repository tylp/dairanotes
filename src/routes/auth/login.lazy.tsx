import { createLazyFileRoute, Link } from "@tanstack/react-router";

export const Route = createLazyFileRoute("/auth/login")({
  component: Login,
});

export function Login() {
  return (
    <div className="flex flex-col gap-2 p-2">
      <h3>Page de connexion</h3>

      <Link to="/notes">Connexion</Link>
      <Link to="/auth/register">Pas encore de compte?</Link>
    </div>
  );
}
