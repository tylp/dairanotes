import { useNoteQuery } from "@/features/notes/api/use-note-query";
import { useNotesQuery } from "@/features/notes/api/use-notes-query";

export function App() {
  const { data: list } = useNotesQuery();

  const { data } = useNoteQuery({ id: 1 });

  console.log({ list, data });

  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>

      <p>You have {list?.length} notes</p>
      <p>{data?.title}</p>
    </div>
  );
}
