import { useQuery } from "@tanstack/react-query";
import { Note } from "../types/note";
import { invoke } from "@tauri-apps/api/core";

export type UseNoteQueryReturn = Note;

const queryFn = (id: number): Promise<UseNoteQueryReturn> => {
  return invoke("notes_show", { id });
};

export const useNoteQuery = ({ id }: { id: Note["id"] }) => {
  return useQuery({
    enabled: !!id,
    queryKey: ["notes", id],
    queryFn: () => queryFn(id),
  });
};
