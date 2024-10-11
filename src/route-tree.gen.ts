/* prettier-ignore-start */

/* eslint-disable */

// @ts-nocheck

// noinspection JSUnusedGlobalSymbols

// This file is auto-generated by TanStack Router

import { createFileRoute } from "@tanstack/react-router"

// Import Routes

import { Route as rootRoute } from "./routes/__root"

// Create Virtual Routes

const IndexLazyImport = createFileRoute("/")()
const NotesIndexLazyImport = createFileRoute("/notes/")()
const NotesCreateLazyImport = createFileRoute("/notes/create")()
const NotesIdLazyImport = createFileRoute("/notes/$id")()
const NotesIdEditLazyImport = createFileRoute("/notes//$id/edit")()

// Create/Update Routes

const IndexLazyRoute = IndexLazyImport.update({
  path: "/",
  getParentRoute: () => rootRoute,
} as any).lazy(() => import("./routes/index.lazy").then((d) => d.Route))

const NotesIndexLazyRoute = NotesIndexLazyImport.update({
  path: "/notes/",
  getParentRoute: () => rootRoute,
} as any).lazy(() => import("./routes/notes/index.lazy").then((d) => d.Route))

const NotesCreateLazyRoute = NotesCreateLazyImport.update({
  path: "/notes/create",
  getParentRoute: () => rootRoute,
} as any).lazy(() => import("./routes/notes/create.lazy").then((d) => d.Route))

const NotesIdLazyRoute = NotesIdLazyImport.update({
  path: "/notes/$id",
  getParentRoute: () => rootRoute,
} as any).lazy(() => import("./routes/notes/$id.lazy").then((d) => d.Route))

const NotesIdEditLazyRoute = NotesIdEditLazyImport.update({
  path: "/notes/$id/edit",
  getParentRoute: () => rootRoute,
} as any).lazy(() =>
  import("./routes/notes/_.$id.edit.lazy").then((d) => d.Route),
)

// Populate the FileRoutesByPath interface

declare module "@tanstack/react-router" {
  interface FileRoutesByPath {
    "/": {
      id: "/"
      path: "/"
      fullPath: "/"
      preLoaderRoute: typeof IndexLazyImport
      parentRoute: typeof rootRoute
    }
    "/notes/$id": {
      id: "/notes/$id"
      path: "/notes/$id"
      fullPath: "/notes/$id"
      preLoaderRoute: typeof NotesIdLazyImport
      parentRoute: typeof rootRoute
    }
    "/notes/create": {
      id: "/notes/create"
      path: "/notes/create"
      fullPath: "/notes/create"
      preLoaderRoute: typeof NotesCreateLazyImport
      parentRoute: typeof rootRoute
    }
    "/notes/": {
      id: "/notes/"
      path: "/notes"
      fullPath: "/notes"
      preLoaderRoute: typeof NotesIndexLazyImport
      parentRoute: typeof rootRoute
    }
    "/notes//$id/edit": {
      id: "/notes//$id/edit"
      path: "/notes/$id/edit"
      fullPath: "/notes/$id/edit"
      preLoaderRoute: typeof NotesIdEditLazyImport
      parentRoute: typeof rootRoute
    }
  }
}

// Create and export the route tree

export interface FileRoutesByFullPath {
  "/": typeof IndexLazyRoute
  "/notes/$id": typeof NotesIdLazyRoute
  "/notes/create": typeof NotesCreateLazyRoute
  "/notes": typeof NotesIndexLazyRoute
  "/notes/$id/edit": typeof NotesIdEditLazyRoute
}

export interface FileRoutesByTo {
  "/": typeof IndexLazyRoute
  "/notes/$id": typeof NotesIdLazyRoute
  "/notes/create": typeof NotesCreateLazyRoute
  "/notes": typeof NotesIndexLazyRoute
  "/notes/$id/edit": typeof NotesIdEditLazyRoute
}

export interface FileRoutesById {
  __root__: typeof rootRoute
  "/": typeof IndexLazyRoute
  "/notes/$id": typeof NotesIdLazyRoute
  "/notes/create": typeof NotesCreateLazyRoute
  "/notes/": typeof NotesIndexLazyRoute
  "/notes//$id/edit": typeof NotesIdEditLazyRoute
}

export interface FileRouteTypes {
  fileRoutesByFullPath: FileRoutesByFullPath
  fullPaths: "/" | "/notes/$id" | "/notes/create" | "/notes" | "/notes/$id/edit"
  fileRoutesByTo: FileRoutesByTo
  to: "/" | "/notes/$id" | "/notes/create" | "/notes" | "/notes/$id/edit"
  id:
    | "__root__"
    | "/"
    | "/notes/$id"
    | "/notes/create"
    | "/notes/"
    | "/notes//$id/edit"
  fileRoutesById: FileRoutesById
}

export interface RootRouteChildren {
  IndexLazyRoute: typeof IndexLazyRoute
  NotesIdLazyRoute: typeof NotesIdLazyRoute
  NotesCreateLazyRoute: typeof NotesCreateLazyRoute
  NotesIndexLazyRoute: typeof NotesIndexLazyRoute
  NotesIdEditLazyRoute: typeof NotesIdEditLazyRoute
}

const rootRouteChildren: RootRouteChildren = {
  IndexLazyRoute: IndexLazyRoute,
  NotesIdLazyRoute: NotesIdLazyRoute,
  NotesCreateLazyRoute: NotesCreateLazyRoute,
  NotesIndexLazyRoute: NotesIndexLazyRoute,
  NotesIdEditLazyRoute: NotesIdEditLazyRoute,
}

export const routeTree = rootRoute
  ._addFileChildren(rootRouteChildren)
  ._addFileTypes<FileRouteTypes>()

/* prettier-ignore-end */

/* ROUTE_MANIFEST_START
{
  "routes": {
    "__root__": {
      "filePath": "__root.tsx",
      "children": [
        "/",
        "/notes/$id",
        "/notes/create",
        "/notes/",
        "/notes//$id/edit"
      ]
    },
    "/": {
      "filePath": "index.lazy.tsx"
    },
    "/notes/$id": {
      "filePath": "notes/$id.lazy.tsx"
    },
    "/notes/create": {
      "filePath": "notes/create.lazy.tsx"
    },
    "/notes/": {
      "filePath": "notes/index.lazy.tsx"
    },
    "/notes//$id/edit": {
      "filePath": "notes/_.$id.edit.lazy.tsx"
    }
  }
}
ROUTE_MANIFEST_END */
