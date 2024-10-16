import { createLazyFileRoute, Link } from "@tanstack/react-router";

export const Route = createLazyFileRoute("/notes/create")({
  component: Create,
});

function Create() {
  return (
    <div className="p-2">
      <h3>Page de création d'une note</h3>

      <Link to="/notes">Retourner à la liste des notes</Link>
    </div>
  );
}
