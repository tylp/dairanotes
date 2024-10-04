import { createServer, Model, Registry } from "miragejs";
import { createTauriResponse } from "./utils/create-tauri-route";
import { Environment } from "@/types/environment";
import { seeds, serializers } from "./configs";
import Schema from "miragejs/orm/schema";

type MakeServerParams = {
  environment?: Environment;
};

export function makeServer({
  environment = Environment.development,
}: MakeServerParams) {
  const server = createServer({
    environment,
    urlPrefix: "http://ipc.localhost",

    models: {
      note: Model,
    },

    seeds,

    routes() {
      this.post("/notes_index", (schema) => {
        return createTauriResponse(schema.notes.all());
      });

      this.post("/notes_show", (schema) => {
        return createTauriResponse(schema.notes.findBy({ id: 1 }));
      });
    },

    serializers,
  });

  return server;
}
