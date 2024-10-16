/// <reference types="vite/client" />

import { Environment } from "./types/environment";

interface ImportMetaEnv {
  readonly APP_ENV?: Environment;
  readonly VITE_APP_ENV?: Environment;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
