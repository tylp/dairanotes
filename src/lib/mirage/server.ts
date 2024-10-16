import { createServer, Model } from "miragejs";
import { createTauriResponse } from "./utils/create-tauri-route";
import { Environment } from "@/types/environment";
import { seeds, serializers } from "./configs";

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

      this.post("/notes_show", (schema, request) => {
        const { id } = JSON.parse(request.requestBody);

        return createTauriResponse(schema.notes.findBy({ id }));
      });
    },

    serializers,
  });

  return server;
}
