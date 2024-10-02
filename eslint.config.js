import globals from "globals";
import pluginJs from "@eslint/js";
import tseslint from "typescript-eslint";
import pluginReact from "eslint-plugin-react";
import { includeIgnoreFile } from "@eslint/compat";
import importPlugin from "eslint-plugin-import";
import eslintPluginUnicorn from "eslint-plugin-unicorn";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const gitignorePath = path.resolve(__dirname, ".gitignore");

export default [
  {
    files: ["src/**/*.{js,mjs,cjs,ts,jsx,tsx}"],
  },
  { languageOptions: { globals: globals.browser } },
  pluginJs.configs.recommended,
  ...tseslint.configs.recommended,
  pluginReact.configs.flat.recommended,
  includeIgnoreFile(gitignorePath),
  {
    settings: {
      react: {
        version: "detect",
      },
    },
  },
  {
    ignores: [],
  },
  {
    plugins: {
      "plugin-import": importPlugin,
      unicorn: eslintPluginUnicorn,
    },
    rules: {
      "plugin-import/no-restricted-paths": [
        "error",
        {
          zones: [
            // disables cross-feature imports:
            // eg. src/features/active-instances should not import from src/features/active-instances, etc.
            {
              target: "./src/features/notes",
              from: "./src/features",
              except: ["./notes"],
            },
            {
              target: "./src/features/users",
              from: "./src/features",
              except: ["./users"],
            },
            // enforce unidirectional codebase:
            // e.g. src/app can import from src/features but not the other way around
            {
              target: "./src/features",
              from: "./src/app",
            },
            // e.g src/features and src/app can import from these shared modules but not the other way around
            {
              target: ["./src/components"],
              from: ["./src/features", "/src/app"],
            },
          ],
        },
      ],
      "unicorn/filename-case": [
        "error",
        {
          case: "kebabCase",
          ignore: [/^\[([A-Za-z]+)]\.(js|ts|tsx|jsx)$/],
        },
      ],
    },
  },
];
