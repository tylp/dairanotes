import { createLazyFileRoute, Link } from "@tanstack/react-router";

export const Route = createLazyFileRoute("/")({
  component: Index,
});

function Index() {
  return (
    <div className="p-2">
      <h3>Bienvenue sur le site de la noterie Flibustier!</h3>

      <Link to="/notes">Voir les notes</Link>
    </div>
  );
}
