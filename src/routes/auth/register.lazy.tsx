import { createLazyFileRoute, Link } from "@tanstack/react-router";

export const Route = createLazyFileRoute("/auth/register")({
  component: Register,
});

function Register() {
  return (
    <div className="flex flex-col gap-2 p-2">
      <h3>Page d'inscription</h3>

      <Link to="/auth/login">Déjà un compte?</Link>
    </div>
  );
}
