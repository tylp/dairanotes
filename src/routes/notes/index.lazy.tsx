import { useNotesQuery } from "@/features/notes/api/use-notes-query";
import { NotePreview } from "@/features/notes/components/note-preview";
import { createLazyFileRoute, Link } from "@tanstack/react-router";

export const Route = createLazyFileRoute("/notes/")({
  component: Index,
});

export function Index() {
  const { data: notes } = useNotesQuery();

  if (!notes) return;

  return (
    <div className="flex flex-col gap-2 p-2">
      <h3>Mes notes</h3>
      <Link to="/notes/create">Créer une nouvelle note</Link>

      <section className="flex flex-col gap-2">
        {notes.map((note) => (
          <Link key={note.id} to={`/notes/${note.id}`}>
            <NotePreview note={note} />
          </Link>
        ))}
      </section>
    </div>
  );
}
