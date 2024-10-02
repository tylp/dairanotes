import { useQuery } from "@tanstack/react-query";
import { Note } from "../types/note";
import { invoke } from "@tauri-apps/api/core";

const queryFn = (id: number): Promise<Note> => {
  return invoke("notes_show", { id });
};

export const useNoteQuery = ({ id }: { id: Note["id"] }) => {
  return useQuery({
    enabled: !!id,
    queryKey: ["notes", id],
    queryFn: () => queryFn(id),
  });
};
