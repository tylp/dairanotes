import { useQuery } from "@tanstack/react-query";
import { Note } from "../types/note";
import { invoke } from "@tauri-apps/api/core";

const queryFn = (): Promise<{ data: Note[] }> => {
  return invoke("notes_index");
};

export const useNotesQuery = () => {
  return useQuery({
    queryKey: ["notes"],
    queryFn,
  });
};
