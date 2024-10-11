import { createLazyFileRoute, Link } from "@tanstack/react-router";

export const Route = createLazyFileRoute("/notes//$id/edit")({
  component: Edit,
});

function Edit() {
  return (
    <div className="flex flex-col gap-2 p-2">
      <h3>Editing note {Route.useParams().id}</h3>
      <Link to="/notes">Retourner à la liste des notes</Link>
      <Link to={`/notes/${Route.useParams().id}`}>Retourner à la note</Link>
    </div>
  );
}
