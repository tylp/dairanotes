import { useQuery } from "@tanstack/react-query";
import { Note } from "../types/note";
import { invoke } from "@tauri-apps/api/core";

export type UseNotesQueryReturn = Note[];

const queryFn = (): Promise<UseNotesQueryReturn> => {
  return invoke("notes_index");
};

export const useNotesQuery = () => {
  return useQuery({
    queryKey: ["notes"],
    queryFn,
  });
};
