import { useNoteQuery } from "@/features/notes/api/use-note-query";
import { createLazyFileRoute, Link } from "@tanstack/react-router";

export const Route = createLazyFileRoute("/notes/$id")({
  component: Show,
});

function Show() {
  const id = Number(Route.useParams().id);

  const { data } = useNoteQuery({ id });

  if (!data) return;

  return (
    <div className="flex flex-col gap-2 p-2">
      <h3>{data.title}</h3>
      <Link to="/notes">Retourner à la liste des notes</Link>
      <Link to={`/notes/${id}/edit`}>Modifier cette note</Link>
    </div>
  );
}
