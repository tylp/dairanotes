import { Server } from "miragejs";
import { NoteSeeds } from "../seeds/note-seeds";

type Data = Record<string, unknown>;

export function seeds(server: Server) {
  createList(server, "note", NoteSeeds);
}

function createList(server: Server, name: string, data: Data[]) {
  data.forEach((item) => {
    server.create(name, item);
  });
}
