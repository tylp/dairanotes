/// <reference types="vite/client" />

import { Environment } from "./types/environment";

interface ImportMetaEnv {
  readonly APP_ENV?: Environment;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
