import { Paper } from "@/components/paper";
import { Note } from "../../types/note";

export type NotePreviewProps = {
  note: Note;
};

export function NotePreview({ note }: NotePreviewProps) {
  return <Paper>{note.title}</Paper>;
}
