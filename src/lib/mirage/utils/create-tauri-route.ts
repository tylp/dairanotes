import { Response } from "miragejs";

type BodyParamType = ConstructorParameters<typeof Response>[2];

export function createTauriResponse(body: BodyParamType) {
  return new Response(
    200,
    {
      "Access-Control-Allow-Origin": "*",
      "Access-Control-Expose-Headers": "Tauri-Response",
      "Content-Type": "application/json",
      "Tauri-Response": "ok",
    },
    body,
  );
}
