import { useNoteQuery } from "@/features/notes/api/use-note-query";

export function App() {
  const { data } = useNoteQuery({ id: 1 });

  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>

      <p>{data?.title}</p>
    </div>
  );
}
