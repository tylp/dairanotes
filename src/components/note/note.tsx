import React from "react";
import { PropsWithChildren } from "react";
import { noteStyles } from "./styles";
import { Color, Padding } from "./types";
import { Test } from "../../app/test";

export type NoteProps = PropsWithChildren & {
  color?: Color;
  padding?: Padding;
};

export function Note({ color = "red", padding = "md", children }: NoteProps) {
  return (
    <article className={noteStyles({ color, padding })}>
      <Test />
      {children}
    </article>
  );
}
